use hexendrum::library::Library;
use hexendrum::playlist::{PlaybackQueue, PlaylistManager, RepeatMode};
use serial_test::serial;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

struct PlaylistTestEnv {
    _workspace: TempDir,
    music_dir: PathBuf,
    playlist_dir: PathBuf,
    old_cache: Option<String>,
    old_config: Option<String>,
    old_home: Option<String>,
}

impl PlaylistTestEnv {
    fn new() -> Self {
        let workspace = tempfile::tempdir().expect("failed to create temp workspace");
        let music_dir = workspace.path().join("music");
        let cache_dir = workspace.path().join("cache");
        let config_dir = workspace.path().join("config");
        let playlist_dir = workspace.path().join("playlists");

        fs::create_dir(&music_dir).expect("failed to create music dir");
        fs::create_dir(&cache_dir).expect("failed to create cache dir");
        fs::create_dir(&config_dir).expect("failed to create config dir");
        fs::create_dir(&playlist_dir).expect("failed to create playlist dir");

        let old_cache = std::env::var("XDG_CACHE_HOME").ok();
        std::env::set_var("XDG_CACHE_HOME", &cache_dir);

        let old_config = std::env::var("XDG_CONFIG_HOME").ok();
        std::env::set_var("XDG_CONFIG_HOME", &config_dir);

        let old_home = std::env::var("HOME").ok();
        std::env::set_var("HOME", workspace.path());

        Self {
            _workspace: workspace,
            music_dir,
            playlist_dir,
            old_cache,
            old_config,
            old_home,
        }
    }

    fn music_dir(&self) -> PathBuf {
        self.music_dir.clone()
    }

    fn playlist_dir(&self) -> PathBuf {
        self.playlist_dir.clone()
    }

    fn create_audio_file<P: AsRef<Path>>(&self, name: P) -> PathBuf {
        let path = self.music_dir.join(name);
        fs::write(&path, b"fake audio data").expect("failed to write audio file");
        path
    }
}

impl Drop for PlaylistTestEnv {
    fn drop(&mut self) {
        if let Some(old_cache) = &self.old_cache {
            std::env::set_var("XDG_CACHE_HOME", old_cache);
        } else {
            std::env::remove_var("XDG_CACHE_HOME");
        }

        if let Some(old_config) = &self.old_config {
            std::env::set_var("XDG_CONFIG_HOME", old_config);
        } else {
            std::env::remove_var("XDG_CONFIG_HOME");
        }

        if let Some(old_home) = &self.old_home {
            std::env::set_var("HOME", old_home);
        } else {
            std::env::remove_var("HOME");
        }
    }
}

#[test]
#[serial]
fn playlist_manager_workflow_and_cleanup() {
    let env = PlaylistTestEnv::new();
    env.create_audio_file("track_a.mp3");
    env.create_audio_file("track_b.mp3");

    let library = Library::new();
    library
        .scan_directories(&[env.music_dir()])
        .expect("scan should succeed");

    let track_list = library.get_tracks();
    assert_eq!(track_list.len(), 2);
    let track_a = track_list[0].clone();
    let track_b = track_list[1].clone();

    let playlist_dir = env.playlist_dir();
    let manager = PlaylistManager::new(playlist_dir.clone()).expect("manager should initialize");

    let playlist_id = manager.create_playlist("My Playlist".into(), Some("Great tracks".into()));
    let mut playlist = manager
        .get_playlist(&playlist_id)
        .expect("playlist should exist");

    playlist.add_track(&track_a);
    playlist.add_track(&track_b);
    assert_eq!(playlist.track_count(), 2);
    assert!(!playlist.is_empty());
    assert!(playlist.move_track_up(1));
    assert!(playlist.move_track_down(0));
    playlist.mark_track_played(&track_a.id);
    assert!(playlist.remove_track(&track_b.id));
    playlist.add_track(&track_b);

    assert!(
        manager.update_playlist(playlist.clone()),
        "update should succeed"
    );
    manager
        .save_playlist(&playlist)
        .expect("playlist should save to disk");

    let playlist_path = playlist_dir.join(format!("{}.json", playlist.id));
    let loaded_playlist = manager
        .load_playlist(&playlist_path)
        .expect("playlist should load from disk");
    assert_eq!(loaded_playlist.track_count(), 2);

    manager.set_current_playlist(Some(playlist_id.clone()));
    assert_eq!(
        manager.get_current_playlist().as_deref(),
        Some(playlist_id.as_str())
    );
    manager.set_current_playlist(None);
    assert!(manager.get_current_playlist().is_none());

    assert!(
        manager.update_playlist(loaded_playlist.clone()),
        "update should succeed"
    );
    manager
        .load_all_playlists()
        .expect("loading playlists from disk should succeed");
    assert!(!manager.get_playlists().is_empty());

    // Remove a track from the library so cleanup routines have work to do
    assert!(library.remove_track(&track_b.id));
    let removed_from_single = manager
        .cleanup_playlist(&playlist_id, &library)
        .expect("cleanup should succeed");
    assert_eq!(removed_from_single, 1);

    let removed_total = manager
        .cleanup_missing_tracks(&library)
        .expect("global cleanup should succeed");
    assert_eq!(removed_total, 0);

    assert!(manager.delete_playlist(&playlist_id));
    assert!(manager.get_playlist(&playlist_id).is_none());
}

#[test]
fn playback_queue_operations_cover_all_branches() {
    let queue = PlaybackQueue::new();
    assert!(queue.is_empty());

    let tracks = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    queue.add_tracks(&tracks);
    assert_eq!(queue.len(), 3);

    assert_eq!(queue.next_track(), Some("a".into()));
    assert_eq!(queue.next_track(), Some("b".into()));
    assert_eq!(queue.previous_track(), Some("a".into()));

    queue.set_repeat_mode(RepeatMode::All);
    assert_eq!(queue.get_repeat_mode(), RepeatMode::All);
    assert_eq!(queue.previous_track(), Some("c".into()));

    queue.toggle_shuffle();
    assert!(queue.is_shuffle_enabled());
    queue.toggle_shuffle();
    assert!(!queue.is_shuffle_enabled());

    queue.clear();
    assert!(queue.is_empty());
    assert!(queue.next_track().is_none());
}
