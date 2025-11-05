# Hexendrum 🦀

A modern, open-source music player written in Rust with a beautiful GUI interface.

## Features

- **Multi-format Audio Support**: MP3, FLAC, OGG, WAV, M4A, AAC
- **Music Library Management**: Scan and organize your music collection
- **Playlist Support**: Create, edit, and manage playlists
- **Smart Search**: Search through your library by title, artist, or album
- **Advanced Playback Controls**: Play, pause, skip, volume control
- **Queue Management**: Build and manage playback queues
- **Modern UI**: Electron-powered desktop shell backed by a Rust API
- **Realtime Updates**: Playback, volume, and scan status via WebSocket
- **Configurable**: Customize audio settings, library paths, and more
- **CLI Playbar (optional)**: Follow playback progress in the terminal with `--cli-playbar`
- **Fast & Efficient**: Built in Rust for performance and reliability

## Screenshots

*Screenshots will be added once the application is running*

## Installation

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- System audio drivers
- Linux: ALSA/PulseAudio
- macOS: Core Audio
- Windows: WASAPI

### Building from Source

1. Clone the repository:
```bash
git clone https://github.com/RogueFairyStudios/Hexendrum.git
cd Hexendrum
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```

To display the terminal playbar while the backend runs:
```bash
cargo run --release -- --cli-playbar
```

### Development Build

For development with debugging enabled:
```bash
cargo build
cargo run
```

## Usage

### First Run

1. Launch Hexendrum
2. Go to **Settings** → **Library**
3. Add your music directories
4. Click **Scan Library** to import your music
5. Start playing!

### Basic Controls

- **Play/Pause**: Click the play button or press Space
- **Skip Track**: Use the next/previous buttons
- **Volume**: Adjust using the volume slider
- **Search**: Type in the search bar to find music

### Library Management

- **Scan Library**: Automatically scan configured directories for new music
- **Search**: Find tracks by title, artist, or album
- **Metadata Aware**: Track duration, album art, and tags are read directly from embedded metadata (via Lofty)
- **Browse**: Navigate your music collection by artist, album, or genre

### Playlists

- **Create Playlist**: Click "Create" in the Playlists view
- **Add Tracks**: Use the "+" button to add tracks to playlists
- **Edit**: Reorder tracks, remove tracks, or modify playlist details
- **Play**: Click "Play" to start a playlist

### Queue Management

- **Add to Queue**: Use the "+" button to add tracks to the current queue
- **Queue Controls**: Navigate through queued tracks
- **Repeat Modes**: No repeat, repeat one, or repeat all
- **Shuffle**: Randomize track order

## Configuration

Hexendrum stores configuration in:
- **Linux**: `~/.config/hexendrum/`
- **macOS**: `~/Library/Application Support/hexendrum/`
- **Windows**: `%APPDATA%\hexendrum\`

### Key Settings

- **Audio**: Volume, sample rate, buffer size
- **Library**: Music directories, auto-scan, scan interval
- **GUI**: Theme, window size, file extensions
- **Playlists**: Auto-save, history size

## Architecture

Hexendrum is built with a modular architecture:

- **`audio/`**: Playback engine, rodio integration, Symphonia duration probing
- **`library/`**: Music library indexing, Lofty-based tag extraction
- **`playlist/`**: Playlist, queue, and history management
- **`api/`**: Axum REST + WebSocket server consumed by the Electron renderer
- **`events/`**: Broadcast bus for realtime backend events
- **`renderer/`**: React/Electron frontend (see `package.json`)
- **`config/`**, **`utils/`**: Configuration management and helpers

### Dependencies

- **Audio**: `rodio`, `symphonia` for playback and format support
- **Frontend**: Electron, React (see `renderer/`)
- **Metadata**: `lofty` for cross-format tag extraction
- **File System**: `walkdir` for directory scanning
- **Serialization**: `serde`, `serde_json` for data persistence

## Development

### Project Structure

```
src/
├── main.rs          # Application entry point
├── lib.rs           # Library root and exports
├── api/             # Axum HTTP + WebSocket server
├── audio/           # Audio playback engine and duration probing
├── config/          # Configuration management
├── events/          # Event bus definitions
├── library/         # Library scanning and metadata handling
├── playlist/        # Playlist and queue handling
└── utils/           # Utility functions

renderer/
├── main.js          # Electron main process
├── preload.js       # Context bridge
└── src/             # React UI components
```

### Running Tests

```bash
cargo test
```

### Code Style

This project follows Rust conventions and uses:
- `rustfmt` for code formatting
- `clippy` for linting
- Comprehensive error handling with `anyhow`
- Async/await patterns where appropriate

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Troubleshooting

### Common Issues

**Audio not working:**
- Check system audio drivers
- Verify audio device selection in settings
- Ensure audio files are in supported formats

**Library not scanning:**
- Verify directory paths are correct
- Check file permissions
- Ensure audio files have supported extensions

**GUI not responding:**
- Check system requirements
- Update graphics drivers
- Verify egui compatibility

### Debug Mode

Run with debug logging:
```bash
RUST_LOG=debug cargo run
```

## Roadmap

- [ ] **Audio Visualization**: Waveform and spectrum displays
- [ ] **Streaming Support**: Online radio and streaming services
- [ ] **Crossfade**: Smooth transitions between tracks
- [ ] **Equalizer**: Advanced audio processing
- [ ] **Remote Control**: Web interface and mobile app
- [ ] **Cloud Sync**: Sync playlists across devices
- [ ] **Plugin System**: Extensible functionality
- [ ] **Dark/Light Themes**: Multiple visual themes

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- **egui**: Modern, fast GUI framework
- **rodio**: Cross-platform audio playback
- **symphonia**: Audio format decoding
- **Rust Community**: Excellent ecosystem and tools

## Support

- **Issues**: [GitHub Issues](https://github.com/RogueFairyStudios/Hexendrum/issues)
- **Discussions**: [GitHub Discussions](https://github.com/RogueFairyStudios/Hexendrum/discussions)
- **Wiki**: [Project Wiki](https://github.com/RogueFairyStudios/Hexendrum/wiki)

---

**Made with ❤️ by RogueFairyStudios**

*Hexendrum - Where music meets innovation*
