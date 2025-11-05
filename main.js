const { app, BrowserWindow, ipcMain, dialog, Menu } = require('electron');
const path = require('path');
const https = require('https');
const http = require('http');
const isDev = process.argv.includes('--dev');
const ConfigManager = require('./config-manager');

// Rust backend API configuration
const API_ORIGIN = 'http://127.0.0.1:3030';
const API_BASE_URL = `${API_ORIGIN}/api`;

// Helper function to make HTTP requests to Rust backend
async function apiRequest(endpoint, options = {}) {
  const url = new URL(`${API_BASE_URL}${endpoint}`);
  
  const requestOptions = {
    hostname: url.hostname,
    port: url.port || 3030,
    path: url.pathname + url.search,
    method: options.method || 'GET',
    headers: {
      'Content-Type': 'application/json',
      ...options.headers,
    },
  };

  return new Promise((resolve, reject) => {
    const req = http.request(requestOptions, (res) => {
      let data = '';
      
      res.on('data', (chunk) => {
        data += chunk;
      });
      
      res.on('end', () => {
        try {
          if (res.statusCode >= 200 && res.statusCode < 300) {
            const jsonData = JSON.parse(data);
            resolve(jsonData);
          } else {
            reject(new Error(`HTTP ${res.statusCode}: ${data}`));
          }
        } catch (e) {
          reject(new Error(`Failed to parse response: ${e.message}`));
        }
      });
    });
    
    req.on('error', (error) => {
      reject(new Error(`API request failed: ${error.message}`));
    });
    
    // Handle request body
    if (options.body) {
      const bodyString = typeof options.body === 'string' 
        ? options.body 
        : JSON.stringify(options.body);
      req.write(bodyString);
    }
    
    req.end();
  });
}

function resolveArtworkUrl(artworkPath) {
  if (!artworkPath) {
    return null;
  }

  try {
    return new URL(artworkPath, API_ORIGIN).toString();
  } catch (error) {
    console.warn('Main process: Failed to resolve artwork URL:', artworkPath, error);
    return artworkPath;
  }
}

async function fetchAlbumSummaries(query) {
  try {
    const hasQuery = Boolean(query && query.trim().length > 0);
    const endpoint = hasQuery
      ? `/library/albums/search?q=${encodeURIComponent(query.trim())}`
      : '/library/albums/search';
    const response = await apiRequest(endpoint);

    if (response.success && Array.isArray(response.data)) {
      return response.data;
    }

    return [];
  } catch (error) {
    console.error('Main process: Failed to fetch album summaries:', error);
    return [];
  }
}

function buildArtworkIndex(albums) {
  const byId = new Map();
  const byPair = new Map();

  albums.forEach((album) => {
    const artworkUrl = resolveArtworkUrl(album.artwork_url);
    if (!artworkUrl) {
      return;
    }

    if (album.id) {
      byId.set(album.id, artworkUrl);
    }

    const albumTitle = typeof album.title === 'string' ? album.title.trim() : '';
    if (!albumTitle) {
      return;
    }

    const normalizedAlbum = albumTitle.toLowerCase();
    const artists = new Set();

    if (album.primary_artist) {
      artists.add(album.primary_artist);
    }

    if (Array.isArray(album.artists)) {
      album.artists
        .filter(Boolean)
        .forEach((artist) => artists.add(artist));
    }

    if (artists.size === 0) {
      const key = `${normalizedAlbum}|`;
      if (!byPair.has(key)) {
        byPair.set(key, artworkUrl);
      }
    } else {
      artists.forEach((artist) => {
        const normalizedArtist = artist.trim().toLowerCase();
        const key = `${normalizedAlbum}|${normalizedArtist}`;
        if (!byPair.has(key)) {
          byPair.set(key, artworkUrl);
        }
      });
    }

    const fallbackKey = `${normalizedAlbum}|`;
    if (!byPair.has(fallbackKey)) {
      byPair.set(fallbackKey, artworkUrl);
    }
  });

  return { byId, byPair };
}

function transformTrack(track, artworkIndex) {
  const albumId = track.album_id || null;
  const normalizedAlbum = (track.album || '').trim().toLowerCase();
  const normalizedArtist = (track.artist || '').trim().toLowerCase();

  let artwork = null;

  if (artworkIndex) {
    if (albumId && artworkIndex.byId.has(albumId)) {
      artwork = artworkIndex.byId.get(albumId);
    }

    if (!artwork && normalizedAlbum) {
      const artistKey = `${normalizedAlbum}|${normalizedArtist}`;
      if (normalizedArtist && artworkIndex.byPair.has(artistKey)) {
        artwork = artworkIndex.byPair.get(artistKey);
      }

      if (!artwork) {
        const fallbackKey = `${normalizedAlbum}|`;
        if (artworkIndex.byPair.has(fallbackKey)) {
          artwork = artworkIndex.byPair.get(fallbackKey);
        }
      }
    }
  }

  return {
    id: track.id,
    title: track.title || 'Unknown Title',
    artist: track.artist || 'Unknown Artist',
    album: track.album || 'Unknown Album',
    albumId,
    genre: track.genre || 'Unknown',
    duration: track.duration || 0,
    path: track.path,
    file_size: track.file_size,
    artwork,
  };
}

async function fetchTracksWithArtwork({ query } = {}) {
  const hasQuery = Boolean(query && query.trim().length > 0);
  const endpoint = hasQuery
    ? `/library/search?q=${encodeURIComponent(query.trim())}`
    : '/library/tracks';

  try {
    const [tracksResponse, albumSummaries] = await Promise.all([
      apiRequest(endpoint),
      fetchAlbumSummaries(query),
    ]);

    if (!tracksResponse.success || !Array.isArray(tracksResponse.data)) {
      return {
        success: false,
        error: tracksResponse.error || 'Failed to retrieve tracks from backend',
        tracks: [],
      };
    }

    const artworkIndex = buildArtworkIndex(albumSummaries);
    const albumDetailMap = new Map();

    albumSummaries.forEach((album) => {
      if (album && album.id) {
        albumDetailMap.set(album.id, {
          isManual: Boolean(album.is_manual),
          metadata: album.metadata || null,
        });
      }
    });

    const tracks = tracksResponse.data.map((track) => {
      const transformed = transformTrack(track, artworkIndex);
      if (transformed.albumId && albumDetailMap.has(transformed.albumId)) {
        const details = albumDetailMap.get(transformed.albumId);
        transformed.albumIsManual = details.isManual;
        transformed.albumMetadata = details.metadata;
      } else {
        transformed.albumIsManual = false;
        transformed.albumMetadata = null;
      }
      return transformed;
    });

    return { success: true, tracks };
  } catch (error) {
    console.error('Main process: Failed to fetch tracks with artwork:', error);
    return {
      success: false,
      error: error.message,
      tracks: [],
    };
  }
}

async function fetchAlbumOverrideRecord(albumId) {
  if (!albumId) {
    return { success: false, error: 'Album identifier is required' };
  }

  try {
    const response = await apiRequest(
      `/library/albums/${encodeURIComponent(albumId)}/manual`
    );

    if (response && typeof response.success === 'boolean') {
      return response;
    }

    return { success: false, error: 'Unexpected response from backend' };
  } catch (error) {
    if (typeof error.message === 'string' && error.message.startsWith('HTTP 404')) {
      return { success: true, data: null };
    }

    console.error('Main process: Failed to load album override:', error);
    return { success: false, error: error.message || 'Unknown error' };
  }
}

async function setAlbumOverrideRecord(albumId, payload) {
  if (!albumId) {
    return { success: false, error: 'Album identifier is required' };
  }

  try {
    const response = await apiRequest(
      `/library/albums/${encodeURIComponent(albumId)}/manual`,
      {
        method: 'PUT',
        body: payload,
      }
    );

    if (response && typeof response.success === 'boolean') {
      return response;
    }

    return { success: false, error: 'Unexpected response from backend' };
  } catch (error) {
    console.error('Main process: Failed to persist album override:', error);
    return { success: false, error: error.message || 'Failed to save album override' };
  }
}

// Enable GPU acceleration hints
app.commandLine.appendSwitch('enable-gpu-rasterization');
app.commandLine.appendSwitch('enable-zero-copy');
app.commandLine.appendSwitch('force_high_performance_gpu');
app.commandLine.appendSwitch('ignore-gpu-blocklist');
app.commandLine.appendSwitch('enable-gpu-compositing');

let mainWindow;
let configManager;

function createWindow() {
  // Initialize configuration manager
  configManager = new ConfigManager();
  
  // Create the browser window with improved UX
  const preloadPath = path.resolve(__dirname, 'preload.js');
  console.log('Main process: Preload script path:', preloadPath);
  console.log('Main process: __dirname:', __dirname);
  console.log('Main process: Preload file exists:', require('fs').existsSync(preloadPath));
  console.log('Main process: Current working directory:', process.cwd());
  
  mainWindow = new BrowserWindow({
    width: 1600,
    height: 1000,
    minWidth: 1200,
    minHeight: 800,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      enableRemoteModule: false,
      preload: preloadPath,
      spellcheck: false,
      webSecurity: true,
      backgroundThrottling: false,
      webgl: true
    },
    icon: path.join(__dirname, 'assets/icons/logo.png'),
    titleBarStyle: 'hiddenInset',
    titleBarOverlay: {
      color: '#0a0a0a',
      symbolColor: '#00ff88',
      height: 32
    },
    show: false,
    frame: false,
    autoHideMenuBar: true,
    resizable: true,
    maximizable: true,
    minimizable: true,
    fullscreenable: true,
    backgroundColor: '#0a0a0a'
  });

  mainWindow.setMenuBarVisibility(false);

  // Load the app
  if (isDev) {
    mainWindow.loadURL('http://localhost:4000');
    mainWindow.webContents.openDevTools();
  } else {
    mainWindow.loadFile(path.join(__dirname, 'renderer/index.html'));
  }

  // Enhanced window event handling for better UX
  mainWindow.once('ready-to-show', () => {
    // Smooth window appearance
    mainWindow.show();
    
    // Focus the window
    mainWindow.focus();
    
    // Optional: Show a welcome message or onboarding
    if (isDev) {
      console.log('Hexendrum is ready! 🎵');
    }
  });

  // Handle window state changes
  mainWindow.on('maximize', () => {
    console.log('Window maximized');
  });

  mainWindow.on('unmaximize', () => {
    console.log('Window unmaximized');
  });

  mainWindow.on('enter-full-screen', () => {
    console.log('Entered fullscreen mode');
  });

  mainWindow.on('leave-full-screen', () => {
    console.log('Left fullscreen mode');
  });

  // Handle window closed
  mainWindow.on('closed', () => {
    mainWindow = null;
  });

  // Handle window focus for better user experience
  mainWindow.on('focus', () => {
    // Resume any paused operations
    console.log('Window focused');
  });

  mainWindow.on('blur', () => {
    // Pause non-essential operations
    console.log('Window blurred');
  });

}



function createMenu() {
  const template = [
    {
      label: 'File',
      submenu: [
        {
          label: 'Open Music Folder',
          accelerator: 'CmdOrCtrl+O',
          click: async () => {
            const result = await dialog.showOpenDialog(mainWindow, {
              properties: ['openDirectory'],
              title: 'Select Music Folder'
            });
            
            if (!result.canceled && result.filePaths.length > 0) {
              mainWindow.webContents.send('music-folder-selected', result.filePaths[0]);
            }
          }
        },
        {
          label: 'Import Music Files',
          accelerator: 'CmdOrCtrl+I',
          click: async () => {
            const result = await dialog.showOpenDialog(mainWindow, {
              properties: ['openFile', 'multiSelections'],
              filters: [
                { name: 'Audio Files', extensions: ['mp3', 'flac', 'ogg', 'wav', 'aac', 'm4a'] }
              ],
              title: 'Select Music Files to Import'
            });
            
            if (!result.canceled && result.filePaths.length > 0) {
              mainWindow.webContents.send('music-files-imported', result.filePaths);
            }
          }
        },
        { type: 'separator' },
        {
          label: 'Export Playlist',
          accelerator: 'CmdOrCtrl+E',
          click: () => mainWindow.webContents.send('export-playlist')
        },
        { type: 'separator' },
        {
          label: 'Quit',
          accelerator: process.platform === 'darwin' ? 'Cmd+Q' : 'Ctrl+Q',
          click: () => app.quit()
        }
      ]
    },
    {
      label: 'View',
      submenu: [
        {
          label: 'Toggle Sidebar',
          accelerator: 'CmdOrCtrl+B',
          click: () => mainWindow.webContents.send('toggle-sidebar')
        },
        { type: 'separator' },
        { role: 'reload' },
        { role: 'forceReload' },
        { role: 'toggleDevTools' },
        { type: 'separator' },
        { role: 'resetZoom' },
        { role: 'zoomIn' },
        { role: 'zoomOut' },
        { type: 'separator' },
        { role: 'togglefullscreen' }
      ]
    },
    {
      label: 'Playback',
      submenu: [
        {
          label: 'Play/Pause',
          accelerator: 'Space',
          click: () => mainWindow.webContents.send('playback-toggle')
        },
        {
          label: 'Next Track',
          accelerator: 'Right',
          click: () => mainWindow.webContents.send('playback-next')
        },
        {
          label: 'Previous Track',
          accelerator: 'Left',
          click: () => mainWindow.webContents.send('playback-previous')
        },
        { type: 'separator' },
        {
          label: 'Stop',
          accelerator: 'S',
          click: () => mainWindow.webContents.send('playback-stop')
        },
        {
          label: 'Seek Forward',
          accelerator: 'Shift+Right',
          click: () => mainWindow.webContents.send('seek-forward')
        },
        {
          label: 'Seek Backward',
          accelerator: 'Shift+Left',
          click: () => mainWindow.webContents.send('seek-backward')
        },
        { type: 'separator' },
        {
          label: 'Volume Up',
          accelerator: 'Up',
          click: () => mainWindow.webContents.send('volume-up')
        },
        {
          label: 'Volume Down',
          accelerator: 'Down',
          click: () => mainWindow.webContents.send('volume-down')
        },
        {
          label: 'Mute/Unmute',
          accelerator: 'M',
          click: () => mainWindow.webContents.send('volume-mute')
        }
      ]
    },
    {
      label: 'Library',
      submenu: [
        {
          label: 'Scan for New Music',
          accelerator: 'CmdOrCtrl+R',
          click: () => mainWindow.webContents.send('scan-library')
        },
        {
          label: 'Refresh Library',
          accelerator: 'F5',
          click: () => mainWindow.webContents.send('refresh-library')
        },
        { type: 'separator' },
        {
          label: 'Library Statistics',
          accelerator: 'CmdOrCtrl+L',
          click: () => mainWindow.webContents.send('show-library-stats')
        }
      ]
    },
  ];

  if (process.platform === 'darwin') {
    template.unshift({
      label: app.getName(),
      submenu: [
        { role: 'about' },
        { type: 'separator' },
        { role: 'services' },
        { type: 'separator' },
        { role: 'hide' },
        { role: 'hideOthers' },
        { role: 'unhide' },
        { type: 'separator' },
        { role: 'quit' }
      ]
    });
  }

  const menu = Menu.buildFromTemplate(template);
  Menu.setApplicationMenu(menu);
}

// IPC handlers for communication with renderer
ipcMain.handle('get-app-version', () => {
  return app.getVersion();
});

ipcMain.handle('window-control', (event, action) => {
  const targetWindow = BrowserWindow.fromWebContents(event.sender);
  if (!targetWindow) {
    return { success: false, error: 'No window available' };
  }

  switch (action) {
    case 'minimize':
      targetWindow.minimize();
      break;
    case 'maximize':
      targetWindow.maximize();
      break;
    case 'unmaximize':
      targetWindow.unmaximize();
      break;
    case 'toggle-maximize':
      if (targetWindow.isMaximized()) {
        targetWindow.unmaximize();
      } else {
        targetWindow.maximize();
      }
      break;
    case 'close':
      targetWindow.close();
      break;
    default:
      return { success: false, error: `Unknown action: ${action}` };
  }

  return { success: true };
});

ipcMain.handle('select-music-folder', async () => {
  const result = await dialog.showOpenDialog(mainWindow, {
    properties: ['openDirectory'],
    title: 'Select Music Folder'
  });
  
  if (!result.canceled && result.filePaths.length > 0) {
    return result.filePaths[0];
  }
  return null;
});

ipcMain.handle('select-music-files', async () => {
  const result = await dialog.showOpenDialog(mainWindow, {
    properties: ['openFile', 'multiSelections'],
    filters: [
      { name: 'Audio Files', extensions: ['mp3', 'flac', 'ogg', 'wav', 'aac', 'm4a'] }
    ],
    title: 'Select Music Files'
  });
  
  if (!result.canceled && result.filePaths.length > 0) {
    return result.filePaths;
  }
  return [];
});

// Audio playback IPC handlers
ipcMain.handle('play-track', async (event, filePath) => {
  try {
    console.log('Main process: Playing track:', filePath);
    
    // Call Rust backend API to play track
    const response = await apiRequest('/audio/play', {
      method: 'POST',
      body: { file_path: filePath },
    });
    
    if (response.success) {
      console.log('Main process: Playback started successfully');
      return { success: true, message: 'Track playback started' };
    } else {
      console.error('Main process: Backend returned error:', response.error);
      return { success: false, error: response.error || 'Failed to start playback' };
    }
  } catch (error) {
    console.error('Main process: Error playing track:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('pause-track', async () => {
  try {
    console.log('Main process: Pausing track');
    
    const response = await apiRequest('/audio/pause', {
      method: 'POST',
    });
    
    if (response.success) {
      return { success: true, message: 'Track paused' };
    } else {
      return { success: false, error: response.error || 'Failed to pause' };
    }
  } catch (error) {
    console.error('Main process: Error pausing track:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('stop-track', async () => {
  try {
    console.log('Main process: Stopping track');
    
    const response = await apiRequest('/audio/stop', {
      method: 'POST',
    });
    
    if (response.success) {
      return { success: true, message: 'Track stopped' };
    } else {
      return { success: false, error: response.error || 'Failed to stop' };
    }
  } catch (error) {
    console.error('Main process: Error stopping track:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('pause-playback', async () => {
  try {
    console.log('Main process: Pausing playback');

    const response = await apiRequest('/audio/pause', {
      method: 'POST',
    });

    if (response.success) {
      return { success: true, message: 'Playback paused' };
    } else {
      return { success: false, error: response.error || 'Failed to pause' };
    }
  } catch (error) {
    console.error('Main process: Error pausing playback:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('set-volume', async (event, volume) => {
  try {
    console.log('Main process: Setting volume to:', volume);
    
    const response = await apiRequest('/audio/volume', {
      method: 'POST',
      body: { volume },
    });
    
    if (response.success) {
      return { success: true, message: 'Volume updated' };
    } else {
      return { success: false, error: response.error || 'Failed to set volume' };
    }
  } catch (error) {
    console.error('Main process: Error setting volume:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('get-audio-info', async (event, filePath) => {
  try {
    console.log('Getting audio info for:', filePath);
    // You can implement actual audio metadata extraction here
    // using libraries like 'music-metadata' or 'node-id3'
    return {
      success: true,
      info: {
        title: 'Sample Track',
        artist: 'Sample Artist',
        album: 'Sample Album',
        duration: 180,
        format: 'mp3'
      }
    };
  } catch (error) {
    console.error('Error getting audio info:', error);
    return { success: false, error: error.message };
  }
});

// Additional audio playback IPC handlers
ipcMain.handle('resume-playback', async () => {
  try {
    console.log('Main process: Resuming playback');
    
    const response = await apiRequest('/audio/resume', {
      method: 'POST',
    });
    
    if (response.success) {
      return { success: true, message: 'Playback resumed' };
    } else {
      return { success: false, error: response.error || 'Failed to resume' };
    }
  } catch (error) {
    console.error('Main process: Error resuming playback:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('next-track', async () => {
  try {
    console.log('Playing next track');
    return { success: true, message: 'Next track started' };
  } catch (error) {
    console.error('Error playing next track:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('previous-track', async () => {
  try {
    console.log('Playing previous track');
    return { success: true, message: 'Previous track started' };
  } catch (error) {
    console.error('Error playing previous track:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('scan-library', async (event, directories) => {
  try {
    console.log('Main process: scan-library called with directories:', directories);
    
    // Call Rust backend API to scan library
    const response = await apiRequest('/library/scan', {
      method: 'POST',
      body: { directories },
    });
    
    if (response.success) {
      const tracksResult = await fetchTracksWithArtwork();
      if (!tracksResult.success) {
        return {
          success: false,
          error: tracksResult.error,
          tracks: [],
          count: response.data || 0,
        };
      }

      return {
        success: true,
        tracks: tracksResult.tracks,
        count: response.data || 0,
      };
    }

    return { success: false, error: response.error || 'Unknown error' };
  } catch (error) {
    console.error('Main process: Error scanning library:', error);
    return { success: false, error: error.message };
  }
});

// Additional missing IPC handlers
ipcMain.handle('get-volume', async () => {
  try {
    console.log('Getting current volume');
    return { success: true, volume: 0.7 }; // Default volume
  } catch (error) {
    console.error('Error getting volume:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('get-library', async () => {
  try {
    console.log('Getting library from Rust backend...');

    const tracksResult = await fetchTracksWithArtwork();

    if (tracksResult.success) {
      return {
        success: true,
        tracks: tracksResult.tracks,
      };
    }

    return {
      success: false,
      error: tracksResult.error || 'Failed to get library',
      tracks: [],
    };
  } catch (error) {
    console.error('Error getting library:', error);
    // If backend is not running, return empty library instead of error
    return { 
      success: false, 
      error: error.message,
      tracks: []
    };
  }
});

ipcMain.handle('search-library', async (event, query) => {
  try {
    console.log('Searching library for:', query);

    const tracksResult = await fetchTracksWithArtwork({ query: query || '' });

    if (tracksResult.success) {
      return {
        success: true,
        tracks: tracksResult.tracks,
      };
    }

    return {
      success: false,
      error: tracksResult.error || 'Search failed',
      tracks: [],
    };
  } catch (error) {
    console.error('Error searching library:', error);
    return { 
      success: false, 
      error: error.message,
      tracks: []
    };
  }
});

ipcMain.handle('get-album-override', async (_event, albumId) => {
  return fetchAlbumOverrideRecord(albumId);
});

ipcMain.handle('set-album-override', async (_event, albumId, payload) => {
  const result = await setAlbumOverrideRecord(albumId, payload);
  return result;
});

ipcMain.handle('delete-playlist', async (event, id) => {
  try {
    console.log('Deleting playlist:', id);
    return { success: true, message: 'Playlist deleted' };
  } catch (error) {
    console.error('Error deleting playlist:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('add-to-playlist', async (event, playlistId, trackId) => {
  try {
    console.log('Adding track to playlist:', playlistId, trackId);
    return { success: true, message: 'Track added to playlist' };
  } catch (error) {
    console.error('Error adding track to playlist:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('get-queue', async () => {
  try {
    console.log('Getting current queue');
    return {
      success: true,
      queue: []
    };
  } catch (error) {
    console.error('Error getting queue:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('add-to-queue', async (event, trackId) => {
  try {
    console.log('Adding track to queue:', trackId);
    return { success: true, message: 'Track added to queue' };
  } catch (error) {
    console.error('Error adding track to queue:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('remove-from-queue', async (event, index) => {
  try {
    console.log('Removing track from queue at index:', index);
    return { success: true, message: 'Track removed from queue' };
  } catch (error) {
    console.error('Error removing track from queue:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('clear-queue', async () => {
  try {
    console.log('Clearing queue');
    return { success: true, message: 'Queue cleared' };
  } catch (error) {
    console.error('Error clearing queue:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('get-settings', async () => {
  try {
    console.log('Getting settings');
    return {
      success: true,
      settings: configManager ? configManager.getConfig() : {}
    };
  } catch (error) {
    console.error('Error getting settings:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('save-settings', async (event, settings) => {
  try {
    console.log('Saving settings');
    if (configManager) {
      const success = configManager.saveConfig(settings);
      return { success, message: success ? 'Settings saved' : 'Failed to save settings' };
    }
    return { success: false, error: 'Configuration manager not available' };
  } catch (error) {
    console.error('Error saving settings:', error);
    return { success: false, error: error.message };
  }
});

// Additional UX enhancement IPC handlers
ipcMain.handle('export-playlist', async (event, playlistData) => {
  try {
    console.log('Exporting playlist');
    const result = await dialog.showSaveDialog(mainWindow, {
      title: 'Export Playlist',
      defaultPath: 'playlist.m3u',
      filters: [
        { name: 'M3U Playlist', extensions: ['m3u'] },
        { name: 'M3U8 Playlist', extensions: ['m3u8'] },
        { name: 'All Files', extensions: ['*'] }
      ]
    });
    
    if (!result.canceled) {
      return { success: true, filePath: result.filePath };
    }
    return { success: false, error: 'Export cancelled' };
  } catch (error) {
    console.error('Error exporting playlist:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('show-library-stats', async () => {
  try {
    console.log('Getting library statistics');
    // You can implement actual library statistics here
    return {
      success: true,
      stats: {
        totalTracks: 150,
        totalAlbums: 25,
        totalArtists: 15,
        totalDuration: '12:30:45',
        totalSize: '2.5 GB'
      }
    };
  } catch (error) {
    console.error('Error getting library stats:', error);
    return { success: false, error: error.message };
  }
});

// Library management IPC handlers
ipcMain.handle('scan-music-library', async (event, folderPath) => {
  try {
    console.log('Scanning music library:', folderPath);
    // You can implement actual library scanning here
    // using Node.js fs and path modules
    return { success: true, message: 'Library scan completed' };
  } catch (error) {
    console.error('Error scanning library:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('get-library-tracks', async () => {
  try {
    console.log('Getting library tracks from Rust backend...');

    const tracksResult = await fetchTracksWithArtwork();

    if (tracksResult.success) {
      const tracks = tracksResult.tracks.map((track) => ({
        id: track.id,
        title: track.title,
        artist: track.artist,
        album: track.album,
        albumId: track.albumId,
        path: track.path,
        duration: track.duration,
        artwork: track.artwork,
      }));

      return {
        success: true,
        tracks,
      };
    }

    return {
      success: false,
      error: tracksResult.error || 'Failed to get tracks',
      tracks: [],
    };
  } catch (error) {
    console.error('Error getting library tracks:', error);
    return { 
      success: false, 
      error: error.message,
      tracks: []
    };
  }
});

// Playlist management IPC handlers
ipcMain.handle('create-playlist', async (event, playlistData) => {
  try {
    console.log('Creating playlist:', playlistData);
    return { success: true, message: 'Playlist created successfully' };
  } catch (error) {
    console.error('Error creating playlist:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('get-playlists', async () => {
  try {
    console.log('Getting playlists from backend');
    const response = await apiRequest('/playlists');

    if (response.success && Array.isArray(response.data)) {
      const playlists = response.data.map((playlist, index) => ({
        id: playlist.id || String(index),
        name: playlist.name || 'Untitled Playlist',
        description: playlist.description || '',
        trackCount: playlist.track_count ?? 0,
        createdAt: playlist.created_at,
        modifiedAt: playlist.modified_at,
      }));

      return {
        success: true,
        playlists,
      };
    }

    return {
      success: false,
      playlists: [],
      error: response.error || 'Failed to get playlists',
    };
  } catch (error) {
    console.error('Error getting playlists:', error);
    return { success: false, error: error.message };
  }
});

// Configuration management IPC handlers
ipcMain.handle('get-config', async () => {
  try {
    console.log('Getting configuration');
    return {
      success: true,
      config: configManager.getConfig()
    };
  } catch (error) {
    console.error('Error getting configuration:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('update-config', async (event, section, key, value) => {
  try {
    console.log('Updating configuration:', section, key, value);
    const success = configManager.updateConfig(section, key, value);
    return { success, message: success ? 'Configuration updated' : 'Failed to update configuration' };
  } catch (error) {
    console.error('Error updating configuration:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('add-music-directory', async (event, directory) => {
  try {
    console.log('Main process: Adding music directory:', directory);
    console.log('ConfigManager instance:', configManager);
    console.log('ConfigManager methods:', Object.getOwnPropertyNames(Object.getPrototypeOf(configManager)));
    
    const success = configManager.addMusicDirectory(directory);
    console.log('Add result:', success);
    
    const result = { 
      success, 
      message: success ? 'Music directory added' : 'Directory already exists or failed to add',
      directories: configManager.getMusicDirectories()
    };
    console.log('Returning result:', result);
    return result;
  } catch (error) {
    console.error('Error adding music directory:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('remove-music-directory', async (event, directory) => {
  try {
    console.log('Removing music directory:', directory);
    const success = configManager.removeMusicDirectory(directory);
    return { 
      success, 
      message: success ? 'Music directory removed' : 'Directory not found or failed to remove',
      directories: configManager.getMusicDirectories()
    };
  } catch (error) {
    console.error('Error removing music directory:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('get-music-directories', async () => {
  try {
    console.log('Getting music directories');
    return {
      success: true,
      directories: configManager.getMusicDirectories()
    };
  } catch (error) {
    console.error('Error getting music directories:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('reset-config', async () => {
  try {
    console.log('Resetting configuration to defaults');
    const success = configManager.resetToDefaults();
    return { 
      success, 
      message: success ? 'Configuration reset to defaults' : 'Failed to reset configuration',
      config: configManager.getConfig()
    };
  } catch (error) {
    console.error('Error resetting configuration:', error);
    return { success: false, error: error.message };
  }
});

ipcMain.handle('validate-config', async () => {
  try {
    console.log('Validating configuration');
    const errors = configManager.validateConfig(configManager.getConfig());
    return {
      success: true,
      isValid: errors.length === 0,
      errors: errors
    };
  } catch (error) {
    console.error('Error validating configuration:', error);
    return { success: false, error: error.message };
  }
});

// App event handlers
app.whenReady().then(() => {
  createWindow();
  createMenu();
  
  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow();
    }
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('before-quit', () => {
  // Clean up any resources before quitting
});

// Handle uncaught exceptions
process.on('uncaughtException', (error) => {
  console.error('Uncaught Exception:', error);
});

process.on('unhandledRejection', (reason, promise) => {
  console.error('Unhandled Rejection at:', promise, 'reason:', reason);
});
