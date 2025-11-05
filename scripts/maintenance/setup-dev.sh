#!/bin/bash

# Hexendrum Development Setup Script
echo "🎵 Setting up Hexendrum development environment..."

# Create necessary directories
echo "📁 Creating development directories..."
mkdir -p ~/.config/hexendrum
mkdir -p ~/.config/hexendrum/playlists
mkdir -p ~/.local/share/hexendrum
mkdir -p ~/.cache/hexendrum

# Install development tools
echo "🔧 Installing development tools..."
if command -v rustup &> /dev/null; then
    rustup component add rustfmt
    rustup component add clippy
    echo "✅ Rust tools installed"
else
    echo "⚠️  rustup not found, skipping Rust tool installation"
fi

# Check for ALSA development libraries (required for Linux builds)
echo "🔍 Checking for system dependencies..."
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if ! pkg-config --exists alsa 2>/dev/null; then
        echo "⚠️  ALSA development libraries not found!"
        echo "   This is required for building on Linux."
        echo ""
        if command -v pacman &> /dev/null; then
            echo "   To install on Arch Linux/SteamOS:"
            echo "   sudo pacman -S alsa-lib pkg-config"
        elif command -v apt-get &> /dev/null; then
            echo "   To install on Debian/Ubuntu:"
            echo "   sudo apt-get install libasound2-dev pkg-config"
        else
            echo "   Please install ALSA development libraries for your distribution."
        fi
        echo ""
    else
        echo "✅ ALSA development libraries found"
    fi
fi

# Create sample configuration
echo "⚙️  Creating sample configuration..."
cat > ~/.config/hexendrum/config.toml << EOF
# Hexendrum Configuration File

[audio]
default_volume = 0.7
sample_rate = 44100
buffer_size = 4096

[library]
auto_scan = true
scan_interval = 300

[gui]
theme = "auto"
show_file_extensions = false

[playlist]
auto_save = true
max_history = 100
EOF

echo "✅ Development environment setup complete!"
echo ""
echo "📁 Configuration directory: ~/.config/hexendrum/"
echo "🎵 Playlist directory: ~/.config/hexendrum/playlists/"
echo "💾 Cache directory: ~/.cache/hexendrum/"
echo ""
echo "🚀 Next steps:"
echo "   1. Add your music directories to the config"
echo "   2. Run 'cargo build' to build the project"
echo "   3. Run 'cargo run' to start the application"
