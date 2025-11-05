# Troubleshooting Guide

## Library Not Scanning Music

### Problem

The backend starts but doesn't automatically scan for music files, resulting in an empty library.

### Root Cause

The library system was designed to:
1. Load from cache on startup (if cache exists)
2. Require manual scanning via API endpoint or configuration

However, there was no automatic scanning on startup even if configured directories existed.

### Solution

The backend now:
1. **Automatically loads from cache** when a `Library` instance is created
2. **Checks configuration** for `auto_scan` and `music_directories` settings
3. **Automatically scans** configured directories if `auto_scan` is enabled

### How It Works Now

1. **On Startup:**
   - Library instance is created and automatically tries to load from cache
   - Logs show how many tracks were loaded from cache
   - Configuration is loaded (if exists)
   - If `auto_scan: true` and directories are configured, scanning starts automatically

2. **Manual Scanning:**
   - Via API: `POST /api/library/scan` with directories list
   - Via Frontend: Use Settings page to configure directories and scan

3. **Cache:**
   - After scanning, library is saved to cache
   - Cache is located at: `~/.cache/hexendrum/library_cache.json`
   - Cache is validated (checks if files still exist and haven't changed)

### Configuration

To enable auto-scan, create or edit `~/.config/hexendrum/config.toml`:

```toml
[library]
music_directories = ["/path/to/your/music"]
auto_scan = true
scan_interval = 300
```

### Checking Library Status

1. **Check backend logs:**
   ```bash
   cargo run
   ```
   Look for:
   - "Loaded X tracks from cache"
   - "Auto-scan enabled - scanning N directory(ies)..."
   - "Auto-scan completed: X tracks found"

2. **Check API:**
   ```bash
   curl http://127.0.0.1:3030/api/library/stats
   ```

3. **Check cache file:**
   ```bash
   ls -lh ~/.cache/hexendrum/library_cache.json
   ```

### Manual Scanning

If auto-scan is not enabled or you want to scan manually:

```bash
curl -X POST http://127.0.0.1:3030/api/library/scan \
  -H "Content-Type: application/json" \
  -d '{"directories": ["/path/to/your/music"]}'
```

Or use the Swagger UI at `http://127.0.0.1:3030/swagger-ui`

### Common Issues

1. **No tracks found:**
   - Check if directories exist and are accessible
   - Verify directory paths are correct (use absolute paths)
   - Check if files have supported extensions (.mp3, .flac, .ogg, .wav, .m4a)

2. **Cache not loading:**
   - Cache may be invalid or corrupted
   - Delete cache and rescan: `rm ~/.cache/hexendrum/library_cache.json`

3. **Auto-scan not working:**
   - Check if config file exists and is valid
   - Verify `auto_scan = true` in config
   - Check if `music_directories` are configured and exist

4. **Scan fails silently:**
   - Check backend logs for error messages
   - Verify file permissions
   - Check if directories are accessible

