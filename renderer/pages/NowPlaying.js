import React, { useMemo } from 'react';
import {
  Play,
  Pause,
  SkipBack,
  SkipForward,
  Volume2,
  VolumeX,
  Heart,
  Share2,
  Clock3,
} from 'lucide-react';
import { useAudio } from '../contexts/AudioContext';
import './NowPlaying.css';

const formatTime = (seconds = 0) => {
  if (!seconds || Number.isNaN(seconds)) {
    return '0:00';
  }
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins}:${secs.toString().padStart(2, '0')}`;
};

const NowPlaying = () => {
  const {
    currentTrack,
    queue,
    isPlaying,
    progress,
    duration,
    togglePlayPause,
    nextTrack,
    previousTrack,
    setVolume,
    volume,
  } = useAudio();

  const upcoming = useMemo(() => queue.slice(0, 10), [queue]);

  if (!currentTrack) {
    return (
      <div className="now-playing-page">
        <div className="now-playing-empty">
          <div className="now-playing-empty__artwork">♪</div>
          <h2>No track playing</h2>
          <p>Select a song from your library to get the music going.</p>
        </div>
      </div>
    );
  }

  const progressPercent = duration ? Math.min((progress / duration) * 100, 100) : 0;

  return (
    <div className="now-playing-page">
      <div className="now-playing-content">
        <div className="track-display">
          <div className="artwork-container">
            {currentTrack.artwork ? (
              <img src={currentTrack.artwork} alt={currentTrack.title} />
            ) : (
              <div className="artwork-placeholder">
                <span>{currentTrack.title?.charAt(0) ?? '♪'}</span>
              </div>
            )}
          </div>

          <div className="track-details">
            <h1 className="track-title">{currentTrack.title}</h1>
            <h2 className="track-artist">{currentTrack.artist}</h2>
            <h3 className="track-album">{currentTrack.album}</h3>

            <div className="track-actions">
              <button className="action-btn" type="button" aria-label="Toggle favorite">
                <Heart size={20} />
              </button>
              <button className="action-btn" type="button" aria-label="Share track">
                <Share2 size={20} />
              </button>
            </div>
          </div>
        </div>

        <div className="player-controls">
          <div className="control-buttons">
            <button className="control-btn" type="button" onClick={previousTrack} aria-label="Previous track">
              <SkipBack size={24} />
            </button>
            <button className="control-btn play-btn" type="button" onClick={togglePlayPause} aria-label="Play or pause">
              {isPlaying ? <Pause size={32} /> : <Play size={32} />}
            </button>
            <button className="control-btn" type="button" onClick={nextTrack} aria-label="Next track">
              <SkipForward size={24} />
            </button>
          </div>

          <div className="progress-container">
            <span className="time">{formatTime(progress)}</span>
            <div className="progress-bar" role="presentation">
              <div className="progress-fill" style={{ width: `${progressPercent}%` }}></div>
            </div>
            <span className="time">{formatTime(duration)}</span>
          </div>
        </div>

        <div className="volume-control">
          <button
            className="volume-toggle"
            type="button"
            onClick={() => setVolume(volume > 0 ? 0 : 0.7)}
            aria-label="Toggle mute"
          >
            {volume > 0 ? <Volume2 size={20} /> : <VolumeX size={20} />}
          </button>
          <div className="volume-slider">
            <input
              type="range"
              min="0"
              max="1"
              step="0.01"
              value={volume}
              onChange={(event) => setVolume(parseFloat(event.target.value))}
              aria-label="Adjust volume"
            />
          </div>
        </div>

        <div className="up-next">
          <h2>Up next</h2>
          {upcoming.length === 0 ? (
            <p className="up-next__empty">Nothing queued yet. Add tracks from the library.</p>
          ) : (
            <ul>
              {upcoming.map((item, index) => (
                <li key={`${item.id}-${index}`} className={index === 0 ? 'current' : ''}>
                  <div className="up-next__order">{index + 1}</div>
                  <div className="up-next__meta">
                    <strong>{item.title}</strong>
                    <span>{item.artist}</span>
                  </div>
                  <div className="up-next__duration">
                    <Clock3 size={14} />
                    <span>{formatTime(item.duration)}</span>
                  </div>
                </li>
              ))}
            </ul>
          )}
        </div>
      </div>
    </div>
  );
};

export default NowPlaying;
