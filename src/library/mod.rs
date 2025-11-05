use anyhow::Result;
use chrono::{DateTime, Utc};
use dirs;
use lofty::{file::TaggedFileExt, prelude::Accessor, probe::Probe};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tracing::{debug, info, warn};
use walkdir::WalkDir;

use crate::audio::is_supported_audio_format;
use crate::utils::ensure_directory;

mod albums;
pub use albums::{album_identifier, AlbumService, AlbumSummary};

fn merge_metadata_from_tag(
    tag: &dyn Accessor,
    title: &mut Option<String>,
    artist: &mut Option<String>,
    album: &mut Option<String>,
    track_number: &mut Option<u32>,
    year: &mut Option<i32>,
    genre: &mut Option<String>,
) {
    if title.is_none() {
        if let Some(value) = tag.title() {
            *title = Some(value.to_string());
        }
    }

    if artist.is_none() {
        if let Some(value) = tag.artist() {
            *artist = Some(value.to_string());
        }
    }

    if album.is_none() {
        if let Some(value) = tag.album() {
            *album = Some(value.to_string());
        }
    }

    if track_number.is_none() {
        if let Some(value) = tag.track() {
            *track_number = Some(value);
        }
    }

    if year.is_none() {
        if let Some(value) = tag.year() {
            *year = Some(value as i32);
        }
    }

    if genre.is_none() {
        if let Some(value) = tag.genre() {
            *genre = Some(value.to_string());
        }
    }
}

/// Track metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackMetadata {
    /// Track title
    pub title: Option<String>,
    /// Artist name
    pub artist: Option<String>,
    /// Album name
    pub album: Option<String>,
    /// Track number
    pub track_number: Option<u32>,
    /// Year
    pub year: Option<i32>,
    /// Genre
    pub genre: Option<String>,
    /// Duration in seconds
    pub duration: Option<u64>,
    /// File size in bytes
    pub file_size: u64,
    /// Last modified time
    pub last_modified: DateTime<Utc>,
    /// File path
    pub file_path: PathBuf,
}

/// A music track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    /// Track metadata
    pub metadata: TrackMetadata,
    /// Unique identifier
    pub id: String,
}

impl Track {
    /// Create a new track from a file path
    pub fn new(file_path: PathBuf) -> Result<Self> {
        let metadata = TrackMetadata::from_file(&file_path)?;
        let id = uuid::Uuid::new_v4().to_string();

        Ok(Self { metadata, id })
    }

    /// Get display name for the track
    pub fn display_name(&self) -> String {
        if let Some(title) = &self.metadata.title {
            if let Some(artist) = &self.metadata.artist {
                format!("{} - {}", artist, title)
            } else {
                title.clone()
            }
        } else {
            self.metadata
                .file_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        }
    }

    /// Get album artist info
    #[allow(dead_code)]
    pub fn album_artist(&self) -> Option<String> {
        self.metadata.artist.clone()
    }

    /// Get album info
    #[allow(dead_code)]
    pub fn album(&self) -> Option<String> {
        self.metadata.album.clone()
    }
}

impl TrackMetadata {
    /// Create metadata from a file
    pub fn from_file(file_path: &Path) -> Result<Self> {
        let metadata = std::fs::metadata(file_path)?;
        let file_size = metadata.len();
        let last_modified = metadata.modified()?.into();

        // Try to read metadata using Lofty (supports ID3, Vorbis Comments, MP4, etc.)
        let mut title = None;
        let mut artist = None;
        let mut album = None;
        let mut track_number = None;
        let mut year = None;
        let mut genre = None;

        if let Ok(tagged_file) = Probe::open(file_path).and_then(|p| p.read()) {
            if let Some(primary_tag) = tagged_file.primary_tag() {
                merge_metadata_from_tag(
                    primary_tag,
                    &mut title,
                    &mut artist,
                    &mut album,
                    &mut track_number,
                    &mut year,
                    &mut genre,
                );
            }

            for tag in tagged_file.tags() {
                merge_metadata_from_tag(
                    tag,
                    &mut title,
                    &mut artist,
                    &mut album,
                    &mut track_number,
                    &mut year,
                    &mut genre,
                );
            }
        }

        // Try to get duration using symphonia
        let duration = crate::audio::get_audio_duration(file_path)
            .ok()
            .map(|d| d.as_secs());

        Ok(Self {
            title,
            artist,
            album,
            track_number,
            year,
            genre,
            duration,
            file_size,
            last_modified,
            file_path: file_path.to_path_buf(),
        })
    }
}

/// Cache entry for a track - includes file modification time for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedTrack {
    track: Track,
    file_mtime: DateTime<Utc>,
}

/// Library cache structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LibraryCache {
    tracks: Vec<CachedTrack>,
    cached_at: DateTime<Utc>,
}

/// Music library
pub struct Library {
    tracks: Arc<Mutex<HashMap<String, Track>>>,
    track_paths: Arc<Mutex<HashMap<PathBuf, String>>>,
    is_scanning: Arc<Mutex<bool>>,
    cache_path: PathBuf,
}

impl Library {
    /// Create a new music library
    pub fn new() -> Self {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| {
                dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("~"))
                    .join(".cache")
            })
            .join("hexendrum");

        ensure_directory(&cache_dir).ok();

        let cache_path = cache_dir.join("library_cache.json");

        let library = Self {
            tracks: Arc::new(Mutex::new(HashMap::new())),
            track_paths: Arc::new(Mutex::new(HashMap::new())),
            is_scanning: Arc::new(Mutex::new(false)),
            cache_path,
        };

        // Try to load from cache automatically on creation
        if let Err(e) = library.load_from_cache() {
            debug!("Failed to auto-load from cache: {}", e);
        }

        library
    }

    /// Get cache file path
    fn get_cache_path(&self) -> &Path {
        &self.cache_path
    }

    /// Load library from cache
    pub fn load_from_cache(&self) -> Result<usize> {
        let cache_path = self.get_cache_path();

        if !cache_path.exists() {
            debug!("No cache file found at {:?}", cache_path);
            return Ok(0);
        }

        let content = fs::read_to_string(cache_path)?;
        let cache: LibraryCache = serde_json::from_str(&content)?;

        let mut tracks_map = HashMap::new();
        let mut track_paths_map = HashMap::new();
        let mut loaded_count = 0;
        let mut invalidated_count = 0;

        for cached_track in cache.tracks {
            let file_path = &cached_track.track.metadata.file_path;

            // Check if file still exists and modification time matches
            if file_path.exists() {
                if let Ok(metadata) = std::fs::metadata(file_path) {
                    if let Ok(file_mtime) = metadata.modified() {
                        let file_mtime_utc: DateTime<Utc> = file_mtime.into();

                        // If file hasn't changed, use cached data
                        if file_mtime_utc == cached_track.file_mtime {
                            tracks_map
                                .insert(cached_track.track.id.clone(), cached_track.track.clone());
                            track_paths_map
                                .insert(file_path.clone(), cached_track.track.id.clone());
                            loaded_count += 1;
                            continue;
                        } else {
                            debug!("File modified, will rescan: {:?}", file_path);
                            invalidated_count += 1;
                        }
                    }
                }
            } else {
                debug!("File no longer exists: {:?}", file_path);
                invalidated_count += 1;
            }
        }

        // Update library with cached tracks
        {
            let mut tracks = self.tracks.lock().unwrap();
            let mut track_paths = self.track_paths.lock().unwrap();
            *tracks = tracks_map;
            *track_paths = track_paths_map;
        }

        info!(
            "Loaded {} tracks from cache ({} invalidated)",
            loaded_count, invalidated_count
        );

        Ok(loaded_count)
    }

    /// Save library to cache
    pub fn save_to_cache(&self) -> Result<()> {
        let tracks = self.tracks.lock().unwrap();

        let cached_tracks: Vec<CachedTrack> = tracks
            .values()
            .filter_map(|track| {
                let file_path = &track.metadata.file_path;

                // Get current file modification time
                if let Ok(metadata) = std::fs::metadata(file_path) {
                    if let Ok(mtime) = metadata.modified() {
                        let mtime_utc: DateTime<Utc> = mtime.into();
                        return Some(CachedTrack {
                            track: track.clone(),
                            file_mtime: mtime_utc,
                        });
                    }
                }

                None
            })
            .collect();

        let cache = LibraryCache {
            tracks: cached_tracks,
            cached_at: Utc::now(),
        };

        let cache_path = self.get_cache_path();

        // Ensure parent directory exists
        if let Some(parent) = cache_path.parent() {
            ensure_directory(parent)?;
        }

        let content = serde_json::to_string_pretty(&cache)?;
        fs::write(cache_path, content)?;

        info!("Saved {} tracks to cache", cache.tracks.len());

        Ok(())
    }

    /// Clear the cache file
    #[allow(dead_code)]
    pub fn clear_cache(&self) -> Result<()> {
        let cache_path = self.get_cache_path();
        if cache_path.exists() {
            fs::remove_file(cache_path)?;
            info!("Cache cleared");
        }
        Ok(())
    }

    /// Scan directories for music files
    pub fn scan_directories(&self, directories: &[PathBuf]) -> Result<()> {
        eprintln!("Starting library scan...");
        eprintln!("Directories to scan: {:?}", directories);

        let mut is_scanning = self.is_scanning.lock().unwrap();
        if *is_scanning {
            eprintln!("Library scan already in progress");
            return Ok(());
        }
        *is_scanning = true;
        drop(is_scanning);

        let mut new_tracks = HashMap::new();
        let mut new_track_paths = HashMap::new();

        for directory in directories {
            eprintln!("Scanning directory: {:?}", directory);
            if directory.exists() && directory.is_dir() {
                eprintln!("Directory exists and is valid");
                self.scan_directory(directory, &mut new_tracks, &mut new_track_paths)?;
            } else {
                eprintln!(
                    "Directory does not exist or is not a directory: {:?}",
                    directory
                );
            }
        }

        // Update the library
        {
            let mut tracks = self.tracks.lock().unwrap();
            let mut track_paths = self.track_paths.lock().unwrap();

            eprintln!("Library scan completed. Total tracks: {}", new_tracks.len());

            *tracks = new_tracks;
            *track_paths = new_track_paths;
        }

        // Save to cache after scanning
        if let Err(e) = self.save_to_cache() {
            warn!("Failed to save library to cache: {}", e);
        }

        let mut is_scanning = self.is_scanning.lock().unwrap();
        *is_scanning = false;

        Ok(())
    }

    /// Scan a single directory
    fn scan_directory(
        &self,
        directory: &Path,
        tracks: &mut HashMap<String, Track>,
        track_paths: &mut HashMap<PathBuf, String>,
    ) -> Result<()> {
        eprintln!("Scanning directory contents: {:?}", directory);
        let mut file_count = 0;
        let mut audio_file_count = 0;

        for entry in WalkDir::new(directory)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            file_count += 1;

            if path.is_file() && is_supported_audio_format(path) {
                audio_file_count += 1;
                eprintln!("Found audio file: {:?}", path);
                if let Ok(track) = Track::new(path.to_path_buf()) {
                    eprintln!("Successfully created track: {}", track.display_name());
                    tracks.insert(track.id.clone(), track.clone());
                    track_paths.insert(path.to_path_buf(), track.id);
                } else {
                    eprintln!("Failed to create track from: {:?}", path);
                }
            }
        }

        eprintln!(
            "Directory scan complete: {} total files, {} audio files",
            file_count, audio_file_count
        );
        Ok(())
    }

    /// Get all tracks
    pub fn get_tracks(&self) -> Vec<Track> {
        let tracks = self.tracks.lock().unwrap();
        tracks.values().cloned().collect()
    }

    /// Get track by ID
    #[allow(dead_code)]
    pub fn get_track(&self, id: &str) -> Option<Track> {
        let tracks = self.tracks.lock().unwrap();
        tracks.get(id).cloned()
    }

    /// Get track by file path
    #[allow(dead_code)]
    pub fn get_track_by_path(&self, path: &Path) -> Option<Track> {
        let track_paths = self.track_paths.lock().unwrap();
        if let Some(id) = track_paths.get(path) {
            let tracks = self.tracks.lock().unwrap();
            tracks.get(id).cloned()
        } else {
            None
        }
    }

    /// Search tracks by query
    pub fn search_tracks(&self, query: &str) -> Vec<Track> {
        let tracks = self.tracks.lock().unwrap();
        let query_lower = query.to_lowercase();

        tracks
            .values()
            .filter(|track| {
                let title = track.metadata.title.as_deref().unwrap_or("").to_lowercase();
                let artist = track
                    .metadata
                    .artist
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase();
                let album = track.metadata.album.as_deref().unwrap_or("").to_lowercase();

                title.contains(&query_lower)
                    || artist.contains(&query_lower)
                    || album.contains(&query_lower)
            })
            .cloned()
            .collect()
    }

    /// Get tracks by artist
    #[allow(dead_code)]
    pub fn get_tracks_by_artist(&self, artist: &str) -> Vec<Track> {
        let tracks = self.tracks.lock().unwrap();
        tracks
            .values()
            .filter(|track| track.metadata.artist.as_deref() == Some(artist))
            .cloned()
            .collect()
    }

    /// Get tracks by album
    #[allow(dead_code)]
    pub fn get_tracks_by_album(&self, album: &str) -> Vec<Track> {
        let tracks = self.tracks.lock().unwrap();
        tracks
            .values()
            .filter(|track| track.metadata.album.as_deref() == Some(album))
            .cloned()
            .collect()
    }

    /// Get all artists
    pub fn get_artists(&self) -> Vec<String> {
        let tracks = self.tracks.lock().unwrap();
        let mut artists = std::collections::HashSet::new();

        for track in tracks.values() {
            if let Some(artist) = &track.metadata.artist {
                artists.insert(artist.clone());
            }
        }

        let mut artists: Vec<_> = artists.into_iter().collect();
        artists.sort();
        artists
    }

    /// Get all albums
    pub fn get_albums(&self) -> Vec<String> {
        let tracks = self.tracks.lock().unwrap();
        let mut albums = std::collections::HashSet::new();

        for track in tracks.values() {
            if let Some(album) = &track.metadata.album {
                albums.insert(album.clone());
            }
        }

        let mut albums: Vec<_> = albums.into_iter().collect();
        albums.sort();
        albums
    }

    /// Get track count
    pub fn track_count(&self) -> usize {
        let tracks = self.tracks.lock().unwrap();
        tracks.len()
    }

    /// Check if library is currently scanning
    #[allow(dead_code)]
    pub fn is_scanning(&self) -> bool {
        *self.is_scanning.lock().unwrap()
    }

    /// Get all track IDs that exist in the library
    #[allow(dead_code)]
    pub fn get_track_ids(&self) -> Vec<String> {
        let tracks = self.tracks.lock().unwrap();
        tracks.keys().cloned().collect()
    }

    /// Check if a track ID exists in the library
    pub fn track_exists(&self, track_id: &str) -> bool {
        let tracks = self.tracks.lock().unwrap();
        tracks.contains_key(track_id)
    }

    /// Remove a track from the library (e.g., when file is deleted)
    #[allow(dead_code)]
    pub fn remove_track(&self, track_id: &str) -> bool {
        let mut tracks = self.tracks.lock().unwrap();
        let mut track_paths = self.track_paths.lock().unwrap();

        if let Some(track) = tracks.remove(track_id) {
            track_paths.remove(&track.metadata.file_path);
            // Update cache after removal
            drop(tracks);
            drop(track_paths);
            if let Err(e) = self.save_to_cache() {
                warn!("Failed to update cache after track removal: {}", e);
            }
            true
        } else {
            false
        }
    }
}

/// Initialize the library system
pub async fn init() -> Result<()> {
    // Check if cache exists for logging
    let cache_path = dirs::cache_dir()
        .unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("~"))
                .join(".cache")
        })
        .join("hexendrum")
        .join("library_cache.json");

    if cache_path.exists() {
        info!("Library cache found at {:?}", cache_path);
        info!("Call library.load_from_cache() after creating a Library instance to load cached tracks");
    } else {
        debug!("No library cache found - will perform fresh scan on first use");
    }

    info!("Library system initialized successfully");
    Ok(())
}
