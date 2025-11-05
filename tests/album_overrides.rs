use hexendrum::library::{
    album_identifier, AlbumExportFormat, AlbumService, Library, ManualAlbumUpdate,
};
use tempfile::TempDir;

struct AlbumTestEnv {
    _workspace: TempDir,
    music_dir: std::path::PathBuf,
    old_cache: Option<String>,
    old_config: Option<String>,
    old_home: Option<String>,
}

impl AlbumTestEnv {
    fn new() -> Self {
        let workspace = tempfile::tempdir().expect("failed to create temp workspace");
        let music_dir = workspace.path().join("music");
        let cache_dir = workspace.path().join("cache");
        let config_dir = workspace.path().join("config");

        std::fs::create_dir(&music_dir).expect("failed to create music dir");
        std::fs::create_dir(&cache_dir).expect("failed to create cache dir");
        std::fs::create_dir(&config_dir).expect("failed to create config dir");

        let old_cache = std::env::var("XDG_CACHE_HOME").ok();
        std::env::set_var("XDG_CACHE_HOME", &cache_dir);

        let old_config = std::env::var("XDG_CONFIG_HOME").ok();
        std::env::set_var("XDG_CONFIG_HOME", &config_dir);

        let old_home = std::env::var("HOME").ok();
        std::env::set_var("HOME", workspace.path());

        Self {
            _workspace: workspace,
            music_dir,
            old_cache,
            old_config,
            old_home,
        }
    }

    fn music_dir(&self) -> std::path::PathBuf {
        self.music_dir.clone()
    }

    fn create_audio_file<P: AsRef<std::path::Path>>(&self, name: P) -> std::path::PathBuf {
        let path = self.music_dir.join(name);
        std::fs::write(&path, b"fake audio data").expect("failed to write audio file");
        path
    }
}

impl Drop for AlbumTestEnv {
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

#[tokio::test]
async fn manual_override_can_be_set_and_exported() {
    let _env = AlbumTestEnv::new();
    let service = AlbumService::new(None);

    let update = ManualAlbumUpdate {
        title: Some("Custom Album".into()),
        primary_artist: Some("Manual Artist".into()),
        search_album: Some("Lookup Album".into()),
        search_artist: Some("Lookup Artist".into()),
        refresh_artwork: false,
    };

    let record = service
        .set_manual_override("album-manual-test", update)
        .await
        .expect("override should be stored");

    assert_eq!(
        record.title.as_deref(),
        Some("Custom Album"),
        "manual title should persist"
    );
    assert_eq!(
        record.primary_artist.as_deref(),
        Some("Manual Artist"),
        "manual artist should persist"
    );
    assert!(
        record.metadata.is_none(),
        "without API key metadata should remain unset"
    );

    let retrieved = service
        .get_override("album-manual-test")
        .expect("override should be retrievable");
    assert_eq!(
        retrieved.search_album.as_deref(),
        Some("Lookup Album"),
        "lookup details should persist"
    );

    let export_json = service
        .export_overrides(AlbumExportFormat::Json)
        .expect("JSON export should succeed");
    assert!(
        export_json.contains("Custom Album"),
        "export should include manual data"
    );

    let export_yaml = service
        .export_overrides(AlbumExportFormat::Yaml)
        .expect("YAML export should succeed");
    assert!(
        export_yaml.contains("Manual Artist"),
        "YAML export should include manual data"
    );
}

#[tokio::test]
async fn manual_override_updates_album_search_results() {
    let env = AlbumTestEnv::new();
    let service = AlbumService::new(None);
    let library = Library::new();

    env.create_audio_file("track.mp3");
    library
        .scan_directories(&[env.music_dir()])
        .expect("scan should succeed");

    let track = library
        .get_tracks()
        .into_iter()
        .next()
        .expect("library should contain the scanned track");
    let album_name = track
        .metadata
        .album
        .clone()
        .expect("track should expose album metadata");

    let album_id = album_identifier(track.metadata.artist.as_deref(), &album_name);

    service
        .set_manual_override(
            &album_id,
            ManualAlbumUpdate {
                title: Some("Renamed Album".into()),
                primary_artist: Some("Manual Artist".into()),
                search_album: None,
                search_artist: None,
                refresh_artwork: false,
            },
        )
        .await
        .expect("manual override should be stored");

    let albums = service.search_albums(&library, None).await;
    let summary = albums
        .into_iter()
        .find(|album| album.id == album_id)
        .expect("album summary should exist");

    assert_eq!(summary.title, "Renamed Album");
    assert_eq!(
        summary.primary_artist.as_deref(),
        Some("Manual Artist"),
        "manual artist should override aggregated artist"
    );
    assert!(
        summary.is_manual,
        "album summary should be marked as manually overridden"
    );
}
