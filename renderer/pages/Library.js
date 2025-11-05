import React, { useMemo, useState } from 'react';
import { Search, Filter, Grid3X3, List, Play, Plus, Heart, Pencil } from 'lucide-react';
import { useAudio } from '../contexts/AudioContext';
import AlbumEditorModal from '../components/AlbumEditorModal';
import './Library.css';

const createEmptyAlbumEditorState = () => ({
  isOpen: false,
  albumId: null,
  albumName: '',
  albumArtist: '',
  values: {
    title: '',
    primaryArtist: '',
    searchAlbum: '',
    searchArtist: '',
  },
  defaults: {
    title: '',
    primaryArtist: '',
    searchAlbum: '',
    searchArtist: '',
  },
  loading: false,
  saving: false,
  error: null,
  metadata: null,
  refreshingArtwork: false,
});

const Library = () => {
  const [viewMode, setViewMode] = useState('grid');
  const [sortBy, setSortBy] = useState('title');
  const [filterGenre, setFilterGenre] = useState('all');
  const [selectedTracks, setSelectedTracks] = useState(() => new Set());
  const [albumEditorState, setAlbumEditorState] = useState(() => createEmptyAlbumEditorState());

  const {
    library,
    loading,
    playTrack,
    addToQueue,
    createPlaylist,
    librarySearchQuery,
    setLibrarySearchQuery,
    refreshLibrary,
    addNotification,
    getAlbumOverride,
    setAlbumOverride,
  } = useAudio();

  const closeAlbumEditor = () => {
    setAlbumEditorState(createEmptyAlbumEditorState());
  };

  const handleAlbumEditorFieldChange = (field, value) => {
    setAlbumEditorState((previous) => ({
      ...previous,
      values: {
        ...previous.values,
        [field]: value,
      },
      error: null,
    }));
  };

  const handleAlbumEditorReset = () => {
    setAlbumEditorState((previous) => ({
      ...previous,
      values: { ...previous.defaults },
      error: null,
    }));
  };

  const openAlbumEditorForTrack = async (track) => {
    if (!track?.albumId) {
      addNotification(
        'error',
        'Unable to edit album',
        'This track does not include a stable album identifier.'
      );
      return;
    }

    const fallbackTitle = track.album || '';
    const fallbackArtist = track.artist || '';

    setAlbumEditorState({
      isOpen: true,
      albumId: track.albumId,
      albumName: fallbackTitle || 'Unknown album',
      albumArtist: fallbackArtist,
      values: {
        title: fallbackTitle,
        primaryArtist: fallbackArtist,
        searchAlbum: fallbackTitle,
        searchArtist: fallbackArtist,
      },
      defaults: {
        title: fallbackTitle,
        primaryArtist: fallbackArtist,
        searchAlbum: fallbackTitle,
        searchArtist: fallbackArtist,
      },
      loading: true,
      saving: false,
      error: null,
      metadata: track.albumMetadata || null,
      refreshingArtwork: false,
    });

    try {
      const override = await getAlbumOverride(track.albumId);
      if (override) {
        setAlbumEditorState((previous) => ({
          ...previous,
          loading: false,
          metadata: override.metadata ?? previous.metadata,
          values: {
            title: override.title ?? previous.values.title,
            primaryArtist: override.primary_artist ?? previous.values.primaryArtist,
            searchAlbum: override.search_album ?? previous.values.searchAlbum,
            searchArtist: override.search_artist ?? previous.values.searchArtist,
          },
        }));
      } else {
        setAlbumEditorState((previous) => ({ ...previous, loading: false }));
      }
    } catch (error) {
      console.error('Failed to fetch album override:', error);
      setAlbumEditorState((previous) => ({
        ...previous,
        loading: false,
        error: error.message || 'Unable to load manual metadata for this album.',
      }));
    }
  };

  const handleAlbumEditorSubmit = async () => {
    if (!albumEditorState.albumId) {
      return;
    }

    const payload = {
      title: albumEditorState.values.title,
      primary_artist: albumEditorState.values.primaryArtist,
      search_album: albumEditorState.values.searchAlbum,
      search_artist: albumEditorState.values.searchArtist,
      refresh_artwork: false,
    };

    setAlbumEditorState((previous) => ({ ...previous, saving: true, error: null }));

    try {
      await setAlbumOverride(albumEditorState.albumId, payload);
      addNotification(
        'success',
        'Album metadata updated',
        'Manual album details saved successfully.'
      );

      try {
        await refreshLibrary();
      } catch (refreshError) {
        console.warn('Library refresh after manual update failed:', refreshError);
      }

      setAlbumEditorState(createEmptyAlbumEditorState());
    } catch (error) {
      console.error('Failed to save album override:', error);
      addNotification(
        'error',
        'Album update failed',
        error.message || 'Please check your connection to the backend.'
      );
      setAlbumEditorState((previous) => ({
        ...previous,
        saving: false,
        error: error.message || 'Failed to save album metadata.',
      }));
    }
  };

  const handleAlbumArtworkRefresh = async () => {
    if (!albumEditorState.albumId) {
      return;
    }

    const payload = {
      title: albumEditorState.values.title,
      primary_artist: albumEditorState.values.primaryArtist,
      search_album: albumEditorState.values.searchAlbum,
      search_artist: albumEditorState.values.searchArtist,
      refresh_artwork: true,
    };

    setAlbumEditorState((previous) => ({
      ...previous,
      refreshingArtwork: true,
      error: null,
    }));

    try {
      const record = await setAlbumOverride(albumEditorState.albumId, payload);

      addNotification(
        'success',
        'Artwork refresh requested',
        'Album artwork will update once the new image is cached.'
      );

      setAlbumEditorState((previous) => ({
        ...previous,
        refreshingArtwork: false,
        metadata: record?.metadata ?? previous.metadata,
      }));

      try {
        await refreshLibrary();
      } catch (refreshError) {
        console.warn('Library refresh after artwork refresh failed:', refreshError);
      }
    } catch (error) {
      console.error('Failed to refresh album artwork:', error);
      addNotification(
        'error',
        'Artwork refresh failed',
        error.message || 'Hexendrum could not refresh the artwork for this album.'
      );
      setAlbumEditorState((previous) => ({
        ...previous,
        refreshingArtwork: false,
        error: error.message || 'Artwork refresh failed.',
      }));
    }
  };

  const stats = useMemo(() => {
    const artists = new Set();
    const albums = new Set();

    library.forEach((track) => {
      if (track.artist) artists.add(track.artist);
      if (track.album) albums.add(track.album);
    });

    return {
      tracks: library.length,
      artists: artists.size,
      albums: albums.size,
    };
  }, [library]);

  const genres = useMemo(() => {
    const set = new Set();
    library.forEach((track) => {
      if (track.genre && track.genre !== 'Unknown') {
        set.add(track.genre);
      }
    });
    return ['all', ...Array.from(set).sort()];
  }, [library]);

  const filteredTracks = useMemo(() => {
    let next = [...library];

    if (librarySearchQuery.trim()) {
      const needle = librarySearchQuery.trim().toLowerCase();
      next = next.filter(
        (track) =>
          track.title?.toLowerCase().includes(needle) ||
          track.artist?.toLowerCase().includes(needle) ||
          track.album?.toLowerCase().includes(needle)
      );
    }

    if (filterGenre !== 'all') {
      next = next.filter((track) => track.genre === filterGenre);
    }

    next.sort((a, b) => {
      switch (sortBy) {
        case 'title':
          return a.title.localeCompare(b.title);
        case 'artist':
          return a.artist.localeCompare(b.artist);
        case 'album':
          return a.album.localeCompare(b.album);
        case 'duration':
          return (a.duration || 0) - (b.duration || 0);
        default:
          return 0;
      }
    });

    return next;
  }, [library, librarySearchQuery, sortBy, filterGenre]);

  const handleTrackSelect = (trackId) => {
    setSelectedTracks((prev) => {
      const next = new Set(prev);
      if (next.has(trackId)) {
        next.delete(trackId);
      } else {
        next.add(trackId);
      }
      return next;
    });
  };

  const handlePlaySelected = () => {
    if (selectedTracks.size === 0) return;
    const selectedList = filteredTracks.filter((track) => selectedTracks.has(track.id));
    if (selectedList.length === 0) return;

    playTrack(selectedList[0]);
    selectedList.slice(1).forEach((track) => addToQueue(track));
    setSelectedTracks(() => new Set());
  };

  const handleAddSelectedToPlaylist = async () => {
    if (selectedTracks.size === 0) return;
    const playlistName = window.prompt('Name for the new playlist?');
    if (!playlistName) return;
    const trackIds = filteredTracks
      .filter((track) => selectedTracks.has(track.id))
      .map((track) => track.id);
    await createPlaylist(playlistName, trackIds);
    setSelectedTracks(() => new Set());
  };

  const formatDuration = (seconds) => {
    if (!seconds) return '0:00';
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  const renderArtwork = (track) => {
    const content = track.artwork ? (
      <img src={track.artwork} alt={`${track.title} artwork`} loading="lazy" />
    ) : (
      <div className="artwork-placeholder">
        <span role="img" aria-hidden="true">
          ♪
        </span>
      </div>
    );

    return (
      <div className="track-artwork">
        {content}
        {track.albumIsManual && (
          <span className="album-manual-badge" title="Manual album metadata applied">
            Manual
          </span>
        )}
      </div>
    );
  };

  const renderGrid = () => (
    <div className="tracks-grid">
      {filteredTracks.map((track) => {
        const isSelected = selectedTracks.has(track.id);
        return (
          <div
            key={track.id}
            className={`track-card ${isSelected ? 'selected' : ''}`}
            onClick={() => handleTrackSelect(track.id)}
          >
            <div className="track-card-header">
              <label
                className="track-checkbox"
                onClick={(event) => event.stopPropagation()}
              >
                <input
                  type="checkbox"
                  checked={isSelected}
                  onChange={() => handleTrackSelect(track.id)}
                />
              </label>
              <button
                className="track-play-btn"
                title="Play track"
                onClick={(event) => {
                  event.stopPropagation();
                  playTrack(track);
                }}
              >
                <Play size={16} />
              </button>
            </div>

            {renderArtwork(track)}

            <div className="track-info">
              <div className="track-title" title={track.title}>
                {track.title}
              </div>
              <div className="track-artist" title={track.artist}>
                {track.artist}
              </div>
              <div className="track-album" title={track.album}>
                {track.album}
              </div>
              <div className="track-duration">{formatDuration(track.duration)}</div>
            </div>

            <div className="track-actions">
              <button
                className="action-btn"
                title="Play now"
                onClick={(event) => {
                  event.stopPropagation();
                  playTrack(track);
                }}
              >
                <Play size={14} />
              </button>
              <button
                className="action-btn"
                title="Add to queue"
                onClick={(event) => {
                  event.stopPropagation();
                  addToQueue(track);
                }}
              >
                <Plus size={14} />
              </button>
              <button
                className="action-btn"
                title="Edit album metadata"
                onClick={(event) => {
                  event.stopPropagation();
                  openAlbumEditorForTrack(track);
                }}
              >
                <Pencil size={14} />
              </button>
              <button
                className="action-btn"
                title="Like track"
                onClick={(event) => event.stopPropagation()}
              >
                <Heart size={14} />
              </button>
            </div>
          </div>
        );
      })}
    </div>
  );

  const renderList = () => (
    <div className="tracks-list">
      <div className="list-header">
        <span>Title</span>
        <span>Artist</span>
        <span>Album</span>
        <span>Genre</span>
        <span>Duration</span>
        <span>Actions</span>
      </div>
      {filteredTracks.map((track) => {
        const isSelected = selectedTracks.has(track.id);
        return (
          <div
            key={track.id}
            className={`list-item ${isSelected ? 'selected' : ''}`}
            onClick={() => handleTrackSelect(track.id)}
          >
            <div className="list-column">
              <input
                type="checkbox"
                checked={isSelected}
                onClick={(event) => event.stopPropagation()}
                onChange={() => handleTrackSelect(track.id)}
              />
              <span className="track-title" title={track.title}>
                {track.title}
              </span>
            </div>
            <div className="list-column track-artist" title={track.artist}>
              {track.artist}
            </div>
            <div className="list-column track-album" title={track.album}>
              <span>{track.album}</span>
              {track.albumIsManual && (
                <span className="album-manual-chip" title="Manual album metadata applied">
                  Manual
                </span>
              )}
            </div>
            <div className="list-column track-genre">{track.genre || 'Unknown'}</div>
            <div className="list-column track-duration">
              {formatDuration(track.duration)}
            </div>
            <div className="list-column">
              <button
                className="action-btn"
                title="Play now"
                onClick={(event) => {
                  event.stopPropagation();
                  playTrack(track);
                }}
              >
                <Play size={14} />
              </button>
              <button
                className="action-btn"
                title="Add to queue"
                onClick={(event) => {
                  event.stopPropagation();
                  addToQueue(track);
                }}
              >
                <Plus size={14} />
              </button>
              <button
                className="action-btn"
                title="Edit album metadata"
                onClick={(event) => {
                  event.stopPropagation();
                  openAlbumEditorForTrack(track);
                }}
              >
                <Pencil size={14} />
              </button>
            </div>
          </div>
        );
      })}
    </div>
  );

  const renderEmptyState = () => (
    <div className="empty-state">
      <div className="empty-state-icon">🎧</div>
      <div className="empty-state-title">No music here yet</div>
      <div className="empty-state-description">
        Connect the Hexendrum backend and scan your music folders to populate your library.
      </div>
    </div>
  );

  return (
    <div className="library-page">
      <div className="page-header">
        <div className="header-content">
          <h1>Music Library</h1>
          <p>
            {stats.tracks} tracks • {stats.artists} artists • {stats.albums} albums
          </p>
        </div>
        {selectedTracks.size > 0 && (
          <div className="selection-actions">
            <button className="btn btn-primary" onClick={handlePlaySelected}>
              <Play size={16} />
              Play Selected ({selectedTracks.size})
            </button>
            <button className="btn btn-secondary" onClick={handleAddSelectedToPlaylist}>
              <Plus size={16} />
              Add to Playlist
            </button>
          </div>
        )}
      </div>

      <div className="library-controls">
        <div className="search-section">
          <div className="search-container">
            <Search size={20} className="search-icon" />
            <input
              type="text"
              placeholder="Search tracks, artists, or albums..."
              value={librarySearchQuery}
              onChange={(event) => setLibrarySearchQuery(event.target.value)}
              className="search-input"
            />
          </div>
        </div>

        <div className="filter-section">
          <div className="filter-group">
            <Filter size={16} />
            <label htmlFor="sort-select">Sort by:</label>
            <select
              id="sort-select"
              value={sortBy}
              onChange={(event) => setSortBy(event.target.value)}
              className="filter-select"
            >
              <option value="title">Title</option>
              <option value="artist">Artist</option>
              <option value="album">Album</option>
              <option value="duration">Duration</option>
            </select>
          </div>

          <div className="filter-group">
            <label htmlFor="genre-select">Genre:</label>
            <select
              id="genre-select"
              value={filterGenre}
              onChange={(event) => setFilterGenre(event.target.value)}
              className="filter-select"
            >
              {genres.map((genre) => (
                <option key={genre} value={genre}>
                  {genre === 'all' ? 'All Genres' : genre}
                </option>
              ))}
            </select>
          </div>

          <div className="view-toggle">
            <button
              className={`view-btn ${viewMode === 'grid' ? 'active' : ''}`}
              onClick={() => setViewMode('grid')}
              title="Grid view"
            >
              <Grid3X3 size={18} />
            </button>
            <button
              className={`view-btn ${viewMode === 'list' ? 'active' : ''}`}
              onClick={() => setViewMode('list')}
              title="List view"
            >
              <List size={18} />
            </button>
          </div>
        </div>
      </div>

      <div className="tracks-container">
        {loading ? (
          <div className="loading-container">
            <div className="loading-spinner" />
          </div>
        ) : filteredTracks.length === 0 ? (
          renderEmptyState()
        ) : viewMode === 'grid' ? (
          renderGrid()
        ) : (
          renderList()
        )}
      </div>

      <AlbumEditorModal
        isOpen={albumEditorState.isOpen}
        albumName={albumEditorState.albumName}
        albumArtist={albumEditorState.albumArtist}
        values={albumEditorState.values}
        defaults={albumEditorState.defaults}
        loading={albumEditorState.loading}
        saving={albumEditorState.saving}
        refreshingArtwork={albumEditorState.refreshingArtwork}
        error={albumEditorState.error}
        metadata={albumEditorState.metadata}
        onChange={handleAlbumEditorFieldChange}
        onSubmit={handleAlbumEditorSubmit}
        onClose={closeAlbumEditor}
        onReset={handleAlbumEditorReset}
        onRefreshArtwork={handleAlbumArtworkRefresh}
      />
    </div>
  );
};

export default Library;
