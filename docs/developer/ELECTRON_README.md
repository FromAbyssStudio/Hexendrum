# Hexendrum Electron Interface

This document explains how to set up and use the new Electron-based user interface for Hexendrum, which replaces the previous egui-based interface with a modern, web-based UI.

## Overview

The new interface uses:
- **Electron** - Cross-platform desktop application framework
- **React** - Modern JavaScript UI library
- **Styled Components** - CSS-in-JS styling solution
- **Lucide React** - Beautiful icon library
- **Framer Motion** - Animation library

## Features

- 🎨 Modern, responsive design with dark theme
- 🎵 Beautiful music player interface
- 📱 Responsive layout that works on all screen sizes
- ⌨️ Keyboard shortcuts for playback control
- 🎛️ Advanced audio controls and queue management
- 🔍 Powerful search and filtering capabilities
- 📚 Library management with grid and list views
- 🎼 Playlist creation and management
- ⚙️ Comprehensive settings panel

## Prerequisites

Before setting up the Electron interface, ensure you have:

- **Node.js** (v16 or higher)
- **npm** (comes with Node.js)
- **Rust** (for the backend)

## Installation

1. **Install Node.js dependencies:**
   ```bash
   npm install
   ```

2. **Build the Rust backend:**
   ```bash
   cargo build --release
   ```

3. **Start the development server:**
   ```bash
   # This runs both Rust backend and Electron frontend together
   npm run dev
   ```
   
   This command starts:
   - **Rust backend** (`cargo run`)
   - **Webpack dev server** on `http://localhost:4000`
   - **Electron frontend** (connects to webpack dev server)
   
   The backend and frontend communicate through Electron's IPC system.

## Development

### Project Structure

```
renderer/                 # React frontend
├── components/          # Reusable UI components
├── pages/              # Page components
├── contexts/           # React contexts (AudioContext)
├── styles/             # CSS files
└── index.js            # Main entry point

main.js                  # Electron main process
preload.js              # Secure bridge between processes
package.json             # Node.js dependencies and scripts
```

### Available Scripts

- `npm start` - Start the production Electron app (frontend only)
- `npm run dev` - Start both Rust backend and Electron frontend in development mode with hot reload
- `npm run dev:backend` - Start only the Rust backend
- `npm run dev:frontend` - Start only the Electron frontend (requires backend running separately)
- `npm run build` - Build both Rust backend and Electron app for production
- `npm run dist` - Create distributable packages

### Key Components

#### AudioContext
Manages global audio state including:
- Current track and playback state
- Volume control
- Queue management
- Repeat and shuffle modes
- Library and playlist data

#### Sidebar
Provides navigation between app sections:
- Library
- Playlists
- Now Playing
- Settings
- Quick actions and library stats

#### Player
Bottom player bar with:
- Track information display
- Playback controls (play/pause, next/previous)
- Progress bar with seeking
- Volume control
- Queue management
- Repeat and shuffle toggles

#### Library Page
Main music library interface:
- Grid and list view modes
- Advanced search and filtering
- Track selection and bulk actions
- Sort by title, artist, album, or duration
- Genre filtering

## Keyboard Shortcuts

- **Space** - Play/Pause
- **Left Arrow** - Previous track
- **Right Arrow** - Next track
- **Up Arrow** - Volume up
- **Down Arrow** - Volume down
- **Ctrl/Cmd + O** - Open music folder
- **Ctrl/Cmd + Q** - Quit application

## Building for Distribution

### Linux
```bash
npm run dist
# Creates AppImage in dist/ folder
```

### macOS
```bash
npm run dist
# Creates .dmg file in dist/ folder
```

### Windows
```bash
npm run dist
# Creates .exe installer in dist/ folder
```

## Architecture

### Process Communication
- **Main Process** (main.js) - Manages app lifecycle and Rust backend
- **Renderer Process** (React app) - Handles UI and user interactions
- **Preload Script** (preload.js) - Secure bridge between processes
- **Rust Backend** - Audio processing and file management

### Data Flow
1. User interacts with React UI
2. React calls Electron API through preload script
3. Main process communicates with Rust backend
4. Rust processes audio and returns results
5. Results flow back through the chain to update UI

## Customization

### Themes
The app uses CSS custom properties for theming. Modify `renderer/styles/global.css` to change:
- Color palette
- Spacing and sizing
- Typography
- Shadows and borders
- Animations

### Styling
- **Global styles** - `renderer/styles/global.css`
- **Component styles** - Individual CSS files for each component
- **Utility classes** - Pre-built CSS classes for common patterns

### Adding New Features
1. Create new React components in `renderer/components/`
2. Add new pages in `renderer/pages/`
3. Update routing in `renderer/App.js`
4. Add new IPC handlers in `main.js` and `preload.js`
5. Extend AudioContext for new state management

## Troubleshooting

### Common Issues

**App won't start:**
- Ensure Rust backend is built (`cargo build --release`)
- Check Node.js version compatibility
- Verify all dependencies are installed

**Audio not working:**
- Check system audio settings
- Ensure Rust backend has proper permissions
- Verify audio file paths are correct

**UI not responsive:**
- Check browser console for JavaScript errors
- Verify React components are properly imported
- Check CSS custom properties are defined

### Debug Mode
Run with `npm run dev` to enable:
- Developer tools
- Hot reload
- Console logging
- Error reporting

## Performance

### Optimization Tips
- Use React.memo for expensive components
- Implement virtual scrolling for large lists
- Lazy load page components
- Optimize images and artwork
- Use CSS transforms for animations

### Memory Management
- Clean up event listeners
- Dispose of audio resources
- Limit queue size
- Implement proper cleanup in useEffect hooks

## Contributing

When contributing to the Electron interface:

1. Follow React best practices
2. Use TypeScript for new components (optional but recommended)
3. Maintain consistent styling with CSS custom properties
4. Add proper error handling and loading states
5. Test on multiple platforms
6. Update documentation for new features

## License

This interface is part of Hexendrum and follows the same MIT license as the main project.

---

For more information about the Rust backend, see the main project README.
