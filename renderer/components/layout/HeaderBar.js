import React, { useState, useEffect } from 'react';
import {
  Bell,
  Flame,
  Minus,
  Search,
  SlidersHorizontal,
  Sparkles,
  Square,
  ToggleRight,
  X,
} from 'lucide-react';
import { useNavigate, useLocation } from 'react-router-dom';
import { useAudio } from '../../contexts/AudioContext';
import '../../styles/Shell.css';

const HeaderBar = ({ onToggleQueue, queueVisible }) => {
  const {
    currentTrack,
    queue,
    notificationCount,
    notifications,
    librarySearchQuery,
    setLibrarySearchQuery,
  } = useAudio();
  const [searchValue, setSearchValue] = useState(librarySearchQuery);
  const navigate = useNavigate();
  const location = useLocation();

  useEffect(() => {
    setSearchValue(librarySearchQuery);
  }, [librarySearchQuery]);

  useEffect(() => {
    const handler = setTimeout(() => {
      setLibrarySearchQuery(searchValue);
      if (searchValue.trim() && location.pathname !== '/library') {
        navigate('/library');
      }
    }, 250);

    return () => clearTimeout(handler);
  }, [searchValue, setLibrarySearchQuery, navigate, location.pathname]);

  const handleWindowAction = (action) => {
    if (window?.electronAPI?.windowControls?.[action]) {
      window.electronAPI.windowControls[action]();
    }
  };

  const handleSearchChange = (event) => {
    setSearchValue(event.target.value);
  };

  const handleSearchKeyDown = (event) => {
    if (event.key === 'Escape') {
      setSearchValue('');
      setLibrarySearchQuery('');
      event.target.blur();
    } else if (event.key === 'Enter') {
      setLibrarySearchQuery(searchValue);
      if (location.pathname !== '/library') {
        navigate('/library');
      }
    }
  };

  return (
    <header className="command-bar">
      <div className="command-bar__primary">
        <div className="command-bar__search">
          <Search size={18} />
          <input
            type="search"
            placeholder="Search songs, artists, albums..."
            value={searchValue}
            onChange={handleSearchChange}
            onKeyDown={handleSearchKeyDown}
          />
          <kbd>⌘K</kbd>
        </div>

        <div className="command-bar__chips">
          <Chip icon={Flame} label="Trending" />
          <Chip icon={Sparkles} label="Fresh finds" />
          <Chip icon={SlidersHorizontal} label="Filters" onClick={onToggleQueue} active={queueVisible} />
        </div>
      </div>

      <div className="command-bar__meta">
        <div className="command-bar__now-playing">
          <span className="command-bar__meta-label">Now playing</span>
          <strong>{currentTrack ? currentTrack.title : 'Nothing queued'}</strong>
          <span className="command-bar__meta-sub">
            {queue.length > 0 ? `${queue.length} tracks in queue` : 'Queue is empty'}
          </span>
        </div>

        <div className="command-bar__actions">
          <div className="command-bar__window-controls">
            <button
              className="command-bar__window-btn"
              type="button"
              aria-label="Minimize window"
              onClick={() => handleWindowAction('minimize')}
            >
              <Minus size={12} />
            </button>
            <button
              className="command-bar__window-btn"
              type="button"
              aria-label="Toggle maximize window"
              onClick={() => handleWindowAction('toggleMaximize')}
            >
              <Square size={10} />
            </button>
            <button
              className="command-bar__window-btn command-bar__window-btn--close"
              type="button"
              aria-label="Close window"
              onClick={() => handleWindowAction('close')}
            >
              <X size={12} />
            </button>
          </div>

          <button className="command-bar__icon-btn" aria-label="Toggle theme">
            <ToggleRight size={18} />
          </button>
          <button className="command-bar__icon-btn" aria-label="Notifications">
            <Bell size={18} />
            {(notificationCount ?? notifications?.length ?? 0) > 0 && (
              <span className="command-bar__badge">{notificationCount ?? notifications.length}</span>
            )}
          </button>
          <div className="command-bar__avatar">
            <span>ML</span>
          </div>
        </div>
      </div>
    </header>
  );
};

const Chip = ({ icon: Icon, label, onClick, active }) => (
  <button
    type="button"
    className={`command-bar__chip ${active ? 'command-bar__chip--active' : ''}`}
    onClick={onClick}
  >
    <Icon size={14} />
    <span>{label}</span>
  </button>
);

export default HeaderBar;
