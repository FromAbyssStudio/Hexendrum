import React from 'react';
import { Clock3, ListMusic, Music4, X } from 'lucide-react';
import { useAudio } from '../../contexts/AudioContext';
import '../../styles/Shell.css';

const QueuePanel = ({ visible, onClose }) => {
  const { queue, currentQueueIndex, currentTrack } = useAudio();

  return (
    <aside className={`queue-panel ${visible ? 'queue-panel--visible' : ''}`}>
      <div className="queue-panel__header">
        <div>
          <h3>Up next</h3>
          <span>{queue.length} tracks in queue</span>
        </div>
        <button className="queue-panel__close" onClick={onClose} aria-label="Hide queue">
          <X size={16} />
        </button>
      </div>

      {currentTrack && (
        <div className="queue-panel__current">
          <div className="queue-panel__artwork">
            {currentTrack.artwork ? <img src={currentTrack.artwork} alt={currentTrack.title} /> : <Music4 size={20} />}
          </div>
          <div>
            <span className="queue-panel__current-label">Now playing</span>
            <strong>{currentTrack.title}</strong>
            <span className="queue-panel__current-artist">{currentTrack.artist}</span>
          </div>
        </div>
      )}

      <div className="queue-panel__list">
        {queue.length === 0 ? (
          <div className="queue-panel__empty">
            <ListMusic size={28} />
            <p>Your queue is waiting for tracks.</p>
            <span>Add songs from the library to build the mood.</span>
          </div>
        ) : (
          queue.map((item, index) => (
            <button
              key={`${item.id}-${index}`}
              className={`queue-panel__item ${
                index === currentQueueIndex ? 'queue-panel__item--active' : ''
              }`}
            >
              <div className="queue-panel__order">{index + 1}</div>
              <div className="queue-panel__meta">
                <strong>{item.title}</strong>
                <span>{item.artist}</span>
              </div>
              <div className="queue-panel__duration">
                <Clock3 size={14} />
                <span>{formatDuration(item.duration)}</span>
              </div>
            </button>
          ))
        )}
      </div>
    </aside>
  );
};

const formatDuration = (seconds) => {
  if (!seconds) return '0:00';
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins}:${secs.toString().padStart(2, '0')}`;
};

export default QueuePanel;
