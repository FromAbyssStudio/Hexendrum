use hexendrum::config::Config;
use serial_test::serial;
use std::fs;
use tempfile::TempDir;

fn setup_env() -> (TempDir, Option<String>, Option<String>, Option<String>) {
    let workspace = tempfile::tempdir().expect("failed to create temp workspace");
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

    (workspace, old_cache, old_config, old_home)
}

fn restore_env(old_cache: Option<String>, old_config: Option<String>, old_home: Option<String>) {
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

#[test]
#[serial]
fn config_save_and_load_roundtrip() {
    let (_workspace, old_cache, old_config, old_home) = setup_env();

    let mut config = Config::default();
    config.audio.default_volume = 0.42;
    config.library.auto_scan = false;
    config.playlist.auto_save = false;

    config.save().expect("saving config should succeed");

    let loaded = Config::load().expect("loading config should succeed");
    assert_eq!(loaded.audio.default_volume, 0.42);
    assert!(!loaded.library.auto_scan);
    assert!(!loaded.playlist.auto_save);

    restore_env(old_cache, old_config, old_home);
}
