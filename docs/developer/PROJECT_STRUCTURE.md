# Hexendrum Project Structure 🦀

This document provides a comprehensive overview of the Hexendrum project structure, explaining the purpose and organization of each directory and file.

## Root Directory Structure

```
Hexendrum/
├── src/                    # Source code
├── docs/                   # Documentation
├── tests/                  # Test files
├── scripts/                # Build and maintenance scripts
├── assets/                 # Static assets
├── examples/               # Example code and configurations
├── tools/                  # Development tools
├── .github/                # GitHub-specific files
├── Cargo.toml             # Project configuration
├── Cargo.lock             # Dependency lock file
├── Makefile               # Build automation
├── LICENSE                # License information
├── README.md              # Main project documentation
└── .gitignore             # Git ignore patterns
```

## Source Code (`src/`)

### Main Files

- **`main.rs`**: Application entry point, initializes logging and launches GUI
- **`lib.rs`**: Library root, exports public API and defines module structure

### Core Modules

#### Audio Module (`src/audio/`)
- **`mod.rs`**: Module definition and public exports
- **Audio playback engine**: Handles multi-format audio playback
- **Format detection**: Identifies and validates audio file formats
- **Device management**: Manages audio output devices
- **State management**: Tracks playback state (playing, paused, stopped)

#### Library Module (`src/library/`)
- **`mod.rs`**: Module definition and public exports
- **Music library management**: Scans directories and builds music database
- **Metadata extraction**: Reads ID3 tags and audio file information
- **Search functionality**: Provides search and filtering capabilities
- **File organization**: Manages track relationships and categorization

#### Playlist Module (`src/playlist/`)
- **`mod.rs`**: Module definition and public exports
- **Playlist management**: CRUD operations for playlists
- **Playback queue**: Manages track queue and navigation
- **Import/export**: Handles playlist file formats (JSON, M3U)
- **Queue controls**: Repeat modes, shuffle, track ordering

#### GUI Module (`src/gui/`)
- **`mod.rs`**: Module definition and public exports
- **Main application**: Core GUI application logic
- **View management**: Handles different application views
- **Event handling**: Processes user interactions
- **UI components**: Reusable interface elements

#### Configuration Module (`src/config/`)
- **`mod.rs`**: Module definition and public exports
- **Configuration loading**: Reads from files and environment variables
- **Settings management**: Handles user preferences
- **Validation**: Ensures configuration values are valid
- **Persistence**: Saves configuration changes

#### Utilities Module (`src/utils/`)
- **`mod.rs`**: Module definition and public exports
- **Formatting functions**: Time, file size, duration formatting
- **File operations**: Path manipulation and file utilities
- **String utilities**: Text processing and manipulation
- **Time utilities**: Date/time formatting and parsing

## Documentation (`docs/`)

### User Documentation (`docs/user/`)
- **`README.md`**: Comprehensive user guide
- **Installation instructions**: Setup and configuration
- **Usage tutorials**: How to use the application
- **Troubleshooting**: Common issues and solutions

### Developer Documentation (`docs/developer/`)
- **`README.md`**: Developer guide and contribution guidelines
- **`CONTRIBUTING.md`**: Detailed contribution process
- **`PROJECT_STRUCTURE.md`**: This document
- **`CHANGELOG.md`**: Version history and changes
- **`PROJECT_STATUS.md`**: Current development status

### API Reference (`docs/api/`)
- **`README.md`**: Complete API documentation
- **Type definitions**: All public types and structures
- **Function signatures**: Method documentation
- **Code examples**: Usage examples and patterns

### Examples (`docs/examples/`)
- **`config.toml`**: Sample configuration file
- **Code samples**: Example implementations
- **Configuration templates**: Pre-configured setups

## Test Files (`tests/`)

### Unit Tests (`tests/unit/`)
- **`audio/`**: Audio module unit tests
- **`library/`**: Library module unit tests
- **`playlist/`**: Playlist module unit tests
- **`config/`**: Configuration module unit tests
- **`utils/`**: Utility function unit tests

### Integration Tests (`tests/integration/`)
- **End-to-end workflows**: Complete user scenarios
- **Module interactions**: Cross-module functionality
- **Performance tests**: Load and stress testing

### Benchmarks (`tests/benchmarks/`)
- **Performance benchmarks**: Speed and memory usage
- **Load testing**: Large library performance
- **Audio processing**: Decoding and playback performance

## Scripts (`scripts/`)

### Build Scripts (`scripts/build/`)
- **`build.sh`**: Main build automation script
- **Cross-platform builds**: Linux, macOS, Windows
- **Release builds**: Optimized production builds

### Deployment Scripts (`scripts/deploy/`)
- **Package creation**: Distribution packages
- **Installation scripts**: System installation
- **Update mechanisms**: Version update handling

### Maintenance Scripts (`scripts/maintenance/`)
- **`setup-dev.sh`**: Development environment setup
- **Code quality**: Formatting and linting
- **Dependency management**: Update and audit

## Assets (`assets/`)

### Icons (`assets/icons/`)
- **Application icons**: App launcher icons
- **Interface icons**: UI element icons
- **File type icons**: Audio format icons

### Images (`assets/images/`)
- **Screenshots**: Application screenshots
- **Graphics**: UI graphics and illustrations
- **Branding**: Logo and visual identity

### Themes (`assets/themes/`)
- **Light theme**: Bright color scheme
- **Dark theme**: Dark color scheme
- **Custom themes**: User-defined themes

## Examples (`examples/`)

### Basic Examples
- **Simple player**: Minimal audio player implementation
- **Library scanner**: Basic music library scanning
- **Playlist creator**: Simple playlist management

### Advanced Examples
- **Custom GUI**: Extended interface implementations
- **Audio effects**: Audio processing examples
- **Integration**: Third-party service integration

## Tools (`tools/`)

### Development Tools
- **Code generators**: Template and boilerplate generation
- **Analysis tools**: Code quality and performance analysis
- **Testing utilities**: Test data and mock generators

### Build Tools
- **Cross-compilation**: Multi-platform builds
- **Package creation**: Distribution package tools
- **Installation**: System integration tools

## GitHub Integration (`.github/`)

### Workflows (`.github/workflows/`)
- **`ci.yml`**: Continuous integration pipeline
- **Automated testing**: Multi-platform testing
- **Build verification**: Compilation and packaging
- **Quality checks**: Code formatting and linting

### Templates
- **Issue templates**: Bug report and feature request forms
- **PR templates**: Pull request guidelines
- **Release templates**: Version release automation

## Configuration Files

### Cargo Configuration
- **`Cargo.toml`**: Project metadata and dependencies
- **`Cargo.lock`**: Exact dependency versions
- **Build profiles**: Debug, release, and custom configurations

### Build Automation
- **`Makefile`**: Common development tasks
- **Build scripts**: Platform-specific build logic
- **CI/CD**: Automated build and test processes

## File Organization Principles

### Separation of Concerns
- **Core logic**: Separated from UI and configuration
- **Module boundaries**: Clear interfaces between components
- **Dependency direction**: Core modules don't depend on UI

### Scalability
- **Modular design**: Easy to add new features
- **Plugin architecture**: Extensible functionality
- **Configuration-driven**: Behavior controlled by settings

### Maintainability
- **Consistent patterns**: Similar structures across modules
- **Documentation**: Comprehensive inline and external docs
- **Testing**: Thorough test coverage for all components

## Development Workflow

### Code Organization
1. **Feature development**: Create feature branches
2. **Module implementation**: Implement in appropriate module
3. **Testing**: Add unit and integration tests
4. **Documentation**: Update relevant documentation
5. **Code review**: Submit for peer review

### File Naming Conventions
- **Rust files**: `snake_case.rs`
- **Directories**: `snake_case/`
- **Configuration**: `kebab-case.toml`
- **Scripts**: `kebab-case.sh`

### Import Organization
```rust
// Standard library imports
use std::path::{Path, PathBuf};

// Third-party imports
use anyhow::Result;
use serde::{Deserialize, Serialize};

// Local imports
use crate::audio::AudioPlayer;
use crate::library::Library;
```

## Future Structure Considerations

### Planned Additions
- **Plugin system**: Extensible functionality
- **Web interface**: Remote control capabilities
- **Mobile app**: Companion mobile application
- **Cloud sync**: Multi-device synchronization

### Scalability Improvements
- **Database backend**: Persistent storage optimization
- **Caching system**: Performance improvements
- **API server**: External integration capabilities
- **Microservices**: Distributed architecture

---

This structure provides a solid foundation for the Hexendrum project, ensuring maintainability, scalability, and ease of development. As the project grows, this structure can be extended while maintaining the established patterns and principles.
