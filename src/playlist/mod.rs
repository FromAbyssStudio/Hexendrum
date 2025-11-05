use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::library::{Library, Track};

/// Playlist entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistEntry {
    /// Track ID
    pub track_id: String,
    /// Added timestamp
    pub added_at: DateTime<Utc>,
    /// Play count
    pub play_count: u32,
    /// Last played timestamp
    pub last_played: Option<DateTime<Utc>>,
}

/// A music playlist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    /// Playlist ID
    pub id: String,
    /// Playlist name
    pub name: String,
    /// Playlist description
    pub description: Option<String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Modified timestamp
    pub modified_at: DateTime<Utc>,
    /// Playlist entries
    pub entries: Vec<PlaylistEntry>,
    /// Playlist file path (if saved)
    pub file_path: Option<PathBuf>,
}

#[allow(dead_code)]
impl Playlist {
    /// Create a new playlist
    pub fn new(name: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            created_at: now,
            modified_at: now,
            entries: Vec::new(),
            file_path: None,
        }
    }

    /// Add a track to the playlist
    pub fn add_track(&mut self, track: &Track) {
        let entry = PlaylistEntry {
            track_id: track.id.clone(),
            added_at: Utc::now(),
            play_count: 0,
            last_played: None,
        };

        self.entries.push(entry);
        self.modified_at = Utc::now();
    }

    /// Remove a track from the playlist
    pub fn remove_track(&mut self, track_id: &str) -> bool {
        let initial_len = self.entries.len();
        self.entries.retain(|entry| entry.track_id != track_id);
        let removed = initial_len != self.entries.len();

        if removed {
            self.modified_at = Utc::now();
        }

        removed
    }

    /// Move a track up in the playlist
    pub fn move_track_up(&mut self, index: usize) -> bool {
        if index > 0 && index < self.entries.len() {
            self.entries.swap(index, index - 1);
            self.modified_at = Utc::now();
            true
        } else {
            false
        }
    }

    /// Move a track down in the playlist
    pub fn move_track_down(&mut self, index: usize) -> bool {
        if index < self.entries.len() - 1 {
            self.entries.swap(index, index + 1);
            self.modified_at = Utc::now();
            true
        } else {
            false
        }
    }

    /// Clear all tracks from the playlist
    pub fn clear(&mut self) {
        self.entries.clear();
        self.modified_at = Utc::now();
    }

    /// Get track count
    pub fn track_count(&self) -> usize {
        self.entries.len()
    }

    /// Check if playlist is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get total duration
    pub fn total_duration(&self, library: &crate::library::Library) -> u64 {
        self.entries
            .iter()
            .filter_map(|entry| {
                library
                    .get_track(&entry.track_id)
                    .and_then(|track| track.metadata.duration)
            })
            .sum()
    }

    /// Mark track as played
    pub fn mark_track_played(&mut self, track_id: &str) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.track_id == track_id) {
            entry.play_count += 1;
            entry.last_played = Some(Utc::now());
            self.modified_at = Utc::now();
        }
    }
}

/// Playlist manager
pub struct PlaylistManager {
    playlists: Arc<Mutex<Vec<Playlist>>>,
    current_playlist: Arc<Mutex<Option<String>>>,
    playlist_directory: PathBuf,
}

#[allow(dead_code)]
impl PlaylistManager {
    /// Create a new playlist manager
    pub fn new(playlist_directory: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&playlist_directory)?;

        Ok(Self {
            playlists: Arc::new(Mutex::new(Vec::new())),
            current_playlist: Arc::new(Mutex::new(None)),
            playlist_directory,
        })
    }

    /// Create a new playlist
    pub fn create_playlist(&self, name: String, description: Option<String>) -> String {
        let playlist = Playlist::new(name, description);
        let id = playlist.id.clone();

        let mut playlists = self.playlists.lock().unwrap();
        playlists.push(playlist);

        id
    }

    /// Get a playlist by ID
    pub fn get_playlist(&self, id: &str) -> Option<Playlist> {
        let playlists = self.playlists.lock().unwrap();
        playlists.iter().find(|p| p.id == id).cloned()
    }

    /// Get all playlists
    pub fn get_playlists(&self) -> Vec<Playlist> {
        let playlists = self.playlists.lock().unwrap();
        playlists.clone()
    }

    /// Update a playlist
    pub fn update_playlist(&self, playlist: Playlist) -> bool {
        let mut playlists = self.playlists.lock().unwrap();

        if let Some(index) = playlists.iter().position(|p| p.id == playlist.id) {
            playlists[index] = playlist;
            true
        } else {
            false
        }
    }

    /// Delete a playlist
    pub fn delete_playlist(&self, id: &str) -> bool {
        let mut playlists = self.playlists.lock().unwrap();
        let initial_len = playlists.len();
        playlists.retain(|p| p.id != id);

        let removed = initial_len != playlists.len();
        if removed {
            let mut current = self.current_playlist.lock().unwrap();
            if current.as_deref() == Some(id) {
                *current = None;
            }
        }

        removed
    }

    /// Set current playlist
    pub fn set_current_playlist(&self, id: Option<String>) {
        let mut current = self.current_playlist.lock().unwrap();
        *current = id;
    }

    /// Get current playlist
    pub fn get_current_playlist(&self) -> Option<String> {
        self.current_playlist.lock().unwrap().clone()
    }

    /// Save playlist to file
    pub fn save_playlist(&self, playlist: &Playlist) -> Result<()> {
        let file_path = self
            .playlist_directory
            .join(format!("{}.json", playlist.id));
        let content = serde_json::to_string_pretty(playlist)?;
        std::fs::write(&file_path, content)?;

        Ok(())
    }

    /// Load playlist from file
    pub fn load_playlist(&self, file_path: &PathBuf) -> Result<Playlist> {
        let content = std::fs::read_to_string(file_path)?;
        let mut playlist: Playlist = serde_json::from_str(&content)?;
        playlist.file_path = Some(file_path.to_path_buf());
        Ok(playlist)
    }

    /// Load all playlists from directory
    pub fn load_all_playlists(&self) -> Result<()> {
        let mut playlists = Vec::new();

        for entry in std::fs::read_dir(&self.playlist_directory)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(playlist) = self.load_playlist(&path) {
                    playlists.push(playlist);
                }
            }
        }

        let mut playlists_guard = self.playlists.lock().unwrap();
        *playlists_guard = playlists;

        Ok(())
    }

    /// Clean up playlists by removing tracks that no longer exist in the library
    /// Returns the number of tracks removed across all playlists
    pub fn cleanup_missing_tracks(&self, library: &Library) -> Result<usize> {
        let mut playlists = self.playlists.lock().unwrap();
        let mut total_removed = 0;
        let mut playlists_to_save = Vec::new();

        for playlist in playlists.iter_mut() {
            let initial_count = playlist.entries.len();

            // Filter out entries whose tracks don't exist in the library
            playlist.entries.retain(|entry| {
                let exists = library.track_exists(&entry.track_id);
                if !exists {
                    debug!(
                        "Removing track {} from playlist '{}' - track not found in library",
                        entry.track_id, playlist.name
                    );
                }
                exists
            });

            let removed = initial_count - playlist.entries.len();
            if removed > 0 {
                total_removed += removed;
                playlist.modified_at = Utc::now();

                info!(
                    "Removed {} missing track(s) from playlist '{}'",
                    removed, playlist.name
                );

                // Clone playlist to save later (after releasing lock)
                playlists_to_save.push(playlist.clone());
            }
        }

        drop(playlists);

        // Save all updated playlists
        for playlist in playlists_to_save {
            if let Err(e) = self.save_playlist(&playlist) {
                warn!("Failed to save cleaned playlist '{}': {}", playlist.name, e);
            }
        }

        if total_removed > 0 {
            info!(
                "Playlist cleanup complete: {} missing track(s) removed across all playlists",
                total_removed
            );
        } else {
            debug!("Playlist cleanup complete: no missing tracks found");
        }

        Ok(total_removed)
    }

    /// Clean up a specific playlist by removing tracks that no longer exist
    /// Returns the number of tracks removed
    pub fn cleanup_playlist(&self, playlist_id: &str, library: &Library) -> Result<usize> {
        let mut playlists = self.playlists.lock().unwrap();

        let playlist = playlists.iter_mut().find(|p| p.id == playlist_id);

        if let Some(playlist) = playlist {
            let playlist_name = playlist.name.clone();
            let initial_count = playlist.entries.len();

            playlist.entries.retain(|entry| {
                let exists = library.track_exists(&entry.track_id);
                if !exists {
                    debug!(
                        "Removing track {} from playlist '{}' - track not found in library",
                        entry.track_id, playlist_name
                    );
                }
                exists
            });

            let removed = initial_count - playlist.entries.len();
            if removed > 0 {
                playlist.modified_at = Utc::now();

                info!(
                    "Removed {} missing track(s) from playlist '{}'",
                    removed, playlist_name
                );

                // Clone the playlist before dropping the lock
                let playlist_clone = playlist.clone();
                drop(playlists);

                // Save the updated playlist
                if let Err(e) = self.save_playlist(&playlist_clone) {
                    warn!("Failed to save cleaned playlist '{}': {}", playlist_name, e);
                }

                Ok(removed)
            } else {
                drop(playlists);
                Ok(0)
            }
        } else {
            drop(playlists);
            Err(anyhow::anyhow!("Playlist not found: {}", playlist_id))
        }
    }
}

/// Playback queue
pub struct PlaybackQueue {
    tracks: Arc<Mutex<VecDeque<String>>>,
    current_index: Arc<Mutex<Option<usize>>>,
    repeat_mode: Arc<Mutex<RepeatMode>>,
    shuffle: Arc<Mutex<bool>>,
}

/// Repeat mode for playback
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RepeatMode {
    None,
    One,
    All,
}

#[allow(dead_code)]
impl PlaybackQueue {
    /// Create a new playback queue
    pub fn new() -> Self {
        Self {
            tracks: Arc::new(Mutex::new(VecDeque::new())),
            current_index: Arc::new(Mutex::new(None)),
            repeat_mode: Arc::new(Mutex::new(RepeatMode::None)),
            shuffle: Arc::new(Mutex::new(false)),
        }
    }

    /// Add tracks to the queue
    pub fn add_tracks(&self, track_ids: &[String]) {
        let mut tracks = self.tracks.lock().unwrap();
        tracks.extend(track_ids.iter().cloned());
    }

    /// Clear the queue
    pub fn clear(&self) {
        let mut tracks = self.tracks.lock().unwrap();
        tracks.clear();

        let mut current_index = self.current_index.lock().unwrap();
        *current_index = None;
    }

    /// Get next track
    pub fn next_track(&self) -> Option<String> {
        let tracks = self.tracks.lock().unwrap();
        let mut current_index = self.current_index.lock().unwrap();

        match *current_index {
            Some(index) => {
                if index + 1 < tracks.len() {
                    *current_index = Some(index + 1);
                    Some(tracks[index + 1].clone())
                } else {
                    match *self.repeat_mode.lock().unwrap() {
                        RepeatMode::All => {
                            *current_index = Some(0);
                            Some(tracks[0].clone())
                        }
                        _ => None,
                    }
                }
            }
            None => {
                if !tracks.is_empty() {
                    *current_index = Some(0);
                    Some(tracks[0].clone())
                } else {
                    None
                }
            }
        }
    }

    /// Get previous track
    pub fn previous_track(&self) -> Option<String> {
        let tracks = self.tracks.lock().unwrap();
        let mut current_index = self.current_index.lock().unwrap();

        match *current_index {
            Some(index) => {
                if index > 0 {
                    *current_index = Some(index - 1);
                    Some(tracks[index - 1].clone())
                } else {
                    match *self.repeat_mode.lock().unwrap() {
                        RepeatMode::All => {
                            let new_index = tracks.len() - 1;
                            *current_index = Some(new_index);
                            Some(tracks[new_index].clone())
                        }
                        _ => None,
                    }
                }
            }
            None => None,
        }
    }

    /// Get current track
    pub fn current_track(&self) -> Option<String> {
        let tracks = self.tracks.lock().unwrap();
        let current_index = self.current_index.lock().unwrap();

        current_index.and_then(|index| tracks.get(index)).cloned()
    }

    /// Set repeat mode
    pub fn set_repeat_mode(&self, mode: RepeatMode) {
        let mut repeat_mode = self.repeat_mode.lock().unwrap();
        *repeat_mode = mode;
    }

    /// Get repeat mode
    pub fn get_repeat_mode(&self) -> RepeatMode {
        *self.repeat_mode.lock().unwrap()
    }

    /// Toggle shuffle
    pub fn toggle_shuffle(&self) {
        let mut shuffle = self.shuffle.lock().unwrap();
        *shuffle = !*shuffle;
    }

    /// Check if shuffle is enabled
    pub fn is_shuffle_enabled(&self) -> bool {
        *self.shuffle.lock().unwrap()
    }

    /// Get queue length
    pub fn len(&self) -> usize {
        self.tracks.lock().unwrap().len()
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.tracks.lock().unwrap().is_empty()
    }
}

/// Initialize the playlist system
pub async fn init() -> Result<()> {
    // Initialize the playlist system with default settings
    info!("Playlist system initialized successfully");
    Ok(())
}
