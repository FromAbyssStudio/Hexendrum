use anyhow::Result;
use config::{Config as ConfigFile, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Audio settings
    #[serde(default)]
    pub audio: AudioConfig,
    /// Library settings
    #[serde(default)]
    pub library: LibraryConfig,
    /// GUI settings
    #[serde(default)]
    pub gui: GuiConfig,
    /// Playlist settings
    #[serde(default)]
    pub playlist: PlaylistConfig,
    /// External services configuration
    #[serde(default)]
    pub services: ServicesConfig,
}

/// Audio playback configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
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

/// Music library configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
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

/// GUI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
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

/// Playlist configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PlaylistConfig {
    /// Default playlist directory
    pub playlist_directory: PathBuf,
    /// Auto-save playlists
    pub auto_save: bool,
    /// Max playlist history
    pub max_history: usize,
}

/// Third-party services configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ServicesConfig {
    /// Last.fm integration settings
    pub lastfm: LastFmConfig,
}

/// Last.fm API credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LastFmConfig {
    /// Public API key
    pub api_key: String,
    /// Shared secret
    pub shared_secret: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            audio: AudioConfig::default(),
            library: LibraryConfig::default(),
            gui: GuiConfig::default(),
            playlist: PlaylistConfig::default(),
            services: ServicesConfig::default(),
        }
    }
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            default_volume: 0.7,
            output_device: None,
            sample_rate: 44100,
            buffer_size: 4096,
        }
    }
}

impl Default for LibraryConfig {
    fn default() -> Self {
        Self {
            music_directories: vec![dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("~"))
                .join("Music")],
            supported_extensions: vec![
                "mp3".to_string(),
                "flac".to_string(),
                "ogg".to_string(),
                "wav".to_string(),
                "m4a".to_string(),
            ],
            auto_scan: true,
            scan_interval: 300, // 5 minutes
        }
    }
}

impl Default for GuiConfig {
    fn default() -> Self {
        Self {
            theme: "auto".to_string(),
            window_size: (1200, 800),
            window_position: None,
            show_file_extensions: false,
        }
    }
}

impl Default for PlaylistConfig {
    fn default() -> Self {
        Self {
            playlist_directory: dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("~/.config"))
                .join("hexendrum")
                .join("playlists"),
            auto_save: true,
            max_history: 100,
        }
    }
}

impl Default for ServicesConfig {
    fn default() -> Self {
        Self {
            lastfm: LastFmConfig::default(),
        }
    }
}

impl Default for LastFmConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            shared_secret: String::new(),
        }
    }
}

impl Config {
    /// Load configuration from file and environment
    pub fn load() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("hexendrum");

        let config_file = config_dir.join("config.toml");

        let config = ConfigFile::builder()
            .add_source(File::from(config_file.as_path()).required(false))
            .add_source(Environment::with_prefix("HEXENDRUM"))
            .build()?;

        let config: Config = config.try_deserialize()?;
        Ok(config)
    }

    /// Save configuration to file
    #[allow(dead_code)]
    pub fn save(&self) -> Result<()> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("hexendrum");

        std::fs::create_dir_all(&config_dir)?;

        let config_file = config_dir.join("config.toml");
        let config_str = toml::to_string_pretty(self)?;
        std::fs::write(config_file, config_str)?;

        Ok(())
    }
}
