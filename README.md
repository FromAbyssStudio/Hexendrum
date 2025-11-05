# Hexendrum 🎵

A modern, open-source music player built with Electron and React, featuring a beautiful and intuitive interface.

[![CI/CD](https://github.com/fromabyssstudio/Hexendrum/workflows/CI%2FCD%20Pipeline/badge.svg)](https://github.com/fromabyssstudio/Hexendrum/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Node.js Version](https://img.shields.io/badge/node-18+-green.svg)](https://nodejs.org/)
[![Electron Version](https://img.shields.io/badge/electron-28+-blue.svg)](https://www.electronjs.org/)

## 🎵 Quick Start

### Prerequisites
- Node.js 18+ ([Install Node.js](https://nodejs.org/))
- npm or yarn package manager
- **Linux users**: ALSA development libraries
  - Debian/Ubuntu: `sudo apt-get install libasound2-dev pkg-config`
  - Arch Linux/SteamOS: `sudo pacman -S alsa-lib pkg-config`
- **Rust 1.70+** (for backend development): [Install Rust](https://rustup.rs/)

### Build and Run

#### Running Frontend + Backend Together (Development)

```bash
# Clone the repository
git clone https://github.com/fromabyssstudio/Hexendrum.git
cd Hexendrum

# Install dependencies
npm install

# Run both frontend and backend together
npm run dev
# OR using Makefile:
make run-full
```

This will start:
- **Rust backend** (cargo run)
- **Webpack dev server** (http://localhost:4000)
- **Electron frontend** (connects to webpack dev server)

#### Running Backend Only

```bash
# Run Rust backend only
cargo run
# Optional: include the terminal playbar
cargo run -- --cli-playbar
# OR using Makefile:
make run
```

#### Running Frontend Only (without backend)

```bash
# Run only the Electron frontend (requires webpack server)
npm run dev:frontend
```

#### Production Build

```bash
# Build everything for production
npm run build
npm start
```

## Features

- **Multi-format Audio Support**: MP3, FLAC, OGG, WAV, M4A, AAC
- **Music Library Management**: Scan and organize your music collection
- **Playlist Support**: Create, edit, and manage playlists
- **Smart Search**: Search through your library by title, artist, or album
- **Advanced Playback Controls**: Play, pause, skip, volume control, queue management
- **Realtime Updates**: Playback state, volume, and scan progress via WebSocket
- **Modern GUI**: Clean, intuitive interface built with React and Electron
- **Metadata Aware**: Uses embedded tags (via Lofty) for album art, duration, and artist info
- **CLI Playbar (optional)**: Follow playback directly in the terminal with `--cli-playbar`
- **Cross-platform**: Works on Windows, macOS, and Linux
- **Configurable**: Customize audio settings, library paths, and more
- **Fast & Efficient**: Built with modern web technologies for performance

## Documentation

- **[User Guide](docs/user/README.md)** - How to use Hexendrum
- **[Developer Guide](docs/developer/README.md)** - Contributing and development
- **[API Reference](docs/api/README.md)** - Code documentation
- **[Swagger/OpenAPI Docs](docs/api/SWAGGER.md)** - Interactive API documentation (available at `http://127.0.0.1:3030/swagger-ui`)
- **[Changelog](docs/developer/CHANGELOG.md)** - Version history
- **[Project Status](docs/developer/PROJECT_STATUS.md)** - Current development status

## Development

### Setup Development Environment
```bash
npm install
```

### Common Commands
```bash
npm run dev          # Start development server
npm run build        # Build for production
npm run start        # Start production build
npm run dist         # Create distributable packages
```

### Project Structure
```
Hexendrum/
├── renderer/              # React frontend application
│   ├── components/        # Reusable UI components
│   ├── pages/            # Application pages
│   ├── contexts/         # React contexts
│   └── styles/           # CSS and styling
├── main.js               # Electron main process
├── preload.js            # Electron preload script
├── webpack.config.js     # Webpack configuration
├── package.json          # Node.js dependencies and scripts
├── docs/                 # Documentation
│   ├── user/             # User documentation
│   ├── developer/        # Developer documentation
│   └── api/              # API reference
├── assets/               # Icons, images, themes
└── scripts/              # Build and maintenance scripts
```

## Contributing

We welcome contributions! Please see our [Contributing Guide](docs/developer/CONTRIBUTING.md) for details.

### Quick Contribution
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- **egui**: Modern, fast GUI framework
- **rodio**: Cross-platform audio playback
- **symphonia**: Audio format decoding
- **Rust Community**: Excellent ecosystem and tools

## Support

- **Issues**: [GitHub Issues](https://github.com/fromabyssstudio/Hexendrum/issues)
- **Discussions**: [GitHub Discussions](https://github.com/fromabyssstudio/Hexendrum/discussions)
- **Wiki**: [Project Wiki](https://github.com/fromabyssstudio/Hexendrum/wiki)

---

**Made with 🦀 by From Abyss Studio**

*Hexendrum - Where music meets innovation*
