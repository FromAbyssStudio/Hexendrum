use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::fs;
use tokio::process::Command;
use tracing::{debug, warn};
use utoipa::ToSchema;

use super::{Library, Track};
use crate::utils::ensure_directory;

const LAST_FM_IMAGE_PRIORITY: [&str; 5] = ["mega", "extralarge", "large", "medium", "small"];
const LAST_FM_ENDPOINT: &str = "https://ws.audioscrobbler.com/2.0/";

#[derive(Debug, Clone)]
struct AlbumAggregate {
    id: String,
    title: String,
    primary_artist: Option<String>,
    artists: HashSet<String>,
    track_count: usize,
    sample_track: Option<Track>,
}

/// Summary data for an album aggregated from the library
#[derive(Debug, Clone)]
pub struct AlbumSummary {
    pub id: String,
    pub title: String,
    pub primary_artist: Option<String>,
    pub artists: Vec<String>,
    pub track_count: usize,
    pub artwork_path: Option<PathBuf>,
    pub metadata: Option<AlbumMetadata>,
    pub is_manual: bool,
}

/// Rich metadata about an album sourced from manual overrides or remote providers.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AlbumMetadata {
    pub summary: Option<String>,
    pub url: Option<String>,
    pub release_date: Option<String>,
    pub tags: Vec<String>,
    pub source: Option<String>,
}

impl AlbumMetadata {
    fn from_lastfm(album: &Value) -> Self {
        let summary = album
            .get("wiki")
            .and_then(|wiki| wiki.get("summary"))
            .and_then(|value| value.as_str())
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());

        let url = album
            .get("url")
            .and_then(|value| value.as_str())
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());

        let release_date = album
            .get("releasedate")
            .and_then(|value| value.as_str())
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());

        let tags = album
            .get("tags")
            .and_then(|tags| tags.get("tag"))
            .map(|node| {
                if let Some(array) = node.as_array() {
                    array
                        .iter()
                        .filter_map(|tag| {
                            tag.get("name")
                                .and_then(|value| value.as_str())
                                .map(|value| value.trim().to_string())
                                .filter(|value| !value.is_empty())
                        })
                        .collect::<Vec<_>>()
                } else {
                    node.get("name")
                        .and_then(|value| value.as_str())
                        .map(|value| vec![value.trim().to_string()])
                        .unwrap_or_default()
                }
            })
            .unwrap_or_default();

        Self {
            summary,
            url,
            release_date,
            tags,
            source: Some("lastfm".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ManualAlbumUpdate {
    pub title: Option<String>,
    pub primary_artist: Option<String>,
    pub search_album: Option<String>,
    pub search_artist: Option<String>,
    pub refresh_artwork: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumOverrideRecord {
    pub album_id: String,
    pub title: Option<String>,
    pub primary_artist: Option<String>,
    pub search_album: Option<String>,
    pub search_artist: Option<String>,
    pub metadata: Option<AlbumMetadata>,
    pub artwork_path: Option<String>,
    pub updated_at: DateTime<Utc>,
}

impl AlbumOverrideRecord {
    fn new(album_id: &str) -> Self {
        Self {
            album_id: album_id.to_string(),
            title: None,
            primary_artist: None,
            search_album: None,
            search_artist: None,
            metadata: None,
            artwork_path: None,
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlbumExportFormat {
    Json,
    Yaml,
}

#[derive(Clone)]
struct AlbumOverrideStore {
    path: PathBuf,
    data: Arc<Mutex<HashMap<String, AlbumOverrideRecord>>>,
}

impl AlbumOverrideStore {
    fn new() -> Self {
        let path = dirs::config_dir()
            .unwrap_or_else(|| {
                dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("~"))
                    .join(".config")
            })
            .join("hexendrum")
            .join("album_overrides.json");

        let data = Self::load_records(&path);

        Self {
            path,
            data: Arc::new(Mutex::new(data)),
        }
    }

    fn load_records(path: &Path) -> HashMap<String, AlbumOverrideRecord> {
        if !path.exists() {
            return HashMap::new();
        }

        match std::fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str::<Vec<AlbumOverrideRecord>>(&content) {
                Ok(records) => records
                    .into_iter()
                    .map(|record| (record.album_id.clone(), record))
                    .collect(),
                Err(error) => {
                    warn!("Failed to parse album override file {:?}: {}", path, error);
                    HashMap::new()
                }
            },
            Err(error) => {
                warn!("Failed to read album override file {:?}: {}", path, error);
                HashMap::new()
            }
        }
    }

    fn get(&self, album_id: &str) -> Option<AlbumOverrideRecord> {
        let data = self.data.lock().unwrap();
        data.get(album_id).cloned()
    }

    fn set(&self, record: AlbumOverrideRecord) -> Result<AlbumOverrideRecord> {
        {
            let mut data = self.data.lock().unwrap();
            data.insert(record.album_id.clone(), record.clone());
        }
        self.save()?;
        Ok(record)
    }

    fn save(&self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            ensure_directory(parent)?;
        }

        let snapshot: Vec<AlbumOverrideRecord> = {
            let data = self.data.lock().unwrap();
            let mut records: Vec<_> = data.values().cloned().collect();
            records.sort_by(|a, b| a.album_id.cmp(&b.album_id));
            records
        };

        let content = serde_json::to_string_pretty(&snapshot)?;
        std::fs::write(&self.path, content)?;
        Ok(())
    }

    fn export(&self, format: AlbumExportFormat) -> Result<String> {
        let snapshot = {
            let data = self.data.lock().unwrap();
            let mut records: Vec<_> = data.values().cloned().collect();
            records.sort_by(|a, b| a.album_id.cmp(&b.album_id));
            records
        };

        match format {
            AlbumExportFormat::Json => Ok(serde_json::to_string_pretty(&snapshot)?),
            AlbumExportFormat::Yaml => Ok(serde_yaml::to_string(&snapshot)?),
        }
    }
}

/// Service responsible for album aggregation and artwork caching
#[derive(Clone)]
pub struct AlbumService {
    cache_dir: PathBuf,
    lastfm_api_key: Option<String>,
    overrides: AlbumOverrideStore,
}

impl AlbumService {
    /// Create a new album service
    pub fn new(lastfm_api_key: Option<String>) -> Self {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| {
                dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("~"))
                    .join(".cache")
            })
            .join("hexendrum")
            .join("album_art");

        if let Err(error) = ensure_directory(&cache_dir) {
            warn!(
                "Failed to ensure album art cache directory {:?}: {}",
                cache_dir, error
            );
        }

        let overrides = AlbumOverrideStore::new();

        Self {
            cache_dir,
            lastfm_api_key: lastfm_api_key.filter(|value| !value.trim().is_empty()),
            overrides,
        }
    }

    /// Return the album artwork cache directory
    pub fn cache_directory(&self) -> &Path {
        &self.cache_dir
    }

    /// Export manual album overrides as JSON or YAML.
    pub fn export_overrides(&self, format: AlbumExportFormat) -> Result<String> {
        self.overrides.export(format)
    }

    /// Retrieve stored manual override details for an album.
    pub fn get_override(&self, album_id: &str) -> Option<AlbumOverrideRecord> {
        self.overrides.get(album_id)
    }

    /// Search albums using the library data, optionally filtering by query
    pub async fn search_albums(&self, library: &Library, query: Option<&str>) -> Vec<AlbumSummary> {
        let query = query
            .map(|value| value.trim().to_lowercase())
            .filter(|value| !value.is_empty());

        let mut aggregates: HashMap<String, AlbumAggregate> = HashMap::new();

        for track in library.get_tracks() {
            let album_title = match track.metadata.album.as_deref() {
                Some(title) if !title.trim().is_empty() => title.trim(),
                _ => continue,
            };

            let artist = track
                .metadata
                .artist
                .as_ref()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty());
            let album_id = album_identifier(artist, album_title);

            let entry = aggregates
                .entry(album_id.clone())
                .or_insert_with(|| AlbumAggregate {
                    id: album_id.clone(),
                    title: album_title.to_string(),
                    primary_artist: artist.map(|s| s.to_string()),
                    artists: HashSet::new(),
                    track_count: 0,
                    sample_track: None,
                });

            if entry.primary_artist.is_none() {
                entry.primary_artist = artist.map(|s| s.to_string());
            }

            if let Some(artist_value) = artist {
                entry.artists.insert(artist_value.to_string());
            }

            entry.track_count += 1;

            if entry.sample_track.is_none() {
                entry.sample_track = Some(track.clone());
            }
        }

        let mut summaries: Vec<AlbumSummary> = Vec::new();

        for aggregate in aggregates.into_values() {
            if let Some(ref q) = query {
                let matches_title = aggregate.title.to_lowercase().contains(q);
                let matches_artist = aggregate
                    .artists
                    .iter()
                    .any(|artist| artist.to_lowercase().contains(q));

                if !matches_title && !matches_artist {
                    continue;
                }
            }

            let mut artists: Vec<String> = aggregate.artists.into_iter().collect();
            artists.sort();

            let override_record = self.overrides.get(&aggregate.id);
            let mut title = aggregate.title.clone();
            let mut primary_artist = aggregate.primary_artist.clone();
            let mut metadata = None;
            let mut manual_artwork_path: Option<PathBuf> = None;

            if let Some(record) = override_record.as_ref() {
                if let Some(custom_title) = &record.title {
                    title = custom_title.clone();
                }

                if let Some(custom_artist) = &record.primary_artist {
                    primary_artist = Some(custom_artist.clone());
                }

                metadata = record.metadata.clone();
                manual_artwork_path = record.artwork_path.as_ref().map(PathBuf::from);
            }

            let artwork_path = if let Some(path) = manual_artwork_path {
                Some(path)
            } else if let Some(sample_track) = aggregate.sample_track.as_ref() {
                self.ensure_artwork(
                    &aggregate.id,
                    primary_artist.as_deref(),
                    &title,
                    sample_track,
                )
                .await
            } else {
                None
            };

            summaries.push(AlbumSummary {
                id: aggregate.id,
                title,
                primary_artist,
                artists,
                track_count: aggregate.track_count,
                artwork_path,
                metadata,
                is_manual: override_record.is_some(),
            });
        }

        summaries.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
        summaries
    }

    /// Manually override album metadata and refresh artwork/remote metadata when possible.
    pub async fn set_manual_override(
        &self,
        album_id: &str,
        update: ManualAlbumUpdate,
    ) -> Result<AlbumOverrideRecord> {
        if album_id.trim().is_empty() {
            return Err(anyhow!("album identifier must not be empty"));
        }

        let ManualAlbumUpdate {
            title,
            primary_artist,
            search_album,
            search_artist,
            refresh_artwork,
        } = update;

        if !refresh_artwork
            && title.is_none()
            && primary_artist.is_none()
            && search_album.is_none()
            && search_artist.is_none()
        {
            return Err(anyhow!(
                "at least one field must be provided when updating manual album metadata"
            ));
        }

        let mut record = self
            .overrides
            .get(album_id)
            .unwrap_or_else(|| AlbumOverrideRecord::new(album_id));

        if let Some(value) = title {
            record.title = normalize_override_string(value);
        }

        if let Some(value) = primary_artist {
            record.primary_artist = normalize_override_string(value);
        }

        if let Some(value) = search_album {
            record.search_album = normalize_override_string(value);
        }

        if let Some(value) = search_artist {
            record.search_artist = normalize_override_string(value);
        }

        record.updated_at = Utc::now();

        if let Some(api_key) = &self.lastfm_api_key {
            let lookup_artist = record
                .search_artist
                .clone()
                .or_else(|| record.primary_artist.clone());
            let lookup_album = record.search_album.clone().or_else(|| record.title.clone());

            if let (Some(artist), Some(album)) = (lookup_artist.as_deref(), lookup_album.as_deref())
            {
                if let Some(info) = self.fetch_lastfm_album_info(api_key, artist, album).await {
                    if let Some(url) = info.image_url {
                        if refresh_artwork || record.artwork_path.is_none() {
                            if let Some(path) = self.store_artwork_from_url(album_id, &url).await {
                                record.artwork_path = Some(path.to_string_lossy().to_string());
                            }
                        }
                    }

                    if let Some(metadata) = info.metadata {
                        record.metadata = Some(metadata);
                    }
                }
            }
        }

        if record.artwork_path.is_none() || refresh_artwork {
            if let Some(existing) = self.cached_artwork_path(album_id) {
                record.artwork_path = Some(existing.to_string_lossy().to_string());
            }
        }

        self.overrides.set(record)
    }

    /// Get the cached artwork path for an album if it exists
    pub fn cached_artwork_path(&self, album_id: &str) -> Option<PathBuf> {
        let path = self.cache_dir.join(format!("{}.jpg", album_id));
        if path.exists() {
            Some(path)
        } else {
            None
        }
    }

    async fn ensure_artwork(
        &self,
        album_id: &str,
        primary_artist: Option<&str>,
        album_title: &str,
        track: &Track,
    ) -> Option<PathBuf> {
        if let Some(path) = self.cached_artwork_path(album_id) {
            return Some(path);
        }

        let api_key = match &self.lastfm_api_key {
            Some(key) => key.clone(),
            None => return None,
        };

        let artist = primary_artist
            .map(|value| value.to_string())
            .or_else(|| track.metadata.artist.clone())?;

        let image_url = self
            .fetch_lastfm_image_url(
                &api_key,
                &artist,
                album_title,
                track.metadata.title.as_deref(),
            )
            .await?;

        self.store_artwork_from_url(album_id, &image_url).await
    }

    async fn store_artwork_from_url(&self, album_id: &str, image_url: &str) -> Option<PathBuf> {
        let bytes = self.fetch_bytes(image_url).await?;
        let path = self.cache_dir.join(format!("{}.jpg", album_id));

        if let Err(error) = fs::write(&path, &bytes).await {
            warn!("Failed to store album artwork at {:?}: {}", path, error);
            return None;
        }

        Some(path)
    }

    async fn fetch_lastfm_album_info(
        &self,
        api_key: &str,
        artist: &str,
        album: &str,
    ) -> Option<LastfmAlbumInfo> {
        let params = [
            ("method", "album.getinfo"),
            ("artist", artist),
            ("album", album),
            ("api_key", api_key),
            ("format", "json"),
        ];

        let value = self.fetch_lastfm_value(&params).await?;
        let album_value = value.get("album")?;

        let metadata = AlbumMetadata::from_lastfm(album_value);
        let image_url = extract_image_url(album_value.get("image"));

        Some(LastfmAlbumInfo {
            image_url,
            metadata: Some(metadata),
        })
    }

    async fn fetch_lastfm_value(&self, params: &[(&str, &str)]) -> Option<Value> {
        let params: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, *v)).collect();
        let query = serde_urlencoded::to_string(&params).ok()?;
        let url = format!("{}?{}", LAST_FM_ENDPOINT, query);

        let bytes = self.fetch_bytes(&url).await?;
        let value = serde_json::from_slice::<Value>(&bytes).ok()?;
        if value.get("error").is_some() {
            debug!("Last.fm returned error: {:?}", value);
            return None;
        }

        Some(value)
    }

    async fn fetch_lastfm_image_url(
        &self,
        api_key: &str,
        artist: &str,
        album: &str,
        track_title: Option<&str>,
    ) -> Option<String> {
        let mut params = vec![
            ("method", "album.getinfo"),
            ("artist", artist),
            ("album", album),
            ("api_key", api_key),
            ("format", "json"),
        ];

        if let Some(url) = self
            .perform_request(&params, |value| {
                extract_image_url(value.get("album")?.get("image"))
            })
            .await
        {
            return Some(url);
        }

        if let Some(title) = track_title {
            params = vec![
                ("method", "track.getInfo"),
                ("artist", artist),
                ("track", title),
                ("api_key", api_key),
                ("format", "json"),
            ];

            if let Some(url) = self
                .perform_request(&params, |value| {
                    extract_image_url(
                        value
                            .get("track")
                            .and_then(|track| track.get("album"))
                            .and_then(|album| album.get("image")),
                    )
                })
                .await
            {
                return Some(url);
            }
        }

        let search_term = build_search_term(artist, album, track_title);
        params = vec![
            ("method", "track.search"),
            ("track", &search_term),
            ("api_key", api_key),
            ("format", "json"),
        ];

        self.perform_request(&params, |value| {
            let results = value.get("results")?.get("trackmatches")?;
            let tracks = results.get("track")?;

            if let Some(array) = tracks.as_array() {
                array
                    .iter()
                    .find_map(|track| extract_image_url(track.get("image")))
            } else {
                extract_image_url(tracks.get("image"))
            }
        })
        .await
    }

    async fn perform_request<F>(&self, params: &[(&str, &str)], extract: F) -> Option<String>
    where
        F: Fn(&Value) -> Option<String>,
    {
        let value = self.fetch_lastfm_value(params).await?;
        extract(&value)
    }

    async fn fetch_bytes(&self, url: &str) -> Option<Vec<u8>> {
        let output = Command::new("curl")
            .args(["-sSL", url])
            .output()
            .await
            .ok()?;

        if !output.status.success() {
            debug!(
                "curl exited with status {:?} for url {}",
                output.status, url
            );
            return None;
        }

        Some(output.stdout)
    }
}

#[derive(Debug, Clone)]
struct LastfmAlbumInfo {
    image_url: Option<String>,
    metadata: Option<AlbumMetadata>,
}

fn normalize_override_string(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn extract_image_url(value: Option<&Value>) -> Option<String> {
    let images = value?.as_array()?;

    for size in LAST_FM_IMAGE_PRIORITY {
        for image in images {
            let image_size = image.get("size")?.as_str()?;
            if image_size == size {
                if let Some(url) = image.get("#text").and_then(|v| v.as_str()) {
                    if !url.trim().is_empty() {
                        return Some(url.to_string());
                    }
                }
            }
        }
    }

    None
}

fn build_search_term(artist: &str, album: &str, track_title: Option<&str>) -> String {
    if let Some(title) = track_title {
        format!("{} {} {}", artist, album, title)
    } else {
        format!("{} {}", artist, album)
    }
}

pub fn album_identifier(artist: Option<&str>, album: &str) -> String {
    use sha2::{Digest, Sha256};

    let normalized_album = normalize_album_title(album);
    let normalized_artist = normalize_primary_artist(artist);

    let mut hasher = Sha256::new();
    hasher.update(normalized_album.as_bytes());

    if let Some(artist_value) = normalized_artist {
        hasher.update("::");
        hasher.update(artist_value.as_bytes());
    }

    format!("{:x}", hasher.finalize())
}

fn normalize_album_title(album: &str) -> String {
    let lowered = album.to_lowercase();
    let stripped = strip_bracketed(&lowered);

    let mut sanitized = String::with_capacity(stripped.len());
    for ch in stripped.chars() {
        if ch.is_alphanumeric() || ch.is_whitespace() {
            sanitized.push(ch);
        } else {
            sanitized.push(' ');
        }
    }

    let raw_tokens: Vec<&str> = sanitized.split_whitespace().collect();

    if raw_tokens.is_empty() {
        return stripped.split_whitespace().collect::<Vec<_>>().join(" ");
    }

    let has_soundtrack = raw_tokens
        .iter()
        .any(|token| matches!(*token, "soundtrack" | "soundtracks" | "ost"));
    let has_original = raw_tokens.iter().any(|token| *token == "original");
    let has_score = raw_tokens.iter().any(|token| *token == "score");

    let mut tokens: Vec<&str> = Vec::new();

    for token in raw_tokens {
        if is_year_token(token) {
            continue;
        }

        if has_soundtrack {
            match token {
                "soundtrack" | "soundtracks" | "ost" | "game" | "motion" | "picture"
                | "official" => continue,
                "original" => continue,
                _ => {}
            }
        }

        if has_score && has_original && token == "score" {
            continue;
        }

        tokens.push(token);
    }

    if tokens.is_empty() {
        stripped.split_whitespace().collect::<Vec<_>>().join(" ")
    } else {
        tokens.join(" ")
    }
}

fn strip_bracketed(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut depth = 0usize;

    for ch in input.chars() {
        match ch {
            '(' | '[' | '{' | '<' => {
                depth += 1;
            }
            ')' | ']' | '}' | '>' => {
                if depth > 0 {
                    depth -= 1;
                }
            }
            _ => {
                if depth == 0 {
                    result.push(ch);
                }
            }
        }
    }

    result
}

fn is_year_token(token: &str) -> bool {
    if token.len() != 4 {
        return false;
    }

    match token.parse::<u32>() {
        Ok(year) => (1900..=2100).contains(&year),
        Err(_) => false,
    }
}

fn normalize_primary_artist(artist: Option<&str>) -> Option<String> {
    let artist = artist?.trim();
    if artist.is_empty() {
        return None;
    }

    let lowered = artist.to_lowercase();
    let stripped = strip_bracketed(&lowered);
    let mut prepared = stripped.replace(['\r', '\n', '\t'], " ");
    prepared = prepared.replace("feat.", "feat");
    prepared = prepared.replace("ft.", "ft");
    prepared = prepared.replace("vs.", "vs");
    prepared = prepared.replace("pres.", "pres");
    prepared = prepared.replace("prod.", "prod");
    prepared = prepared.replace("feat:", "feat ");
    prepared = prepared.replace("ft:", "ft ");
    prepared = prepared.replace("feat-", "feat ");
    prepared = prepared.replace("ft-", "ft ");

    let primary_slice = truncate_at_secondary_markers(&prepared);
    let segments = split_primary_artist_segments(primary_slice.trim());
    let mut normalized_segments: Vec<String> = Vec::new();

    for segment in segments {
        let cleaned = segment.trim();
        if cleaned.is_empty() {
            continue;
        }

        let canonical = cleaned
            .split_whitespace()
            .filter(|token| !token.is_empty())
            .collect::<Vec<_>>()
            .join(" ");

        if canonical.is_empty() || should_discard_artist_segment(&canonical) {
            continue;
        }

        normalized_segments.push(canonical);
    }

    if normalized_segments.is_empty() {
        return None;
    }

    normalized_segments.sort();
    normalized_segments.dedup();

    let mut result = normalized_segments[0].clone();
    for extra in normalized_segments.iter().skip(1) {
        result.push('|');
        result.push_str(extra);
    }

    Some(result)
}

fn truncate_at_secondary_markers(value: &str) -> &str {
    const SECONDARY_MARKERS: [&str; 10] = [
        " feat ",
        " featuring ",
        " ft ",
        " with ",
        " vs ",
        " x ",
        " presents ",
        " pres ",
        " produced by ",
        " prod by ",
    ];

    SECONDARY_MARKERS
        .iter()
        .filter_map(|marker| value.find(marker).map(|index| index))
        .min()
        .map(|index| &value[..index])
        .unwrap_or(value)
}

fn split_primary_artist_segments(value: &str) -> Vec<String> {
    const PRIMARY_PATTERNS: [&str; 6] = [",", ";", "/", "\\", " & ", " + "];

    let mut segments = vec![value.to_string()];

    for pattern in PRIMARY_PATTERNS {
        let mut next_segments: Vec<String> = Vec::new();

        for segment in segments {
            if segment.contains(pattern) {
                next_segments.extend(segment.split(pattern).map(|part| part.to_string()));
            } else {
                next_segments.push(segment);
            }
        }

        segments = next_segments;
    }

    let mut final_segments: Vec<String> = Vec::new();
    for segment in segments {
        if segment.contains(" and ") {
            let parts: Vec<String> = segment.split(" and ").map(|s| s.to_string()).collect();
            if parts.len() > 1
                && parts
                    .iter()
                    .skip(1)
                    .any(|part| part.trim_start().starts_with("the "))
            {
                final_segments.push(segment);
            } else {
                final_segments.extend(parts);
            }
        } else {
            final_segments.push(segment);
        }
    }

    final_segments
}

fn should_discard_artist_segment(segment: &str) -> bool {
    let trimmed = segment.trim();
    if trimmed.is_empty() {
        return true;
    }

    let tokens: Vec<&str> = trimmed.split_whitespace().collect();
    if tokens.is_empty() {
        return true;
    }

    let normalized = tokens.join(" ");

    matches!(
        normalized.as_str(),
        "various artists"
            | "various artist"
            | "original soundtrack"
            | "soundtrack"
            | "soundtracks"
            | "ost"
            | "original score"
            | "motion picture soundtrack"
            | "game soundtrack"
            | "original game soundtrack"
            | "video game soundtrack"
            | "score"
    ) || tokens.iter().any(|token| {
        matches!(
            *token,
            "softworks"
                | "studios"
                | "studio"
                | "records"
                | "recordings"
                | "publishing"
                | "company"
                | "interactive"
                | "llc"
                | "inc"
                | "team"
                | "soundteam"
        )
    })
}
