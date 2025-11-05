# Changelog

All notable changes to Hexendrum will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project bootstrap
- Basic project structure and architecture
- Core modules: audio, library, playlist, gui, config, utils
- Comprehensive documentation and setup scripts
- CI/CD pipeline with GitHub Actions
- Development tools and scripts

### Changed
- N/A

### Deprecated
- N/A

### Removed
- N/A

### Fixed
- N/A

### Security
- N/A

## [0.1.0] - 2025-01-XX

### Added
- **Core Application Structure**
  - Main application entry point
  - Configuration management system
  - Logging and error handling

- **Audio Module**
  - Multi-format audio playback support
  - Audio state management
  - Volume control and audio device handling

- **Library Module**
  - Music library scanning and management
  - Track metadata extraction (ID3, etc.)
  - Search and filtering capabilities

- **Playlist Module**
  - Playlist creation and management
  - Playback queue system
  - Repeat and shuffle modes

- **GUI Module**
  - Modern egui-based interface
  - Multiple view modes (Library, Playlists, Now Playing, Settings)
  - Responsive and intuitive design

- **Configuration System**
  - User preferences management
  - Audio and library settings
  - Cross-platform configuration storage

- **Development Tools**
  - Comprehensive build system
  - Testing framework
  - Code formatting and linting
  - CI/CD pipeline

### Technical Details
- **Dependencies**: rodio, symphonia, egui, serde, tokio
- **Supported Formats**: MP3, FLAC, OGG, WAV, M4A, AAC
- **Platforms**: Linux, macOS, Windows
- **Architecture**: Modular, thread-safe, async-ready

---

## Version History

- **0.1.0**: Initial release with core functionality
- **Future**: Planned features and improvements

## Release Notes

### Alpha Release (0.1.0)
This is the initial alpha release of Hexendrum. While functional, it should be considered experimental and may contain bugs or incomplete features.

**Known Issues:**
- Limited audio format support
- Basic GUI implementation
- Minimal error handling
- No audio visualization

**Planned for Next Release:**
- Enhanced audio format support
- Improved GUI design
- Better error handling and user feedback
- Audio visualization features
- Performance optimizations

---

## Contributing

To add entries to this changelog, please follow the established format and include:
- Clear description of changes
- Type of change (Added, Changed, Deprecated, Removed, Fixed, Security)
- Impact on users
- Technical details when relevant

## Links

- [GitHub Repository](https://github.com/RogueFairyStudios/Hexendrum)
- [Issue Tracker](https://github.com/RogueFairyStudios/Hexendrum/issues)
- [Documentation](https://github.com/RogueFairyStudios/Hexendrum/wiki)
