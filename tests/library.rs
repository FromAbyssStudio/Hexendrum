use hexendrum::library::Library;
use serial_test::serial;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

struct LibraryTestEnv {
    _workspace: TempDir,
    music_dir: PathBuf,
    cache_dir: PathBuf,
    old_cache: Option<String>,
    old_config: Option<String>,
    old_home: Option<String>,
}

impl LibraryTestEnv {
    fn new() -> Self {
        let workspace = tempfile::tempdir().expect("failed to create temp workspace");
        let music_dir = workspace.path().join("music");
        let cache_dir = workspace.path().join("cache");
        let config_dir = workspace.path().join("config");

        fs::create_dir(&music_dir).expect("failed to create music dir");
        fs::create_dir(&cache_dir).expect("failed to create cache dir");
        fs::create_dir(&config_dir).expect("failed to create config dir");

        let old_cache = std::env::var("XDG_CACHE_HOME").ok();
        std::env::set_var("XDG_CACHE_HOME", &cache_dir);

        let old_config = std::env::var("XDG_CONFIG_HOME").ok();
        std::env::set_var("XDG_CONFIG_HOME", &config_dir);

        let old_home = std::env::var("HOME").ok();
        std::env::set_var("HOME", workspace.path());

        Self {
            _workspace: workspace,
            music_dir,
            cache_dir,
            old_cache,
            old_config,
            old_home,
        }
    }

    fn music_dir(&self) -> PathBuf {
        self.music_dir.clone()
    }

    fn cache_file(&self) -> PathBuf {
        self.cache_dir.join("hexendrum").join("library_cache.json")
    }

    fn create_audio_file<P: AsRef<Path>>(&self, name: P) -> PathBuf {
        let path = self.music_dir.join(name);
        fs::write(&path, b"fake audio data").expect("failed to write audio file");
        path
    }
}

impl Drop for LibraryTestEnv {
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
fn scan_directories_discovers_audio_files_and_populates_cache() {
    let env = LibraryTestEnv::new();
    let track_path = env.create_audio_file("sample.mp3");

    let library = Library::new();
    library
        .scan_directories(&[env.music_dir()])
        .expect("scan should succeed");

    assert_eq!(library.track_count(), 1);

    let retrieved = library
        .get_track_by_path(&track_path)
        .expect("track should be indexed");
    assert_eq!(retrieved.metadata.file_path, track_path);

    let ids = library.get_track_ids();
    assert_eq!(ids.len(), 1, "exactly one track id should be present");
    assert!(
        library.get_tracks_by_artist("non-existent").is_empty(),
        "unknown artist query should yield no tracks"
    );
    assert!(
        library.get_tracks_by_album("non-existent").is_empty(),
        "unknown album query should yield no tracks"
    );
    assert!(
        !library.is_scanning(),
        "scan should not report in-progress after completion"
    );

    let cache_file = env.cache_file();
    assert!(
        cache_file.exists(),
        "library cache should be written after scan"
    );

    library
        .clear_cache()
        .expect("clearing cache should succeed");
    assert!(
        !cache_file.exists(),
        "cache file should be removed after clear_cache()"
    );
}

#[test]
#[serial]
fn remove_track_updates_library_state() {
    let env = LibraryTestEnv::new();
    let keep_path = env.create_audio_file("keep.mp3");
    let remove_path = env.create_audio_file("remove.mp3");

    let library = Library::new();
    library
        .scan_directories(&[env.music_dir()])
        .expect("scan should succeed");

    assert_eq!(
        library.track_count(),
        2,
        "both audio files should be tracked"
    );

    let tracks = library.get_tracks();
    let remove_track_id = tracks
        .iter()
        .find(|t| t.metadata.file_path == remove_path)
        .map(|t| t.id.clone())
        .expect("remove track id should exist");

    assert!(
        library.remove_track(&remove_track_id),
        "remove_track should return true when track exists"
    );
    assert_eq!(
        library.track_count(),
        1,
        "library should reflect removal immediately"
    );
    assert!(
        library.get_track(&remove_track_id).is_none(),
        "removed track should no longer be retrievable by id"
    );
    assert!(
        library.get_track_by_path(&remove_path).is_none(),
        "removed track should no longer be retrievable by path"
    );
    assert!(
        library.get_track_by_path(&keep_path).is_some(),
        "remaining track should still be present"
    );
}
