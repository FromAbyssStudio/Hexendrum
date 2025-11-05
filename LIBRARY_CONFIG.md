# Library Directory Configuration

Hexendrum allows you to configure multiple music directories to scan for your music collection. This guide explains how to set up and manage your library directories.

## Quick Start

### 1. Through the Settings UI (Recommended)
1. Open Hexendrum
2. Go to **Settings** page
3. In the **Music Library** section, you'll see your current music directories
4. Use the **Add Directory** field to add new paths
5. Click the **Add Directory** button or press Enter

### 2. Through Configuration File
The configuration is stored in `config.json` in your Hexendrum installation directory. You can edit this file directly:

```json
{
  "library": {
    "musicDirectories": [
      "~/Music",
      "~/Downloads/Music",
      "/mnt/music",
      "/media/external/music"
    ]
  }
}
```

## Supported Path Formats

### Home Directory Shortcuts
- `~/Music` → `/home/username/Music`
- `~/Downloads/Music` → `/home/username/Downloads/Music`

### Absolute Paths
- `/mnt/music` → Mounted music drive
- `/media/external/music` → External USB drive
- `/home/username/Music` → Full path to music folder

### Network Paths
- `//server/music` → Network share (Windows-style)
- `/mnt/smb/music` → Mounted network share

## Default Directories

When you first install Hexendrum, it automatically adds these default directories:
- `~/Music` - Your home Music folder
- `~/Downloads/Music` - Music in Downloads
- `~/Desktop/Music` - Music on Desktop

## Configuration Options

### Library Settings
- **Auto-scan on startup**: Automatically scan directories when the app starts
- **Scan interval**: How often to scan for new files (in seconds, 0 = disabled)
- **Supported extensions**: File types to include (mp3, flac, ogg, wav, aac, m4a)

### Audio Settings
- **Default volume**: Starting volume level (0.0 to 1.0)
- **Crossfade**: Smooth transitions between tracks
- **Crossfade duration**: Length of crossfade in seconds

### Interface Settings
- **Theme**: Dark, Light, or Auto
- **Show file extensions**: Display file extensions in library view

## Managing Directories

### Adding Directories
1. Type the path in the "Add Directory" field
2. Use `~` for home directory shortcuts
3. Use absolute paths for external drives
4. Click "Add Directory" button

### Removing Directories
1. Find the directory in the list
2. Click the trash icon (🗑️) next to it
3. Confirm the removal

### Validating Configuration
1. Click "Validate Configuration" button
2. Check for any errors or warnings
3. Fix any issues found

## Troubleshooting

### Common Issues

#### Directory Not Found
- Ensure the directory path is correct
- Check if the directory exists
- Verify you have read permissions

#### Permission Denied
- Check file permissions on the directory
- Ensure the user running Hexendrum has access
- Try using absolute paths instead of shortcuts

#### External Drives Not Detected
- Make sure the drive is mounted
- Use absolute paths (e.g., `/mnt/music`)
- Check if the drive is accessible

#### Network Shares
- Ensure the share is mounted
- Use the mounted path (e.g., `/mnt/smb/music`)
- Check network connectivity

### Reset to Defaults
If you encounter issues, you can reset all settings:
1. Go to Settings page
2. Click "Reset to Defaults" button
3. Confirm the reset
4. Restart the application

## Advanced Configuration

### Custom File Extensions
Edit `config.json` to add custom file types:
```json
{
  "library": {
    "supportedExtensions": [
      "mp3", "flac", "ogg", "wav", "aac", "m4a",
      "opus", "wma", "aiff", "alac"
    ]
  }
}
```

### Exclude Patterns
Configure patterns to exclude certain files/folders:
```json
{
  "library": {
    "excludePatterns": [
      "**/.*",
      "**/System Volume Information/**",
      "**/Thumbs.db",
      "**/desktop.ini"
    ]
  }
}
```

### Scan Intervals
Set how often to scan for new files:
- **0**: Disabled (manual scan only)
- **300**: Every 5 minutes
- **3600**: Every hour
- **86400**: Daily

## Best Practices

1. **Use Home Directory Shortcuts**: `~/Music` is more portable than absolute paths
2. **Group Related Music**: Organize music into logical directories
3. **Avoid System Directories**: Don't scan system folders or temporary directories
4. **Regular Validation**: Use the validate button to check for issues
5. **Backup Configuration**: Keep a copy of your `config.json` file

## Support

If you encounter issues with library configuration:
1. Check the console for error messages
2. Validate your configuration
3. Try resetting to defaults
4. Check file permissions and paths
5. Ensure directories are accessible

For additional help, check the main README or create an issue on the project repository.
