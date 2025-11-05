# Hexendrum Developer Guide 🦀

Welcome, developers! This guide will help you understand the codebase and contribute to Hexendrum.

## Table of Contents

- [Getting Started](#getting-started)
- [Project Architecture](#project-architecture)
- [Development Setup](#development-setup)
- [Code Organization](#code-organization)
- [Testing](#testing)
- [Building](#building)
- [Contributing](#contributing)
- [Code Style](#code-style)
- [Debugging](#debugging)

## Getting Started

### Prerequisites

- **Rust 1.70+**: [Install Rust](https://rustup.rs/)
- **Git**: Version control system
- **Basic Rust Knowledge**: Understanding of Rust syntax and concepts
- **Audio Concepts**: Basic understanding of audio formats and playback

### Quick Start

1. **Fork and Clone**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/Hexendrum.git
   cd Hexendrum
   ```

2. **Setup Development Environment**:
   ```bash
   make setup-dev
   ```

3. **Build and Test**:
   ```bash
   make build
   make test
   ```

4. **Run the Application**:
   ```bash
   # Run backend only
   make run
   
   # Run both frontend and backend together (recommended for full development)
   npm run dev
   # OR
   make run-full
   ```
   
   **Note**: The `npm run dev` command starts both the Rust backend and Electron frontend simultaneously,
   which is recommended for full-stack development. The frontend will connect to the backend through
   Electron's IPC system.

## Project Architecture

### High-Level Overview

Hexendrum follows a modular, layered architecture:

```
┌─────────────────────────────────────────────────────────────┐
│                        GUI Layer                            │
├─────────────────────────────────────────────────────────────┤
│                    Business Logic Layer                     │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐  │
│  │   Library   │ │  Playlist   │ │    Playback Queue   │  │
│  │  Management │ │  Management │ │                     │  │
│  └─────────────┘ └─────────────┘ └─────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                     Audio Layer                             │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐  │
│  │   Decoder   │ │   Player    │ │    Device Manager   │  │
│  └─────────────┘ └─────────────┘ └─────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                    System Layer                             │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐  │
│  │   Config    │ │    Utils    │ │    File System      │  │
│  └─────────────┘ └─────────────┘ └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Module Responsibilities

#### Core Modules

- **`audio/`**: Audio playback, format detection, device management
- **`library/`**: Music library scanning, metadata extraction, search
- **`playlist/`**: Playlist creation, management, playback queue
- **`gui/`**: User interface, view management, event handling
- **`config/`**: Configuration management, user preferences
- **`utils/`**: Common utilities, formatting, file operations

#### Cross-Cutting Concerns

- **Error Handling**: Consistent error types and handling patterns
- **Logging**: Structured logging with tracing
- **Configuration**: Environment-based configuration with defaults
- **Testing**: Unit tests, integration tests, and benchmarks

## Development Setup

### Environment Setup

1. **Install Rust Tools**:
   ```bash
   rustup component add rustfmt
   rustup component add clippy
   rustup component add rust-analyzer
   ```

2. **Install Development Dependencies**:
   ```bash
   # For audio development on Linux
   # Debian/Ubuntu:
   sudo apt-get install libasound2-dev pkg-config
   
   # Arch Linux/SteamOS:
   sudo pacman -S alsa-lib pkg-config
   
   # macOS:
   brew install pkg-config
   ```
   
   **Note**: The `alsa-sys` crate (dependency of `rodio`) requires ALSA development libraries
   to be installed on Linux systems. Without these, the build will fail with a pkg-config error.

3. **Setup IDE**:
   - **VS Code**: Install rust-analyzer extension
   - **IntelliJ**: Install Rust plugin
   - **Vim/Emacs**: Configure rust-analyzer

### Configuration

1. **Create Config Directory**:
   ```bash
   mkdir -p ~/.config/hexendrum
   ```

2. **Sample Configuration**:
   ```bash
   cp docs/examples/config.toml ~/.config/hexendrum/
   ```

3. **Environment Variables**:
   ```bash
   export HEXENDRUM_LOG_LEVEL=debug
   export HEXENDRUM_CONFIG_PATH=~/.config/hexendrum
   ```

## Code Organization

### Source Structure

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library root and exports
├── audio/               # Audio playback module
│   ├── mod.rs          # Module definition
│   ├── player.rs       # Audio player implementation
│   ├── decoder.rs      # Audio format decoding
│   └── device.rs       # Audio device management
├── library/             # Music library management
│   ├── mod.rs          # Module definition
│   ├── scanner.rs      # Directory scanning
│   ├── metadata.rs     # Metadata extraction
│   └── search.rs       # Search and filtering
├── playlist/            # Playlist system
│   ├── mod.rs          # Module definition
│   ├── manager.rs      # Playlist management
│   ├── queue.rs        # Playback queue
│   └── import.rs       # Import/export functionality
├── gui/                 # User interface
│   ├── mod.rs          # Module definition
│   ├── app.rs          # Main application
│   ├── views/          # Different view implementations
│   └── widgets/        # Reusable UI components
├── config/              # Configuration management
│   ├── mod.rs          # Module definition
│   ├── loader.rs       # Configuration loading
│   └── validator.rs    # Configuration validation
└── utils/               # Utility functions
    ├── mod.rs          # Module definition
    ├── format.rs       # Formatting utilities
    ├── file.rs         # File operations
    └── time.rs         # Time utilities
```

### Module Patterns

#### Module Definition

```rust
// src/audio/mod.rs
pub mod player;
pub mod decoder;
pub mod device;

pub use player::AudioPlayer;
pub use decoder::AudioDecoder;
pub use device::AudioDevice;

// Re-export commonly used types
pub type AudioResult<T> = Result<T, AudioError>;
```

#### Public API Design

```rust
// Keep public APIs minimal and focused
pub struct AudioPlayer {
    // Private fields
}

impl AudioPlayer {
    // Public methods only
    pub fn new() -> Result<Self, AudioError> { /* ... */ }
    pub fn play(&mut self, path: &Path) -> Result<(), AudioError> { /* ... */ }
    pub fn pause(&mut self) { /* ... */ }
}
```

## Testing

### Test Organization

```
tests/
├── unit/                # Unit tests for individual modules
│   ├── audio/          # Audio module tests
│   ├── library/        # Library module tests
│   └── playlist/       # Playlist module tests
├── integration/         # Integration tests
│   ├── audio_playback.rs
│   ├── library_scanning.rs
│   └── playlist_workflow.rs
└── benchmarks/          # Performance benchmarks
    ├── audio_bench.rs
    └── search_bench.rs
```

### Writing Tests

#### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_player_creation() {
        let player = AudioPlayer::new();
        assert!(player.is_ok());
    }

    #[test]
    fn test_volume_control() {
        let mut player = AudioPlayer::new().unwrap();
        player.set_volume(0.5);
        assert_eq!(player.get_volume(), 0.5);
    }

    #[test]
    fn test_invalid_volume() {
        let mut player = AudioPlayer::new().unwrap();
        player.set_volume(2.0); // Should clamp to 1.0
        assert_eq!(player.get_volume(), 1.0);
    }
}
```

#### Integration Tests

```rust
// tests/integration/audio_playback.rs
use hexendrum::audio::AudioPlayer;
use hexendrum::library::Library;
use std::path::Path;

#[test]
fn test_end_to_end_playback() {
    // Setup
    let library = Library::new();
    let player = AudioPlayer::new().unwrap();
    
    // Test audio file playback
    let test_file = Path::new("tests/fixtures/test.mp3");
    assert!(test_file.exists());
    
    // Play the file
    let result = player.play(test_file);
    assert!(result.is_ok());
    
    // Verify playback state
    assert!(player.is_playing());
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_audio_player_creation

# Run with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test integration

# Run benchmarks
cargo bench
```

## Building

### Build Configurations

```bash
# Debug build (development)
cargo build

# Release build (optimized)
cargo build --release

# Profile build (performance analysis)
cargo build --profile=release

# Cross-compilation
cargo build --target x86_64-unknown-linux-gnu
```

### Build Scripts

```bash
# Using Makefile
make build          # Debug build
make release        # Release build
make clean          # Clean artifacts

# Using build scripts
./scripts/build/build.sh
```

### Dependencies

#### Core Dependencies

- **`rodio`**: Cross-platform audio playback
- **`symphonia`**: Audio format decoding
- **`egui`**: Immediate mode GUI framework
- **`serde`**: Serialization/deserialization
- **`tokio`**: Async runtime

#### Development Dependencies

- **`tokio-test`**: Async testing utilities
- **`criterion`**: Benchmarking framework
- **`mockall`**: Mocking framework

## Contributing

### Development Workflow

1. **Create Feature Branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make Changes**:
   - Write code following style guidelines
   - Add tests for new functionality
   - Update documentation

3. **Quality Checks**:
   ```bash
   make format      # Format code
   make lint        # Run clippy
   make test        # Run tests
   make build       # Ensure compilation
   ```

4. **Commit Changes**:
   ```bash
   git add .
   git commit -m "feat(audio): add FLAC format support"
   ```

5. **Push and Create PR**:
   ```bash
   git push origin feature/your-feature-name
   # Create PR on GitHub
   ```

### Code Review Process

1. **Self-Review**: Review your own code before submitting
2. **Peer Review**: Request review from maintainers
3. **Address Feedback**: Make requested changes
4. **Final Review**: Ensure all feedback is addressed
5. **Merge**: Maintainer merges the PR

## Code Style

### Rust Conventions

- Follow [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- Use `rustfmt` for consistent formatting
- Run `cargo clippy` to catch common issues

### Naming Conventions

```rust
// Structs and enums: PascalCase
pub struct AudioPlayer { }
pub enum AudioState { }

// Functions and variables: snake_case
pub fn play_audio() { }
let audio_buffer = Vec::new();

// Constants: SCREAMING_SNAKE_CASE
pub const DEFAULT_VOLUME: f32 = 0.7;
pub const MAX_BUFFER_SIZE: usize = 8192;

// Types: PascalCase
pub type AudioResult<T> = Result<T, AudioError>;
```

### Error Handling

```rust
// Use anyhow for application-level errors
use anyhow::{Result, Context};

pub fn load_audio_file(path: &Path) -> Result<AudioFile> {
    let file = File::open(path)
        .with_context(|| format!("Failed to open audio file: {}", path.display()))?;
    
    // ... rest of function
}

// Use specific error types for library APIs
#[derive(Debug, thiserror::Error)]
pub enum AudioError {
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

### Documentation

```rust
/// Audio player for handling music playback.
///
/// This struct provides a high-level interface for audio playback,
/// including play, pause, stop, and volume control functionality.
///
/// # Examples
///
/// ```rust
/// use hexendrum::audio::AudioPlayer;
///
/// let player = AudioPlayer::new()?;
/// player.play("song.mp3")?;
/// ```
pub struct AudioPlayer {
    // ... fields
}

impl AudioPlayer {
    /// Creates a new audio player instance.
    ///
    /// # Returns
    ///
    /// Returns `Ok(AudioPlayer)` on success, or `Err(AudioError)` if
    /// the audio system cannot be initialized.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The audio system is not available
    /// - Required audio drivers are missing
    /// - Audio device initialization fails
    pub fn new() -> Result<Self, AudioError> {
        // ... implementation
    }
}
```

## Debugging

### Logging

```rust
use tracing::{info, warn, error, debug};

// Set log level
RUST_LOG=debug cargo run

// In code
debug!("Audio buffer size: {}", buffer_size);
info!("Starting audio playback");
warn!("Low audio buffer, performance may be affected");
error!("Failed to decode audio file: {}", e);
```

### Debug Builds

```bash
# Debug build with symbols
cargo build

# Run with debugger
gdb target/debug/hexendrum

# Use rust-gdb for better Rust support
rust-gdb target/debug/hexendrum
```

### Performance Profiling

```bash
# Install profiling tools
cargo install flamegraph

# Generate flamegraph
cargo flamegraph

# Use perf (Linux)
perf record --call-graph=dwarf cargo run
perf report
```

### Common Debugging Scenarios

1. **Audio Issues**: Check system audio, device selection
2. **GUI Problems**: Verify egui compatibility, graphics drivers
3. **Performance**: Profile with criterion, check memory usage
4. **Build Errors**: Verify dependencies, Rust version

## Next Steps

### Learning Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust Reference](https://doc.rust-lang.org/reference/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [egui Documentation](https://docs.rs/egui/)
- [rodio Documentation](https://docs.rs/rodio/)

### Getting Help

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and planning
- **Code Review**: Learn from PR reviews
- **Community**: Join our Discord server (coming soon)

---

**Ready to contribute?** Check our [Contributing Guide](CONTRIBUTING.md) for detailed guidelines!
