use anyhow::Result;
use dirs;
use events::{EventBus, EventPayload};
use std::sync::Arc;
use tracing::{debug, error, info, Level};
use tracing_subscriber::FmtSubscriber;

mod api;
mod audio;
mod config;
mod events;
mod library;
mod playlist;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let show_cli_playbar = std::env::args().any(|arg| arg == "--cli-playbar");

    // Initialize logging
    let _subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("Starting Hexendrum Music Player Backend...");

    // Initialize the audio system
    match audio::init().await {
        Ok(_) => info!("Audio system initialized successfully"),
        Err(e) => {
            error!("Failed to initialize audio system: {}", e);
            return Err(e);
        }
    }

    // Initialize the library system
    match library::init().await {
        Ok(_) => info!("Library system initialized successfully"),
        Err(e) => {
            error!("Failed to initialize library system: {}", e);
            return Err(e);
        }
    }

    // Initialize the playlist system
    match playlist::init().await {
        Ok(_) => info!("Playlist system initialized successfully"),
        Err(e) => {
            error!("Failed to initialize playlist system: {}", e);
            return Err(e);
        }
    }

    // Initialize library and playlist manager instances
    let library = Arc::new(library::Library::new());

    // Check if library loaded from cache
    let cached_track_count = library.track_count();
    if cached_track_count > 0 {
        info!("Loaded {} tracks from cache", cached_track_count);
    } else {
        info!("No tracks loaded from cache - library is empty");
    }

    // Load configuration
    let config = match config::Config::load() {
        Ok(config) => config,
        Err(error) => {
            debug!(
                "Could not load config file, using defaults (no auto-scan): {}",
                error
            );
            config::Config::default()
        }
    };

    let event_bus = Arc::new(EventBus::new(None));

    if show_cli_playbar {
        info!("CLI playbar enabled (--cli-playbar)");
        spawn_cli_playbar(event_bus.clone());
    }

    if config.library.auto_scan && !config.library.music_directories.is_empty() {
        info!(
            "Auto-scan enabled - scanning {} directory(ies)...",
            config.library.music_directories.len()
        );
        let library_clone = library.clone();
        let directories = config.library.music_directories.clone();
        let event_bus_clone = event_bus.clone();
        tokio::spawn(async move {
            event_bus_clone.emit(EventPayload::library_scan("started", None, None));
            match library_clone.scan_directories(&directories) {
                Ok(_) => {
                    let count = library_clone.track_count();
                    info!("Auto-scan completed: {} tracks found", count);
                    event_bus_clone.emit(EventPayload::library_scan("completed", None, None));
                    event_bus_clone.emit(EventPayload::library_updated(count));
                }
                Err(error) => {
                    error!("Auto-scan failed: {}", error);
                    event_bus_clone.emit(EventPayload::library_scan("failed", None, None));
                }
            }
        });
    } else if config.library.music_directories.is_empty() {
        info!("No music directories configured - skipping auto-scan");
    }

    let lastfm_api_key = config.services.lastfm.api_key.trim().to_string();
    let album_service = Arc::new(library::AlbumService::new(if lastfm_api_key.is_empty() {
        None
    } else {
        Some(lastfm_api_key.clone())
    }));

    if lastfm_api_key.is_empty() {
        info!("Last.fm API key not configured - album artwork caching disabled");
    } else {
        info!(
            "Album artwork cache directory: {:?}",
            album_service.cache_directory()
        );
    }

    let playlist_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config"))
        .join("hexendrum")
        .join("playlists");

    let playlist_manager = match playlist::PlaylistManager::new(playlist_dir.clone()) {
        Ok(manager) => {
            info!("Playlist manager initialized");
            Arc::new(manager)
        }
        Err(e) => {
            error!("Failed to create playlist manager: {}", e);
            return Err(e);
        }
    };

    // Create audio player instance
    let audio_player = match audio::AudioPlayer::new() {
        Ok(player) => {
            info!("Audio player initialized");
            Arc::new(player)
        }
        Err(e) => {
            error!("Failed to create audio player: {}", e);
            return Err(e);
        }
    };

    // Create API state
    let api_state = api::AppState {
        library: library.clone(),
        playlist_manager: playlist_manager.clone(),
        audio_player: audio_player.clone(),
        album_service: album_service.clone(),
        event_bus: event_bus.clone(),
    };

    // Start API server on port 3030
    let api_port = 3030;
    info!("Starting API server on port {}...", api_port);

    // Spawn API server in background
    let api_state_clone = api_state.clone();
    tokio::spawn(async move {
        if let Err(e) = api::start_server(api_state_clone, api_port).await {
            error!("API server error: {}", e);
        }
    });

    info!("Hexendrum backend services are ready");
    info!("API server running at http://127.0.0.1:{}", api_port);
    info!(
        "Swagger UI available at http://127.0.0.1:{}/swagger-ui",
        api_port
    );

    // Keep the backend running
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

fn spawn_cli_playbar(event_bus: Arc<EventBus>) {
    tokio::spawn(async move {
        use tokio::time::{interval, Duration, MissedTickBehavior};

        let mut ticker = interval(Duration::from_secs(1));
        ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

        let mut receiver = event_bus.subscribe();
        let mut track_label: Option<String> = None;
        let mut duration: Option<u64> = None;
        let mut progress: u64 = 0;
        let mut playing = false;
        let mut volume = 0.7f32;

        loop {
            tokio::select! {
                event = receiver.recv() => {
                    match event {
                        Ok(message) => match message.payload {
                            EventPayload::PlaybackState { state, track_path, track_id, volume: vol, track_duration } => {
                                if let Some(v) = vol {
                                    volume = v;
                                }

                                if let Some(d) = track_duration {
                                    duration = Some(d);
                                    if progress > d {
                                        progress = d;
                                    }
                                }

                                if let Some(identifier) = track_path.or(track_id) {
                                    if track_label.as_deref() != Some(identifier.as_str()) {
                                        track_label = Some(identifier);
                                        progress = 0;
                                    }
                                }

                                match state.as_str() {
                                    "playing" => playing = true,
                                    "paused" => playing = false,
                                    "stopped" => {
                                        playing = false;
                                        progress = 0;
                                    }
                                    _ => {}
                                }

                                render_cli_playbar(&track_label, progress, duration, volume, playing);
                            }
                            EventPayload::VolumeChanged { volume: vol } => {
                                volume = vol;
                                render_cli_playbar(&track_label, progress, duration, volume, playing);
                            }
                            EventPayload::LibraryScan { status, .. } => {
                                println!("\n[scan] {}", status);
                                render_cli_playbar(&track_label, progress, duration, volume, playing);
                            }
                            EventPayload::LibraryUpdated { total_tracks } => {
                                println!("\n[library] tracks: {}", total_tracks);
                                render_cli_playbar(&track_label, progress, duration, volume, playing);
                            }
                        },
                        Err(_) => break,
                    }
                }
                _ = ticker.tick() => {
                    if playing {
                        progress = progress.saturating_add(1);
                        if let Some(d) = duration {
                            if progress > d {
                                progress = d;
                            }
                        }
                        render_cli_playbar(&track_label, progress, duration, volume, playing);
                    }
                }
            }
        }
    });
}

fn render_cli_playbar(
    track_label: &Option<String>,
    progress: u64,
    duration: Option<u64>,
    volume: f32,
    playing: bool,
) {
    use std::io::Write;

    let status_icon = if playing { "▶" } else { "⏸" };
    let title = track_label
        .as_deref()
        .map(|s| truncate_title(s, 40))
        .unwrap_or_else(|| "No track".to_string());

    let elapsed = format_seconds(progress);
    let total = duration
        .map(format_seconds)
        .unwrap_or_else(|| "--:--".to_string());
    let remaining = duration
        .map(|d| d.saturating_sub(progress))
        .map(format_seconds)
        .unwrap_or_else(|| "--:--".to_string());
    let vol_percent = (volume * 100.0).round() as i32;

    let line = format!(
        "\r{} {} | {} / {} ( -{} ) | vol {:>3}%",
        status_icon, title, elapsed, total, remaining, vol_percent
    );

    let mut stdout = std::io::stdout();
    let _ = stdout.write_all(line.as_bytes());
    let _ = stdout.flush();
}

fn format_seconds(seconds: u64) -> String {
    let mins = seconds / 60;
    let secs = seconds % 60;
    format!("{}:{:02}", mins, secs)
}

fn truncate_title(title: &str, max_chars: usize) -> String {
    let mut result = String::with_capacity(max_chars);
    let mut count = 0;
    for ch in title.chars() {
        if count + 1 >= max_chars {
            result.push('…');
            return result;
        }
        result.push(ch);
        count += 1;
    }
    result
}
