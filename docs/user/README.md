# Hexendrum User Guide 🦀

Welcome to Hexendrum! This guide will help you get started with using the music player.

## Table of Contents

- [Installation](#installation)
- [First Run](#first-run)
- [Basic Usage](#basic-usage)
- [Library Management](#library-management)
- [Playlists](#playlists)
- [Settings](#settings)
- [Troubleshooting](#troubleshooting)

## Installation

### Prerequisites

- **Linux**: ALSA/PulseAudio audio system
  - Debian/Ubuntu: Install ALSA development libraries: `sudo apt-get install libasound2-dev pkg-config`
  - Arch Linux/SteamOS: Install ALSA library: `sudo pacman -S alsa-lib pkg-config`
  - The `alsa-sys` crate requires these libraries to build on Linux
- **macOS**: Core Audio (no additional dependencies needed)
- **Windows**: WASAPI audio system (no additional dependencies needed)
- **Rust 1.70+**: [Install Rust](https://rustup.rs/)

### Building from Source

1. **Clone the repository**:
   ```bash
   git clone https://github.com/fromabyssstudio/Hexendrum.git
   cd Hexendrum
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Run the application**:
   ```bash
   cargo run --release
   ```

### Using the Makefile

```bash
# Build and run in one command
make run-release

# Just build
make release

# Clean build artifacts
make clean
```

## First Run

### Initial Setup

1. **Launch Hexendrum** for the first time
2. **Go to Settings** → **Library**
3. **Add Music Directories**:
   - Click "Add Directory"
   - Navigate to your music folder
   - Select the folder and click "Add"
4. **Scan Library**:
   - Click "Scan Library" to import your music
   - Wait for the scan to complete
5. **Start Playing**!

### Configuration Directory

Hexendrum stores your settings in:
- **Linux**: `~/.config/hexendrum/`
- **macOS**: `~/Library/Application Support/hexendrum/`
- **Windows**: `%APPDATA%\hexendrum\`

## Basic Usage

### Main Interface

The Hexendrum interface has four main views:

1. **Library** - Browse and search your music collection
2. **Playlists** - Create and manage playlists
3. **Now Playing** - Current track and playback controls
4. **Settings** - Configure the application

### Playback Controls

- **Play/Pause**: ▶️ button or Spacebar
- **Previous Track**: ⏮ button
- **Next Track**: ⏭ button
- **Stop**: ⏹ button
- **Volume**: Use the volume slider

### Keyboard Shortcuts

- **Space**: Play/Pause
- **Left Arrow**: Previous track
- **Right Arrow**: Next track
- **Up/Down Arrow**: Volume up/down
- **Ctrl+S**: Save playlist
- **Ctrl+O**: Open file
- **Ctrl+F**: Search

## Library Management

### Scanning Your Music

1. **Automatic Scanning**: Hexendrum scans your library on startup
2. **Manual Scanning**: Click "Scan Library" in the Library view
3. **Add New Directories**: Go to Settings → Library to add more folders

### Supported Formats

- **MP3** (.mp3) - Most common format
- **FLAC** (.flac) - Lossless audio
- **OGG** (.ogg) - Open source format
- **WAV** (.wav) - Uncompressed audio
- **M4A** (.m4a) - AAC encoded audio
- **AAC** (.aac) - Advanced audio codec

### Searching and Filtering

- **Search Bar**: Type to search by title, artist, or album
- **Filters**: Use the filter buttons to show only specific types
- **Sorting**: Click column headers to sort by different criteria

### Metadata

Hexendrum automatically reads:
- Track title
- Artist name
- Album name
- Track number
- Year
- Genre
- Duration
- File size

## Playlists

### Creating Playlists

1. **Go to Playlists view**
2. **Click "Create New Playlist"**
3. **Enter a name** and optional description
4. **Add tracks** from your library

### Managing Playlists

- **Add Tracks**: Use the "+" button next to any track
- **Remove Tracks**: Select tracks and press Delete
- **Reorder**: Drag and drop tracks to reorder
- **Edit**: Double-click playlist name to edit
- **Delete**: Right-click playlist and select "Delete"

### Playlist Features

- **Auto-save**: Playlists are automatically saved
- **Import/Export**: Save playlists as files
- **Smart Playlists**: Create playlists based on criteria
- **Playlist History**: View recently played playlists

## Settings

### Audio Settings

- **Default Volume**: Set the starting volume level
- **Sample Rate**: Choose audio quality (44.1kHz recommended)
- **Buffer Size**: Adjust for performance vs. latency
- **Output Device**: Select your audio device

### Library Settings

- **Auto-scan**: Automatically scan for new music
- **Scan Interval**: How often to check for changes
- **File Extensions**: Supported audio formats
- **Exclude Patterns**: Skip certain files/folders

### GUI Settings

- **Theme**: Light, dark, or auto
- **Window Size**: Default window dimensions
- **Show File Extensions**: Display file extensions
- **Language**: Interface language (when available)

### Playlist Settings

- **Auto-save**: Save playlists automatically
- **History Size**: Number of recent playlists to remember
- **Default Directory**: Where to save new playlists

## Advanced Features

### Queue Management

- **Add to Queue**: Use the "+" button to add tracks
- **Queue Controls**: Navigate through queued tracks
- **Repeat Modes**: No repeat, repeat one, or repeat all
- **Shuffle**: Randomize track order

### Audio Effects

- **Equalizer**: Adjust frequency bands (coming soon)
- **Crossfade**: Smooth transitions between tracks (coming soon)
- **Normalization**: Consistent volume levels (coming soon)

### Remote Control

- **Web Interface**: Control from your browser (coming soon)
- **Mobile App**: Control from your phone (coming soon)
- **API**: Programmatic control (coming soon)

## Troubleshooting

### Common Issues

#### Audio Not Working

1. **Check System Audio**:
   - Ensure your system audio is working
   - Test with other applications

2. **Audio Device**:
   - Go to Settings → Audio
   - Select the correct output device
   - Try different devices if available

3. **File Format**:
   - Ensure audio files are in supported formats
   - Check if files play in other players

#### Library Not Scanning

1. **Directory Permissions**:
   - Check folder permissions
   - Ensure Hexendrum can read the directories

2. **File Types**:
   - Verify files have supported extensions
   - Check if files are corrupted

3. **Scan Settings**:
   - Go to Settings → Library
   - Verify scan directories are correct
   - Try manual scan

#### GUI Issues

1. **Graphics Drivers**:
   - Update your graphics drivers
   - Ensure OpenGL support

2. **Window Size**:
   - Reset window size in Settings → GUI
   - Try different themes

3. **Performance**:
   - Close other applications
   - Reduce library size for testing

### Debug Mode

Run with debug logging for more information:

```bash
RUST_LOG=debug cargo run
```

### Getting Help

- **GitHub Issues**: Report bugs and request features
- **GitHub Discussions**: Ask questions and get help
- **Documentation**: Check this guide and API docs
- **Community**: Join our Discord server (coming soon)

## Tips and Tricks

### Performance

- **Large Libraries**: Use SSD storage for better scan performance
- **Metadata**: Keep audio files organized with proper tags
- **Regular Scans**: Schedule regular library scans

### Organization

- **Folder Structure**: Organize music by artist/album
- **File Naming**: Use consistent naming conventions
- **Playlists**: Create playlists for different moods/activities

### Backup

- **Configuration**: Backup your config directory
- **Playlists**: Export important playlists
- **Library**: Keep your music files backed up

---

**Need more help?** Check our [Developer Documentation](../developer/README.md) or [API Reference](../api/README.md) for technical details.
