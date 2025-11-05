import React, { useState, useEffect } from 'react';
import { 
  Play, 
  Pause, 
  SkipBack, 
  SkipForward, 
  Volume2, 
  VolumeX,
  Shuffle,
  Repeat,
  Repeat1,
  List,
  Heart
} from 'lucide-react';
import { useAudio } from '../contexts/AudioContext';
import './Player.css';

const Player = ({ track, isPlaying, volume, onPlayPause, onVolumeChange }) => {
  const [showQueue, setShowQueue] = useState(false);
  const [isLiked, setIsLiked] = useState(false);
  const [progress, setProgress] = useState(0);
  const [duration, setDuration] = useState(0);
  const [showVolumeSlider, setShowVolumeSlider] = useState(false);
  
  const { 
    queue, 
    currentQueueIndex, 
    repeatMode, 
    shuffle, 
    togglePlayPause, 
    nextTrack, 
    previousTrack, 
    setVolume, 
    setRepeatMode, 
    toggleShuffle 
  } = useAudio();

  // Update progress bar
  useEffect(() => {
    if (!track) {
      setDuration(0);
      setProgress(0);
      return;
    }

    if (typeof track.duration === 'number' && !Number.isNaN(track.duration)) {
      setDuration(track.duration);
    } else {
      setDuration(0);
    }

    if (typeof track.progress === 'number' && !Number.isNaN(track.progress)) {
      setProgress(track.progress);
    } else {
      setProgress(0);
    }
  }, [track]);

  const formatTime = (seconds) => {
    if (!seconds || Number.isNaN(seconds)) return '0:00';
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  const handleProgressChange = (e) => {
    const newProgress = parseInt(e.target.value);
    setProgress(newProgress);
    // TODO: Implement seeking in audio
  };

  const handleVolumeChange = (e) => {
    const newVolume = parseFloat(e.target.value);
    setVolume(newVolume);
    onVolumeChange(newVolume);
  };

  const toggleVolume = () => {
    if (volume > 0) {
      setVolume(0);
      onVolumeChange(0);
    } else {
      setVolume(0.7);
      onVolumeChange(0.7);
    }
  };

  const getRepeatIcon = () => {
    switch (repeatMode) {
      case 'one':
        return <Repeat1 size={18} />;
      case 'all':
        return <Repeat size={18} />;
      default:
        return <Repeat size={18} />;
    }
  };

  const getRepeatColor = () => {
    return repeatMode !== 'none' ? 'var(--primary)' : 'var(--text-secondary)';
  };

  const getShuffleColor = () => {
    return shuffle ? 'var(--primary)' : 'var(--text-secondary)';
  };

  if (!track) {
    return (
      <div className="player empty">
        <div className="player-content">
          <div className="track-info">
            <div className="track-artwork">
              <div className="artwork-placeholder">
                <span>♪</span>
              </div>
            </div>
            <div className="track-details">
              <div className="track-title">No track selected</div>
              <div className="track-artist">Select a track to start playing</div>
            </div>
          </div>
          <div className="player-controls">
            <button className="control-btn" disabled>
              <SkipBack size={20} />
            </button>
            <button className="control-btn play-btn" disabled>
              <Play size={24} />
            </button>
            <button className="control-btn" disabled>
              <SkipForward size={20} />
            </button>
          </div>
          <div className="player-actions">
            <button className="control-btn" disabled>
              <Volume2 size={20} />
            </button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="player">
      <div className="player-content">
        {/* Track Info */}
        <div className="track-info">
          <div className="track-artwork">
            {track.artwork ? (
              <img src={track.artwork} alt={track.title} />
            ) : (
              <div className="artwork-placeholder">
                <span>♪</span>
              </div>
            )}
          </div>
          <div className="track-details">
            <div className="track-title">{track.title}</div>
            <div className="track-artist">{track.artist}</div>
          </div>
          <button 
            className={`like-btn ${isLiked ? 'liked' : ''}`}
            onClick={() => setIsLiked(!isLiked)}
            title={isLiked ? 'Remove from Liked Songs' : 'Add to Liked Songs'}
          >
            <Heart size={16} fill={isLiked ? 'currentColor' : 'none'} />
          </button>
        </div>

        {/* Playback Controls */}
        <div className="player-controls">
          <div className="control-buttons">
            <button 
              className="control-btn"
              onClick={toggleShuffle}
              title="Toggle Shuffle"
            >
              <Shuffle size={18} style={{ color: getShuffleColor() }} />
            </button>
            <button 
              className="control-btn"
              onClick={previousTrack}
              title="Previous Track"
            >
              <SkipBack size={20} />
            </button>
            <button 
              className="control-btn play-btn"
              onClick={togglePlayPause}
              title={isPlaying ? 'Pause' : 'Play'}
            >
              {isPlaying ? <Pause size={24} /> : <Play size={24} />}
            </button>
            <button 
              className="control-btn"
              onClick={nextTrack}
              title="Next Track"
            >
              <SkipForward size={20} />
            </button>
            <button 
              className="control-btn"
              onClick={() => setRepeatMode(repeatMode === 'none' ? 'all' : repeatMode === 'all' ? 'one' : 'none')}
              title={`Repeat: ${repeatMode === 'none' ? 'Off' : repeatMode === 'all' ? 'All' : 'One'}`}
            >
              {getRepeatIcon()}
            </button>
          </div>
          
          {/* Progress Bar */}
          <div className="progress-container">
            <span className="time-display">{formatTime(progress)}</span>
            <input
              type="range"
              className="progress-bar"
              min="0"
              max={duration || 100}
              value={progress}
              onChange={handleProgressChange}
              title="Seek to position"
            />
            <span className="time-display">{formatTime(duration)}</span>
          </div>
        </div>

        {/* Volume and Queue Controls */}
        <div className="player-actions">
          <div className="volume-control">
            <button 
              className="control-btn"
              onClick={toggleVolume}
              onMouseEnter={() => setShowVolumeSlider(true)}
              onMouseLeave={() => setShowVolumeSlider(false)}
              title="Toggle Mute"
            >
              {volume > 0 ? <Volume2 size={20} /> : <VolumeX size={20} />}
            </button>
            {showVolumeSlider && (
              <div className="volume-slider-container">
                <input
                  type="range"
                  className="volume-slider"
                  min="0"
                  max="1"
                  step="0.01"
                  value={volume}
                  onChange={handleVolumeChange}
                  title="Adjust Volume"
                />
              </div>
            )}
          </div>
          
          <button 
            className="control-btn"
            onClick={() => setShowQueue(!showQueue)}
            title="Show Queue"
          >
            <List size={20} />
          </button>
        </div>
      </div>

      {/* Queue Panel */}
      {showQueue && (
        <div className="queue-panel">
          <div className="queue-header">
            <h3>Queue</h3>
            <button 
              className="close-btn"
              onClick={() => setShowQueue(false)}
            >
              ×
            </button>
          </div>
          <div className="queue-list">
            {queue.length === 0 ? (
              <div className="empty-queue">
                <p>No tracks in queue</p>
              </div>
            ) : (
              queue.map((queueTrack, index) => (
                <div 
                  key={index} 
                  className={`queue-item ${index === currentQueueIndex ? 'current' : ''}`}
                >
                  <div className="queue-track-info">
                    <div className="queue-track-title">{queueTrack.title}</div>
                    <div className="queue-track-artist">{queueTrack.artist}</div>
                  </div>
                  <div className="queue-track-duration">
                    {formatTime(queueTrack.duration)}
                  </div>
                </div>
              ))
            )}
          </div>
        </div>
      )}
    </div>
  );
};

export default Player;
