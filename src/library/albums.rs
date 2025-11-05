use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use serde_json::Value;
use tokio::fs;
use tokio::process::Command;
use tracing::{debug, warn};

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
}

/// Service responsible for album aggregation and artwork caching
#[derive(Clone)]
pub struct AlbumService {
    cache_dir: PathBuf,
    lastfm_api_key: Option<String>,
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

        Self {
            cache_dir,
            lastfm_api_key: lastfm_api_key.filter(|value| !value.trim().is_empty()),
        }
    }

    /// Return the album artwork cache directory
    pub fn cache_directory(&self) -> &Path {
        &self.cache_dir
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

            let artwork_path = if let Some(sample_track) = aggregate.sample_track.as_ref() {
                self.ensure_artwork(
                    &aggregate.id,
                    aggregate.primary_artist.as_deref(),
                    &aggregate.title,
                    sample_track,
                )
                .await
            } else {
                None
            };

            summaries.push(AlbumSummary {
                id: aggregate.id,
                title: aggregate.title,
                primary_artist: aggregate.primary_artist,
                artists,
                track_count: aggregate.track_count,
                artwork_path,
            });
        }

        summaries.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
        summaries
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

        let bytes = self.fetch_bytes(&image_url).await?;
        let path = self.cache_dir.join(format!("{}.jpg", album_id));

        if let Err(error) = fs::write(&path, &bytes).await {
            warn!("Failed to store album artwork at {:?}: {}", path, error);
            return None;
        }

        Some(path)
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
        let params: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, *v)).collect();
        let query = serde_urlencoded::to_string(&params).ok()?;
        let url = format!("{}?{}", LAST_FM_ENDPOINT, query);

        let bytes = self.fetch_bytes(&url).await?;
        let value = serde_json::from_slice::<Value>(&bytes).ok()?;
        if value.get("error").is_some() {
            debug!("Last.fm returned error: {:?}", value);
            return None;
        }

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

    let mut hasher = Sha256::new();
    hasher.update(album.to_lowercase());

    if let Some(artist_value) = artist {
        hasher.update("::");
        hasher.update(artist_value.to_lowercase());
    }

    format!("{:x}", hasher.finalize())
}
