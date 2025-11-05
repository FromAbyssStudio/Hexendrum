//! Hexendrum - A modern, open-source music player written in Rust
//!
//! This crate provides a complete music player application with:
//! - Audio playback support for multiple formats
//! - Music library management
//! - Playlist functionality
//! - Modern GUI interface
//! - Configuration management

pub mod api;
pub mod audio;
pub mod config;

pub mod events;
pub mod library;
pub mod playlist;
pub mod utils;

// Re-export commonly used types
pub use audio::{AudioPlayer, AudioState};
pub use config::Config;
pub use events::{EventBus, EventMessage, EventPayload};
pub use library::{Library, Track, TrackMetadata};
pub use playlist::{Playlist, PlaylistManager};

/// The current version of Hexendrum
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The application name
pub const APP_NAME: &str = "Hexendrum";

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_app_constants() {
        assert_eq!(APP_NAME, "Hexendrum");
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.audio.default_volume, 0.7);
        assert_eq!(config.audio.sample_rate, 44100);
        assert!(config.library.auto_scan);
    }

    #[test]
    fn test_library_creation() {
        let workspace = tempdir().expect("failed to create temp workspace");
        let cache_dir = workspace.path().join("cache");
        let config_dir = workspace.path().join("config");

        fs::create_dir(&cache_dir).expect("failed to create cache dir");
        fs::create_dir(&config_dir).expect("failed to create config dir");

        let old_cache = std::env::var("XDG_CACHE_HOME").ok();
        let old_config = std::env::var("XDG_CONFIG_HOME").ok();
        let old_home = std::env::var("HOME").ok();

        std::env::set_var("XDG_CACHE_HOME", &cache_dir);
        std::env::set_var("XDG_CONFIG_HOME", &config_dir);
        std::env::set_var("HOME", workspace.path());

        let library = Library::new();
        assert_eq!(library.track_count(), 0);
        assert!(!library.is_scanning());

        drop(library);

        if let Some(old) = old_cache {
            std::env::set_var("XDG_CACHE_HOME", old);
        } else {
            std::env::remove_var("XDG_CACHE_HOME");
        }

        if let Some(old) = old_config {
            std::env::set_var("XDG_CONFIG_HOME", old);
        } else {
            std::env::remove_var("XDG_CONFIG_HOME");
        }

        if let Some(old) = old_home {
            std::env::set_var("HOME", old);
        } else {
            std::env::remove_var("HOME");
        }
    }
}
