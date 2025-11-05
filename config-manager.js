const fs = require('fs');
const path = require('path');
const os = require('os');

class ConfigManager {
  constructor() {
    // Use app.getPath('userData') for Electron apps, fallback to __dirname for development
    try {
      const { app } = require('electron');
      this.configPath = path.join(app.getPath('userData'), 'config.json');
      console.log('Using Electron userData path:', this.configPath);
    } catch (error) {
      // Fallback for when not running in Electron context
      // Try to find the project root directory
      let projectRoot = __dirname;
      while (projectRoot !== '/' && !fs.existsSync(path.join(projectRoot, 'package.json'))) {
        projectRoot = path.dirname(projectRoot);
      }
      this.configPath = path.join(projectRoot, 'config.json');
      console.log('Using project root fallback path:', this.configPath);
    }
    this.config = this.loadConfig();
  }

  /**
   * Load configuration from file or create default
   */
  loadConfig() {
    try {
      console.log('Loading config from:', this.configPath);
      console.log('File exists:', fs.existsSync(this.configPath));
      
      if (fs.existsSync(this.configPath)) {
        const configData = fs.readFileSync(this.configPath, 'utf8');
        console.log('Config file content length:', configData.length);
        const config = JSON.parse(configData);
        console.log('Parsed config:', config);
        return this.expandPaths(config);
      } else {
        // Create default config
        console.log('Config file does not exist, creating default');
        const defaultConfig = this.getDefaultConfig();
        this.saveConfig(defaultConfig);
        return defaultConfig;
      }
    } catch (error) {
      console.error('Error loading config:', error);
      return this.getDefaultConfig();
    }
  }

  /**
   * Get default configuration
   */
  getDefaultConfig() {
    const homeDir = os.homedir();
    return {
      library: {
        musicDirectories: [
          path.join(homeDir, 'Music'),
          path.join(homeDir, 'Downloads', 'Music'),
          path.join(homeDir, 'Desktop', 'Music')
        ],
        supportedExtensions: ['mp3', 'flac', 'ogg', 'wav', 'aac', 'm4a'],
        autoScan: true,
        scanInterval: 300,
        excludePatterns: [
          '**/.*',
          '**/System Volume Information/**',
          '**/Thumbs.db',
          '**/desktop.ini'
        ]
      },
      audio: {
        defaultVolume: 0.7,
        crossfade: true,
        crossfadeDuration: 3.0,
        replayGain: true
      },
      playlists: {
        defaultDirectory: path.join(homeDir, 'Music', 'Playlists'),
        autoSave: true,
        maxHistory: 50
      },
      interface: {
        theme: 'dark',
        language: 'en',
        showFileExtensions: false,
        compactMode: false
      },
      services: {
        lastfm: {
          apiKey: '',
          sharedSecret: ''
        }
      },
      advanced: {
        enableLogging: true,
        logLevel: 'info',
        cacheSize: 1000,
        autoUpdate: true
      }
    };
  }

  /**
   * Expand paths with environment variables and home directory
   */
  expandPaths(config) {
    const expanded = JSON.parse(JSON.stringify(config));
    
    // Expand library directories
    if (expanded.library && expanded.library.musicDirectories) {
      expanded.library.musicDirectories = expanded.library.musicDirectories.map(dir => {
        if (typeof dir === 'string') {
          return dir.replace(/^~/, os.homedir());
        }
        return dir;
      });
    }

    // Expand playlist directory
    if (expanded.playlists && expanded.playlists.defaultDirectory) {
      expanded.playlists.defaultDirectory = expanded.playlists.defaultDirectory.replace(/^~/, os.homedir());
    }

    if (!expanded.services) {
      expanded.services = {};
    }

    if (!expanded.services.lastfm) {
      expanded.services.lastfm = {
        apiKey: '',
        sharedSecret: ''
      };
    } else {
      expanded.services.lastfm.apiKey = expanded.services.lastfm.apiKey || '';
      expanded.services.lastfm.sharedSecret = expanded.services.lastfm.sharedSecret || '';
    }

    return expanded;
  }

  /**
   * Save configuration to file
   */
  saveConfig(config) {
    try {
      console.log('Saving config to:', this.configPath);
      console.log('Config to save:', JSON.stringify(config, null, 2));
      
      // Create backup of existing config
      if (fs.existsSync(this.configPath)) {
        const backupPath = this.configPath + '.backup';
        fs.copyFileSync(this.configPath, backupPath);
        console.log('Created backup at:', backupPath);
      }

      // Save new config
      fs.writeFileSync(this.configPath, JSON.stringify(config, null, 2), 'utf8');
      console.log('Config saved successfully');
      this.config = this.expandPaths(config);
      return true;
    } catch (error) {
      console.error('Error saving config:', error);
      console.error('Config path:', this.configPath);
      console.error('Error details:', error.message);
      return false;
    }
  }

  /**
   * Get current configuration
   */
  getConfig() {
    return this.config;
  }

  /**
   * Update specific configuration section
   */
  updateConfig(section, key, value) {
    if (!this.config[section]) {
      this.config[section] = {};
    }
    this.config[section][key] = value;
    return this.saveConfig(this.config);
  }

  /**
   * Add a music directory
   */
  addMusicDirectory(directory) {
    console.log('ConfigManager.addMusicDirectory called with:', directory);
    const expandedDir = directory.replace(/^~/, os.homedir());
    console.log('Expanded directory:', expandedDir);
    console.log('Current music directories:', this.config.library.musicDirectories);
    
    if (!this.config.library.musicDirectories.includes(expandedDir)) {
      console.log('Adding directory to config');
      this.config.library.musicDirectories.push(expandedDir);
      const saveResult = this.saveConfig(this.config);
      console.log('Save result:', saveResult);
      return saveResult;
    }
    console.log('Directory already exists');
    return false; // Directory already exists
  }

  /**
   * Remove a music directory
   */
  removeMusicDirectory(directory) {
    const expandedDir = directory.replace(/^~/, os.homedir());
    const index = this.config.library.musicDirectories.indexOf(expandedDir);
    
    if (index > -1) {
      this.config.library.musicDirectories.splice(index, 1);
      return this.saveConfig(this.config);
    }
    return false; // Directory not found
  }

  /**
   * Get music directories
   */
  getMusicDirectories() {
    return this.config.library.musicDirectories;
  }

  /**
   * Check if a directory is in the music directories
   */
  isMusicDirectory(directory) {
    const expandedDir = directory.replace(/^~/, os.homedir());
    return this.config.library.musicDirectories.includes(expandedDir);
  }

  /**
   * Get supported file extensions
   */
  getSupportedExtensions() {
    return this.config.library.supportedExtensions;
  }

  /**
   * Check if a file extension is supported
   */
  isSupportedExtension(extension) {
    return this.config.library.supportedExtensions.includes(extension.toLowerCase());
  }

  /**
   * Reset configuration to defaults
   */
  resetToDefaults() {
    const defaultConfig = this.getDefaultConfig();
    return this.saveConfig(defaultConfig);
  }

  /**
   * Validate configuration
   */
  validateConfig(config) {
    const errors = [];

    // Validate library directories exist
    if (config.library && config.library.musicDirectories) {
      for (const dir of config.library.musicDirectories) {
        if (!fs.existsSync(dir)) {
          errors.push(`Music directory does not exist: ${dir}`);
        }
      }
    }

    // Validate audio volume range
    if (config.audio && (config.audio.defaultVolume < 0 || config.audio.defaultVolume > 1)) {
      errors.push('Audio volume must be between 0 and 1');
    }

    return errors;
  }
}

module.exports = ConfigManager;
