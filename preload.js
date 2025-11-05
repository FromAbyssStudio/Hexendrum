const { contextBridge, ipcRenderer } = require('electron');

// Expose protected methods that allow the renderer process to use
// the ipcRenderer without exposing the entire object
contextBridge.exposeInMainWorld('electronAPI', {
  // App info
  getAppVersion: () => ipcRenderer.invoke('get-app-version'),
  
  // File dialogs
  selectMusicFolder: () => ipcRenderer.invoke('select-music-folder'),
  selectMusicFiles: () => ipcRenderer.invoke('select-music-files'),
  
  // Playback controls
  playTrack: (trackPath) => ipcRenderer.invoke('play-track', trackPath),
  pausePlayback: () => ipcRenderer.invoke('pause-playback'),
  resumePlayback: () => ipcRenderer.invoke('resume-playback'),
  stopPlayback: () => ipcRenderer.invoke('stop-playback'),
  nextTrack: () => ipcRenderer.invoke('next-track'),
  previousTrack: () => ipcRenderer.invoke('previous-track'),
  setVolume: (volume) => ipcRenderer.invoke('set-volume', volume),
  getVolume: () => ipcRenderer.invoke('get-volume'),
  
  // Library management
  scanLibrary: (directories) => ipcRenderer.invoke('scan-library', directories),
  getLibrary: () => ipcRenderer.invoke('get-library'),
  searchLibrary: (query) => ipcRenderer.invoke('search-library', query),
  getAlbumOverride: (albumId) => ipcRenderer.invoke('get-album-override', albumId),
  setAlbumOverride: (albumId, payload) =>
    ipcRenderer.invoke('set-album-override', albumId, payload),
  
  // Playlist management
  getPlaylists: () => ipcRenderer.invoke('get-playlists'),
  createPlaylist: (playlist) => ipcRenderer.invoke('create-playlist', playlist),
  deletePlaylist: (id) => ipcRenderer.invoke('delete-playlist', id),
  addToPlaylist: (playlistId, trackId) => ipcRenderer.invoke('add-to-playlist', playlistId, trackId),
  
  // Queue management
  getQueue: () => ipcRenderer.invoke('get-queue'),
  addToQueue: (trackId) => ipcRenderer.invoke('add-to-queue', trackId),
  removeFromQueue: (index) => ipcRenderer.invoke('remove-from-queue', index),
  clearQueue: () => ipcRenderer.invoke('clear-queue'),
  
  // Settings
  getSettings: () => ipcRenderer.invoke('get-settings'),
  saveSettings: (settings) => ipcRenderer.invoke('save-settings', settings),
  
  // Configuration management
  getConfig: () => ipcRenderer.invoke('get-config'),
  updateConfig: (section, key, value) => ipcRenderer.invoke('update-config', section, key, value),
  addMusicDirectory: (directory) => ipcRenderer.invoke('add-music-directory', directory),
  removeMusicDirectory: (directory) => ipcRenderer.invoke('remove-music-directory', directory),
  getMusicDirectories: () => ipcRenderer.invoke('get-music-directories'),
  resetConfig: () => ipcRenderer.invoke('reset-config'),
  validateConfig: () => ipcRenderer.invoke('validate-config'),
  
  // Window controls
  windowControls: {
    minimize: () => ipcRenderer.invoke('window-control', 'minimize'),
    maximize: () => ipcRenderer.invoke('window-control', 'maximize'),
    toggleMaximize: () => ipcRenderer.invoke('window-control', 'toggle-maximize'),
    close: () => ipcRenderer.invoke('window-control', 'close')
  },
  
  // Generic invoke method for backward compatibility
  invoke: (channel, ...args) => ipcRenderer.invoke(channel, ...args),
  
  // Event listeners
  onMusicFolderSelected: (callback) => {
    ipcRenderer.on('music-folder-selected', callback);
  },
  onToggleSidebar: (callback) => {
    ipcRenderer.on('toggle-sidebar', callback);
  },
  onPlaybackToggle: (callback) => {
    ipcRenderer.on('playback-toggle', callback);
  },
  onPlaybackNext: (callback) => {
    ipcRenderer.on('playback-next', callback);
  },
  onPlaybackPrevious: (callback) => {
    ipcRenderer.on('playback-previous', callback);
  },
  onVolumeUp: (callback) => {
    ipcRenderer.on('volume-up', callback);
  },
  onVolumeDown: (callback) => {
    ipcRenderer.on('volume-down', callback);
  },
  
  // Remove listeners
  removeAllListeners: (channel) => {
    ipcRenderer.removeAllListeners(channel);
  }
});
