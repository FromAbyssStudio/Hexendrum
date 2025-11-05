import React, { useState, useEffect, useCallback } from 'react';
import { Music, Folder, Settings as SettingsIcon, RefreshCw, Plus, Trash2 } from 'lucide-react';
import { useAudio } from '../contexts/AudioContext';
import './Settings.css';

const Settings = () => {
  const [config, setConfig] = useState(null);
  const [loading, setLoading] = useState(true);
  const [newDirectory, setNewDirectory] = useState('');
  const [message, setMessage] = useState({ type: '', text: '' });
  const { addNotification } = useAudio();

  useEffect(() => {
    loadConfig();
  }, []);

  const showMessage = useCallback(
    (type, text) => {
      setMessage({ type, text });
      if (!text) {
        return;
      }
      const notificationType = type === 'error' ? 'error' : type === 'warning' ? 'warning' : 'success';
      addNotification(notificationType, 'Settings', text, { duration: type === 'error' ? 7000 : 4000 });
    },
    [addNotification]
  );

  const loadConfig = async () => {
    try {
      console.log('Settings: loadConfig called');
      console.log('Settings: window.electronAPI available:', !!window.electronAPI);
      console.log('Settings: getConfig method available:', !!window.electronAPI?.getConfig);
      
      const result = await window.electronAPI.getConfig();
      console.log('Settings: get-config result:', result);
      
      if (result.success) {
        setConfig(result.config);
      } else {
        showMessage('error', 'Failed to load configuration');
      }
    } catch (error) {
      console.error('Settings: Error in loadConfig:', error);
      showMessage('error', 'Error loading configuration');
    } finally {
      setLoading(false);
    }
  };

  const updateConfig = async (section, key, value) => {
    try {
      const result = await window.electronAPI.updateConfig(section, key, value);
      if (result.success) {
        showMessage('success', result.message);
        loadConfig(); // Reload config to get updated values
      } else {
        showMessage('error', result.message);
      }
    } catch (error) {
      showMessage('error', 'Error updating configuration');
    }
  };

  const addMusicDirectory = async () => {
    if (!newDirectory.trim()) return;

    console.log('Settings: addMusicDirectory called with:', newDirectory);
    console.log('Settings: window.electronAPI available:', !!window.electronAPI);
    console.log('Settings: addMusicDirectory method available:', !!window.electronAPI?.addMusicDirectory);

    try {
      console.log('Settings: Calling IPC handler...');
      const result = await window.electronAPI.addMusicDirectory(newDirectory);
      console.log('Settings: IPC result:', result);
      
      if (result.success) {
        showMessage('success', result.message);
        setNewDirectory('');
        loadConfig();
      } else {
        showMessage('error', result.message);
      }
    } catch (error) {
      console.error('Settings: Error in addMusicDirectory:', error);
      showMessage('error', 'Error adding music directory');
    }
  };

  const removeMusicDirectory = async (directory) => {
    try {
      const result = await window.electronAPI.removeMusicDirectory(directory);
      if (result.success) {
        showMessage('success', result.message);
        loadConfig();
      } else {
        showMessage('error', result.message);
      }
    } catch (error) {
      showMessage('error', 'Error removing music directory');
    }
  };

  const resetConfig = async () => {
    if (!window.confirm('Are you sure you want to reset all settings to defaults?')) return;

    try {
      const result = await window.electronAPI.resetConfig();
      if (result.success) {
        showMessage('success', result.message);
        loadConfig();
      } else {
        showMessage('error', result.message);
      }
    } catch (error) {
      showMessage('error', 'Error resetting configuration');
    }
  };

  const validateConfig = async () => {
    try {
      const result = await window.electronAPI.validateConfig();
      if (result.success) {
        if (result.isValid) {
          showMessage('success', 'Configuration is valid');
        } else {
          showMessage('warning', `Configuration has ${result.errors.length} issues`);
          console.log('Configuration errors:', result.errors);
        }
      } else {
        showMessage('error', 'Error validating configuration');
      }
    } catch (error) {
      showMessage('error', 'Error validating configuration');
    }
  };

  const rebuildLibrary = async () => {
    console.log('Settings: rebuildLibrary called');
    
    if (!window.confirm('Are you sure you want to rebuild the library? This will rescan all music directories and may take some time.')) {
      console.log('Settings: User cancelled rebuild');
      return;
    }

    try {
      console.log('Settings: Starting library rebuild...');
      setMessage({ type: 'info', text: 'Rebuilding library... This may take a few minutes.' });
      
      // Get current music directories from config
      const directories = config?.library?.musicDirectories || [];
      console.log('Settings: Music directories to scan:', directories);
      
      if (directories.length === 0) {
        setMessage({ type: 'error', text: 'No music directories configured. Please add directories first.' });
        return;
      }

      console.log('Settings: Calling scanLibrary with directories:', directories);
      const result = await window.electronAPI.scanLibrary(directories);
      console.log('Settings: scanLibrary result:', result);
      
      if (result.success) {
        showMessage('success', `Library rebuilt successfully! Found ${result.tracks?.length || 0} tracks.`);
      } else {
        showMessage('error', result.message || 'Failed to rebuild library');
      }
    } catch (error) {
      console.error('Settings: Error in rebuildLibrary:', error);
      showMessage('error', 'Error rebuilding library');
    }
  };

  if (loading) {
    return (
      <div className="settings">
        <div className="loading">Loading configuration...</div>
      </div>
    );
  }

  return (
    <div className="settings">
      <div className="settings-header">
        <h1 className="settings-title">
          <SettingsIcon className="settings-section-icon" />
          Settings
        </h1>
        <p className="settings-subtitle">Configure Hexendrum to your preferences</p>
      </div>

      {message.text && (
        <div className={`settings-message ${message.type}`}>
          {message.text}
        </div>
      )}

      <div className="settings-content">
        {/* Library Settings */}
        <div className="settings-section">
          <h2 className="settings-section-title">
            <Music className="settings-section-icon" />
            Music Library
          </h2>
          
          <div className="settings-group">
            <label className="settings-label">Music Directories</label>
            <div className="directory-list">
              {config?.library?.musicDirectories?.map((dir, index) => (
                <div key={index} className="directory-item">
                  <Folder className="directory-icon" />
                  <span className="directory-path">{dir}</span>
                  <button
                    className="settings-button danger small"
                    onClick={() => removeMusicDirectory(dir)}
                    title="Remove directory"
                  >
                    <Trash2 size={16} />
                  </button>
                </div>
              ))}
            </div>
            
            <div className="add-directory">
              <input
                type="text"
                className="settings-input"
                placeholder="Enter directory path (e.g., ~/Music, /mnt/music)"
                value={newDirectory}
                onChange={(e) => setNewDirectory(e.target.value)}
                onKeyPress={(e) => e.key === 'Enter' && addMusicDirectory()}
              />
              <button
                className="settings-button"
                onClick={addMusicDirectory}
                disabled={!newDirectory.trim()}
              >
                <Plus size={16} />
                Add Directory
              </button>
            </div>
          </div>

          <div className="settings-group">
            <label className="settings-label">Auto-scan on startup</label>
            <div className="settings-checkbox">
              <input
                type="checkbox"
                checked={config?.library?.autoScan || false}
                onChange={(e) => updateConfig('library', 'autoScan', e.target.checked)}
              />
              <span className="settings-checkbox-label">Automatically scan music directories when the app starts</span>
            </div>
          </div>

          <div className="settings-group">
            <label className="settings-label">Scan Interval (seconds)</label>
            <input
              type="number"
              className="settings-input"
              min="0"
              max="3600"
              value={config?.library?.scanInterval || 300}
              onChange={(e) => updateConfig('library', 'scanInterval', parseInt(e.target.value) || 0)}
            />
            <p className="settings-description">How often to scan for new music files (0 = disabled)</p>
          </div>
        </div>

        {/* Audio Settings */}
        <div className="settings-section">
          <h2 className="settings-section-title">
            <Music className="settings-section-icon" />
            Audio Settings
          </h2>
          
          <div className="settings-group">
            <label className="settings-label">Default Volume</label>
            <input
              type="range"
              className="settings-slider"
              min="0"
              max="1"
              step="0.1"
              value={config?.audio?.defaultVolume || 0.7}
              onChange={(e) => updateConfig('audio', 'defaultVolume', parseFloat(e.target.value))}
            />
            <span className="volume-value">{Math.round((config?.audio?.defaultVolume || 0.7) * 100)}%</span>
          </div>

          <div className="settings-group">
            <label className="settings-label">Crossfade</label>
            <div className="settings-checkbox">
              <input
                type="checkbox"
                checked={config?.audio?.crossfade || false}
                onChange={(e) => updateConfig('audio', 'crossfade', e.target.checked)}
              />
              <span className="settings-checkbox-label">Enable crossfade between tracks</span>
            </div>
          </div>

          {config?.audio?.crossfade && (
            <div className="settings-group">
              <label className="settings-label">Crossfade Duration (seconds)</label>
              <input
                type="number"
                className="settings-input"
                min="0.5"
                max="10"
                step="0.5"
                value={config?.audio?.crossfadeDuration || 3.0}
                onChange={(e) => updateConfig('audio', 'crossfadeDuration', parseFloat(e.target.value))}
              />
            </div>
          )}
        </div>

        {/* Interface Settings */}
        <div className="settings-section">
          <h2 className="settings-section-title">
            <SettingsIcon className="settings-section-icon" />
            Interface
          </h2>
          
          <div className="settings-group">
            <label className="settings-label">Theme</label>
            <select
              className="settings-select"
              value={config?.interface?.theme || 'dark'}
              onChange={(e) => updateConfig('interface', 'theme', e.target.value)}
            >
              <option value="dark">Dark</option>
              <option value="light">Light</option>
              <option value="auto">Auto</option>
            </select>
          </div>

          <div className="settings-group">
            <label className="settings-label">Show File Extensions</label>
            <div className="settings-checkbox">
              <input
                type="checkbox"
                checked={config?.interface?.showFileExtensions || false}
                onChange={(e) => updateConfig('interface', 'showFileExtensions', e.target.checked)}
              />
              <span className="settings-checkbox-label">Display file extensions in the library</span>
            </div>
          </div>
        </div>

        {/* Services */}
        <div className="settings-section">
          <h2 className="settings-section-title">
            <SettingsIcon className="settings-section-icon" />
            Connected Services
          </h2>

          <div className="settings-group">
            <label className="settings-label">Last.fm API Key</label>
            <input
              type="text"
              className="settings-input"
              value={config?.services?.lastfm?.apiKey || ''}
              onChange={(e) => updateConfig('services', 'lastfm', {
                ...config?.services?.lastfm,
                apiKey: e.target.value,
                sharedSecret: config?.services?.lastfm?.sharedSecret || '',
              })}
              placeholder="Enter your Last.fm API key"
            />
          </div>

          <div className="settings-group">
            <label className="settings-label">Last.fm Shared Secret</label>
            <input
              type="password"
              className="settings-input"
              value={config?.services?.lastfm?.sharedSecret || ''}
              onChange={(e) => updateConfig('services', 'lastfm', {
                ...config?.services?.lastfm,
                apiKey: config?.services?.lastfm?.apiKey || '',
                sharedSecret: e.target.value,
              })}
              placeholder="Enter your Last.fm shared secret"
            />
            <p className="settings-description">These credentials enable album art and scrobbling via Last.fm.</p>
          </div>
        </div>

        {/* Actions */}
        <div className="settings-section">
          <div className="settings-actions">
            <button className="settings-button secondary" onClick={validateConfig}>
              <RefreshCw size={16} />
              Validate Configuration
            </button>
            <button className="settings-button secondary" onClick={rebuildLibrary}>
              <RefreshCw size={16} />
              Rebuild Library
            </button>
            <button className="settings-button secondary" onClick={resetConfig}>
              Reset to Defaults
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Settings;
