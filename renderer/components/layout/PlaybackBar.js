import React from 'react';
import {
  ListMusic,
  Pause,
  Play,
  Repeat,
  Repeat1,
  Shuffle,
  SkipBack,
  SkipForward,
  Volume2,
  VolumeX,
} from 'lucide-react';
import { useAudio } from '../../contexts/AudioContext';
import '../../styles/Shell.css';

const PlaybackBar = ({ onToggleQueue, queueVisible }) => {
  const {
    currentTrack,
    isPlaying,
    volume,
    progress,
    duration,
    togglePlayPause,
    nextTrack,
    previousTrack,
    setVolume,
    setRepeatMode,
    repeatMode,
    toggleShuffle,
    shuffle,
  } = useAudio();

  const safeDuration = Math.max(0, duration || 0);
  const safeProgress = Math.min(Math.max(0, progress || 0), safeDuration || Number.MAX_SAFE_INTEGER);
  const remaining = Math.max(0, safeDuration - safeProgress);

  const hasTrack = Boolean(currentTrack);

  const handleVolumeChange = (event) => {
    setVolume(parseFloat(event.target.value));
  };

  const handleRepeatCycle = () => {
    const nextMode = repeatMode === 'none' ? 'all' : repeatMode === 'all' ? 'one' : 'none';
    setRepeatMode(nextMode);
  };

  const repeatIcon = repeatMode === 'one' ? <Repeat1 size={18} /> : <Repeat size={18} />;

  const renderArtwork = () => {
    if (!hasTrack) {
      return '♪';
    }

    if (currentTrack?.artwork) {
      return <img src={currentTrack.artwork} alt={currentTrack.title} />;
    }

    return currentTrack.title?.charAt(0)?.toUpperCase() ?? '♪';
  };

  return (
    <footer className="playback-bar">
      <div className="playback-bar__now">
        <div className="playback-bar__artwork">{renderArtwork()}</div>
        <div className="playback-bar__meta">
          <strong>{hasTrack ? currentTrack.title : 'Nothing playing'}</strong>
          <span>{hasTrack ? currentTrack.artist : 'Queue something to get started'}</span>
        </div>
        <button
          type="button"
          className={`playback-bar__queue-btn ${queueVisible ? 'playback-bar__queue-btn--active' : ''}`}
          onClick={onToggleQueue}
          aria-label="Toggle queue"
        >
          <ListMusic size={18} />
        </button>
      </div>

      <div className="playback-bar__controls">
        <div className="playback-bar__buttons">
          <button
            type="button"
            className={`playback-bar__btn ${shuffle ? 'playback-bar__btn--active' : ''}`}
            onClick={toggleShuffle}
            aria-label="Toggle shuffle"
          >
            <Shuffle size={18} />
          </button>
          <button type="button" className="playback-bar__btn" onClick={previousTrack} aria-label="Previous track">
            <SkipBack size={20} />
          </button>
          <button
            type="button"
            className="playback-bar__btn playback-bar__btn--primary"
            onClick={togglePlayPause}
            aria-label={isPlaying ? 'Pause' : 'Play'}
            disabled={!hasTrack}
          >
            {isPlaying ? <Pause size={22} /> : <Play size={22} />}
          </button>
          <button type="button" className="playback-bar__btn" onClick={nextTrack} aria-label="Next track">
            <SkipForward size={20} />
          </button>
          <button
            type="button"
            className={`playback-bar__btn ${repeatMode !== 'none' ? 'playback-bar__btn--active' : ''}`}
            onClick={handleRepeatCycle}
            aria-label="Toggle repeat"
          >
            {repeatIcon}
          </button>
        </div>

        <div className="playback-bar__timeline">
          <span>{formatTime(safeProgress)}</span>
          <input
            type="range"
            min="0"
            max={safeDuration}
            value={Math.min(safeProgress, safeDuration)}
            onChange={() => {}}
            aria-label="Playback progress"
            disabled={!hasTrack}
          />
          <span>{safeDuration > 0 ? `-${formatTime(remaining)}` : formatTime(remaining)}</span>
        </div>
      </div>

      <div className="playback-bar__extras">
        <div className="playback-bar__volume">
          <button
            type="button"
            className="playback-bar__btn"
            onClick={() => setVolume(volume > 0 ? 0 : 0.7)}
            aria-label="Mute audio"
          >
            {volume > 0 ? <Volume2 size={18} /> : <VolumeX size={18} />}
          </button>
          <input
            type="range"
            min="0"
            max="1"
            step="0.01"
            value={volume}
            onChange={handleVolumeChange}
            aria-label="Adjust volume"
          />
        </div>
      </div>
    </footer>
  );
};

const formatTime = (seconds = 0) => {
  if (!seconds || Number.isNaN(seconds)) {
    return '0:00';
  }
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins}:${secs.toString().padStart(2, '0')}`;
};

export default PlaybackBar;
