# Hexendrum API Reference 🦀

This document provides the complete API reference for the Hexendrum music player library.

## Table of Contents

- [Getting Started](#getting-started)
- [Core Types](#core-types)
- [Audio Module](#audio-module)
- [Library Module](#library-module)
- [Playlist Module](#playlist-module)
- [Configuration Module](#configuration-module)
- [Utilities Module](#utilities-module)
- [Error Handling](#error-handling)
- [Examples](#examples)

## Getting Started

### Adding Hexendrum to Your Project

```toml
# Cargo.toml
[dependencies]
hexendrum = "0.1.0"
```

### Basic Usage

```rust
use hexendrum::{AudioPlayer, Library, PlaylistManager};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize components
    let player = AudioPlayer::new()?;
    let library = Library::new();
    let playlist_manager = PlaylistManager::new("playlists/")?;
    
    // Use the components
    Ok(())
}
```

## Core Types

### Re-exports

```rust
// Main application types
pub use audio::{AudioPlayer, AudioState};
pub use config::Config;
pub use library::{Library, Track, TrackMetadata};
pub use playlist::{Playlist, PlaylistManager, PlaybackQueue};

// Constants
pub const VERSION: &str;
pub const APP_NAME: &str;
```

## Audio Module

### AudioPlayer

The main audio playback interface.

```rust
pub struct AudioPlayer {
    // Private fields
}

impl AudioPlayer {
    /// Creates a new audio player instance.
    pub fn new() -> Result<Self, anyhow::Error>
    
    /// Plays an audio file.
    pub fn play(&self, file_path: &Path) -> Result<(), anyhow::Error>
    
    /// Pauses playback.
    pub fn pause(&self) -> Result<(), anyhow::Error>
    
    /// Resumes playback.
    pub fn resume(&self) -> Result<(), anyhow::Error>
    
    /// Stops playback.
    pub fn stop(&self) -> Result<(), anyhow::Error>
    
    /// Sets the volume (0.0 to 1.0).
    pub fn set_volume(&self, volume: f32) -> Result<(), anyhow::Error>
    
    /// Gets the current volume.
    pub fn get_volume(&self) -> f32
    
    /// Gets the current playback state.
    pub fn get_state(&self) -> AudioState
    
    /// Gets the current track path.
    pub fn get_current_track(&self) -> Option<String>
    
    /// Checks if audio is playing.
    pub fn is_playing(&self) -> bool
    
    /// Checks if audio is paused.
    pub fn is_paused(&self) -> bool
    
    /// Checks if audio is stopped.
    pub fn is_stopped(&self) -> bool
}
```

### AudioState

Represents the current state of audio playback.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum AudioState {
    Stopped,
    Playing,
    Paused,
    Loading,
}
```

### Utility Functions

```rust
/// Gets the duration of an audio file.
pub fn get_audio_duration(file_path: &Path) -> Result<Duration, anyhow::Error>

/// Checks if a file is a supported audio format.
pub fn is_supported_audio_format(file_path: &Path) -> bool
```

## Library Module

### Library

Manages the music library and provides search functionality.

```rust
pub struct Library {
    // Private fields
}

impl Library {
    /// Creates a new music library.
    pub fn new() -> Self
    
    /// Scans directories for music files.
    pub fn scan_directories(&self, directories: &[PathBuf]) -> Result<(), anyhow::Error>
    
    /// Gets all tracks in the library.
    pub fn get_tracks(&self) -> Vec<Track>
    
    /// Gets a track by ID.
    pub fn get_track(&self, id: &str) -> Option<Track>
    
    /// Gets a track by file path.
    pub fn get_track_by_path(&self, path: &Path) -> Option<Track>
    
    /// Searches tracks by query.
    pub fn search_tracks(&self, query: &str) -> Vec<Track>
    
    /// Gets tracks by artist.
    pub fn get_tracks_by_artist(&self, artist: &str) -> Vec<Track>
    
    /// Gets tracks by album.
    pub fn get_tracks_by_album(&self, album: &str) -> Vec<Track>
    
    /// Gets all artists in the library.
    pub fn get_artists(&self) -> Vec<String>
    
    /// Gets all albums in the library.
    pub fn get_albums(&self) -> Vec<String>
    
    /// Gets the total track count.
    pub fn track_count(&self) -> usize
    
    /// Checks if the library is currently scanning.
    pub fn is_scanning(&self) -> bool
}
```

### Track

Represents a music track with metadata.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    /// Track metadata
    pub metadata: TrackMetadata,
    /// Unique identifier
    pub id: String,
}

impl Track {
    /// Creates a new track from a file path.
    pub fn new(file_path: PathBuf) -> Result<Self, anyhow::Error>
    
    /// Gets the display name for the track.
    pub fn display_name(&self) -> String
    
    /// Gets the album artist info.
    pub fn album_artist(&self) -> Option<String>
    
    /// Gets the album info.
    pub fn album(&self) -> Option<String>
}
```

### TrackMetadata

Contains all metadata for a track.

```rust
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

impl TrackMetadata {
    /// Creates metadata from a file.
    pub fn from_file(file_path: &Path) -> Result<Self, anyhow::Error>
}
```

## Playlist Module

### PlaylistManager

Manages playlists and provides CRUD operations.

```rust
pub struct PlaylistManager {
    // Private fields
}

impl PlaylistManager {
    /// Creates a new playlist manager.
    pub fn new(playlist_directory: PathBuf) -> Result<Self, anyhow::Error>
    
    /// Creates a new playlist.
    pub fn create_playlist(&self, name: String, description: Option<String>) -> String
    
    /// Gets a playlist by ID.
    pub fn get_playlist(&self, id: &str) -> Option<Playlist>
    
    /// Gets all playlists.
    pub fn get_playlists(&self) -> Vec<Playlist>
    
    /// Updates a playlist.
    pub fn update_playlist(&self, playlist: Playlist) -> bool
    
    /// Deletes a playlist.
    pub fn delete_playlist(&self, id: &str) -> bool
    
    /// Sets the current playlist.
    pub fn set_current_playlist(&self, id: Option<String>)
    
    /// Gets the current playlist.
    pub fn get_current_playlist(&self) -> Option<String>
    
    /// Saves a playlist to file.
    pub fn save_playlist(&self, playlist: &Playlist) -> Result<(), anyhow::Error>
    
    /// Loads a playlist from file.
    pub fn load_playlist(&self, file_path: &PathBuf) -> Result<Playlist, anyhow::Error>
    
    /// Loads all playlists from directory.
    pub fn load_all_playlists(&self) -> Result<(), anyhow::Error>
}
```

### Playlist

Represents a music playlist.

```rust
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

impl Playlist {
    /// Creates a new playlist.
    pub fn new(name: String, description: Option<String>) -> Self
    
    /// Adds a track to the playlist.
    pub fn add_track(&mut self, track: &Track)
    
    /// Removes a track from the playlist.
    pub fn remove_track(&mut self, track_id: &str) -> bool
    
    /// Moves a track up in the playlist.
    pub fn move_track_up(&mut self, index: usize) -> bool
    
    /// Moves a track down in the playlist.
    pub fn move_track_down(&mut self, index: usize) -> bool
    
    /// Clears all tracks from the playlist.
    pub fn clear(&mut self)
    
    /// Gets the track count.
    pub fn track_count(&self) -> usize
    
    /// Checks if the playlist is empty.
    pub fn is_empty(&self) -> bool
    
    /// Gets the total duration.
    pub fn total_duration(&self, library: &Library) -> u64
    
    /// Marks a track as played.
    pub fn mark_track_played(&mut self, track_id: &str)
}
```

### PlaybackQueue

Manages the playback queue and provides navigation.

```rust
pub struct PlaybackQueue {
    // Private fields
}

impl PlaybackQueue {
    /// Creates a new playback queue.
    pub fn new() -> Self
    
    /// Adds tracks to the queue.
    pub fn add_tracks(&self, track_ids: &[String])
    
    /// Clears the queue.
    pub fn clear(&self)
    
    /// Gets the next track.
    pub fn next_track(&self) -> Option<String>
    
    /// Gets the previous track.
    pub fn previous_track(&self) -> Option<String>
    
    /// Gets the current track.
    pub fn current_track(&self) -> Option<String>
    
    /// Sets the repeat mode.
    pub fn set_repeat_mode(&self, mode: RepeatMode)
    
    /// Gets the repeat mode.
    pub fn get_repeat_mode(&self) -> RepeatMode
    
    /// Toggles shuffle.
    pub fn toggle_shuffle(&self)
    
    /// Checks if shuffle is enabled.
    pub fn is_shuffle_enabled(&self) -> bool
    
    /// Gets the queue length.
    pub fn len(&self) -> usize
    
    /// Checks if the queue is empty.
    pub fn is_empty(&self) -> bool
}
```

### RepeatMode

Defines the repeat behavior for playback.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RepeatMode {
    None,
    One,
    All,
}
```

## Configuration Module

### Config

Main configuration structure containing all application settings.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Audio settings
    pub audio: AudioConfig,
    /// Library settings
    pub library: LibraryConfig,
    /// GUI settings
    pub gui: GuiConfig,
    /// Playlist settings
    pub playlist: PlaylistConfig,
}

impl Config {
    /// Loads configuration from file and environment.
    pub fn load() -> Result<Self, anyhow::Error>
    
    /// Saves configuration to file.
    pub fn save(&self) -> Result<(), anyhow::Error>
}

impl Default for Config {
    fn default() -> Self
}
```

### Configuration Sub-structs

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    /// Default volume (0.0 to 1.0)
    pub default_volume: f32,
    /// Audio output device
    pub output_device: Option<String>,
    /// Sample rate
    pub sample_rate: u32,
    /// Buffer size
    pub buffer_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryConfig {
    /// Default music directories to scan
    pub music_directories: Vec<PathBuf>,
    /// Supported audio file extensions
    pub supported_extensions: Vec<String>,
    /// Auto-scan on startup
    pub auto_scan: bool,
    /// Scan interval in seconds (0 = disabled)
    pub scan_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiConfig {
    /// Theme (light, dark, auto)
    pub theme: String,
    /// Window size
    pub window_size: (u32, u32),
    /// Window position
    pub window_position: Option<(i32, i32)>,
    /// Show file extensions
    pub show_file_extensions: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistConfig {
    /// Default playlist directory
    pub playlist_directory: PathBuf,
    /// Auto-save playlists
    pub auto_save: bool,
    /// Max playlist history
    pub max_history: usize,
}
```

## Utilities Module

### Formatting Functions

```rust
/// Formats duration as MM:SS.
pub fn format_duration(duration: Duration) -> String

/// Formats duration in seconds as MM:SS.
pub fn format_duration_seconds(seconds: u64) -> String

/// Formats file size in human readable format.
pub fn format_file_size(bytes: u64) -> String

/// Gets file extension from path.
pub fn get_file_extension(path: &Path) -> Option<String>

/// Checks if a file is an audio file.
pub fn is_audio_file(path: &Path) -> bool
```

### File Operations

```rust
/// Gets relative path from base directory.
pub fn get_relative_path(path: &Path, base: &Path) -> Option<PathBuf>

/// Ensures directory exists, creates if it doesn't.
pub fn ensure_directory(path: &Path) -> std::io::Result<()>

/// Gets file name without extension.
pub fn get_file_name_without_extension(path: &Path) -> Option<String>

/// Sanitizes filename for safe storage.
pub fn sanitize_filename(filename: &str) -> String
```

### Time Utilities

```rust
/// Parses time string in format MM:SS or HH:MM:SS.
pub fn parse_time_string(time_str: &str) -> Option<Duration>

/// Gets human readable time ago string.
pub fn time_ago(timestamp: DateTime<Utc>) -> String
```

### String Utilities

```rust
/// Truncates string to specified length with ellipsis.
pub fn truncate_string(s: &str, max_length: usize) -> String

/// Capitalizes first letter of string.
pub fn capitalize_first(s: &str) -> String

/// Converts string to title case.
pub fn to_title_case(s: &str) -> String
```

## Error Handling

### Error Types

Hexendrum uses `anyhow::Error` for most error handling, providing context and easy error propagation.

```rust
use anyhow::{Result, Context};

pub fn example_function() -> Result<()> {
    let file = File::open("audio.mp3")
        .with_context(|| "Failed to open audio file")?;
    
    // ... rest of function
    Ok(())
}
```

### Custom Error Types

For library APIs, specific error types are used:

```rust
#[derive(Debug, thiserror::Error)]
pub enum AudioError {
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Decoding error: {0}")]
    DecodingError(String),
}
```

## Examples

### Basic Audio Playback

```rust
use hexendrum::AudioPlayer;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let player = AudioPlayer::new()?;
    
    // Play an audio file
    player.play(Path::new("music/song.mp3"))?;
    
    // Wait for playback to finish
    while player.is_playing() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
    Ok(())
}
```

### Library Management

```rust
use hexendrum::{Library, Config};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let library = Library::new();
    let config = Config::load()?;
    
    // Scan music directories
    library.scan_directories(&config.library.music_directories)?;
    
    // Search for tracks
    let results = library.search_tracks("rock");
    println!("Found {} rock tracks", results.len());
    
    // Get all artists
    let artists = library.get_artists();
    println!("Artists in library: {:?}", artists);
    
    Ok(())
}
```

### Playlist Management

```rust
use hexendrum::{PlaylistManager, Library, Track};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let playlist_manager = PlaylistManager::new(PathBuf::from("playlists/"))?;
    let library = Library::new();
    
    // Create a new playlist
    let playlist_id = playlist_manager.create_playlist(
        "My Favorites".to_string(),
        Some("My favorite songs".to_string())
    );
    
    // Get the playlist
    if let Some(mut playlist) = playlist_manager.get_playlist(&playlist_id) {
        // Add some tracks
        let tracks = library.get_tracks();
        for track in tracks.iter().take(5) {
            playlist.add_track(track);
        }
        
        // Save the playlist
        playlist_manager.update_playlist(playlist);
    }
    
    Ok(())
}
```

### Configuration Management

```rust
use hexendrum::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let mut config = Config::load()?;
    
    // Modify settings
    config.audio.default_volume = 0.8;
    config.gui.theme = "dark".to_string();
    
    // Save configuration
    config.save()?;
    
    println!("Configuration updated and saved!");
    Ok(())
}
```

### Custom Audio Player

```rust
use hexendrum::{AudioPlayer, PlaybackQueue, Track};
use std::sync::{Arc, Mutex};

struct CustomPlayer {
    audio_player: Arc<AudioPlayer>,
    queue: Arc<PlaybackQueue>,
    current_track: Arc<Mutex<Option<Track>>>,
}

impl CustomPlayer {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            audio_player: Arc::new(AudioPlayer::new()?),
            queue: Arc::new(PlaybackQueue::new()),
            current_track: Arc::new(Mutex::new(None)),
        })
    }
    
    fn play_track(&self, track: &Track) -> Result<(), Box<dyn std::error::Error>> {
        self.audio_player.play(&track.metadata.file_path)?;
        
        let mut current = self.current_track.lock().unwrap();
        *current = Some(track.clone());
        
        Ok(())
    }
    
    fn next_track(&self) -> Option<String> {
        self.queue.next_track()
    }
}
```

---

**Need more examples?** Check our [GitHub repository](https://github.com/fromabyssstudio/Hexendrum) for additional code samples and use cases.
