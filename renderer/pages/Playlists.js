import React from 'react';
import { Plus, Play, Edit, Trash2, Heart } from 'lucide-react';
import { useAudio } from '../contexts/AudioContext';
import './Playlists.css';

const Playlists = () => {
  const {
    playlists,
    library,
    playTrack,
    addToQueue,
    createPlaylist,
    deletePlaylist,
    togglePlaylistFavorite,
  } = useAudio();

  const handleCreatePlaylist = async () => {
    const name = window.prompt('Name for the new playlist?');
    if (!name) return;
    await createPlaylist(name);
  };

  const handlePlayPlaylist = (playlist) => {
    if (!playlist?.tracks?.length) return;

    const trackIds = playlist.tracks.map((item) =>
      typeof item === 'string' ? item : item.id
    );

    const tracksToPlay = trackIds
      .map((id) => library.find((track) => track.id === id))
      .filter(Boolean);

    if (tracksToPlay.length === 0) return;

    playTrack(tracksToPlay[0]);
    tracksToPlay.slice(1).forEach((track) => addToQueue(track));
  };

  const handleDeletePlaylist = async (playlistId) => {
    const confirmed = window.confirm('Delete this playlist?');
    if (!confirmed) return;
    await deletePlaylist(playlistId);
  };

  const formatDuration = (totalSeconds = 0) => {
    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = Math.floor(totalSeconds % 60);

    if (hours > 0) {
      return `${hours}:${minutes.toString().padStart(2, '0')}:${seconds
        .toString()
        .padStart(2, '0')}`;
    }
    return `${minutes}:${seconds.toString().padStart(2, '0')}`;
  };

  const computePlaylistDuration = (playlist) => {
    if (!playlist?.tracks?.length) return 0;

    const trackIds = playlist.tracks.map((item) =>
      typeof item === 'string' ? item : item.id
    );

    return trackIds.reduce((sum, id) => {
      const track = library.find((item) => item.id === id);
      return sum + (track?.duration || 0);
    }, 0);
  };

  const renderEmptyState = () => (
    <div className="empty-state">
      <div className="empty-state-icon">🎛️</div>
      <div className="empty-state-title">No playlists yet</div>
      <div className="empty-state-description">
        Create a playlist to start grouping tracks together.
      </div>
    </div>
  );

  return (
    <div className="playlists-page">
      <div className="page-header">
        <div className="header-content">
          <h1>Playlists</h1>
          <p>Curate the perfect mix for every moment</p>
        </div>
        <button className="btn btn-primary" onClick={handleCreatePlaylist}>
          <Plus size={16} />
          New Playlist
        </button>
      </div>

      {playlists.length === 0 ? (
        renderEmptyState()
      ) : (
        <div className="playlists-grid">
          {playlists.map((playlist) => {
            const durationSeconds = computePlaylistDuration(playlist);
            return (
              <div key={playlist.id} className="playlist-card">
                <div className="playlist-header">
                  <div className="playlist-actions">
                    <button
                      className="action-btn"
                      title="Play playlist"
                      onClick={() => handlePlayPlaylist(playlist)}
                    >
                      <Play size={16} />
                    </button>
                    <button
                      className="action-btn"
                      title="Edit playlist"
                      onClick={() => window.alert('Playlist editing coming soon')}
                    >
                      <Edit size={16} />
                    </button>
                    <button
                      className="action-btn"
                      title="Delete playlist"
                      onClick={() => handleDeletePlaylist(playlist.id)}
                    >
                      <Trash2 size={16} />
                    </button>
                  </div>
                </div>

                <div className="playlist-info">
                  <div className="playlist-name">{playlist.name}</div>
                  <div className="playlist-stats">
                    {playlist.trackCount ?? playlist.tracks?.length ?? 0} tracks •{' '}
                    {formatDuration(durationSeconds)}
                  </div>
                </div>

                <button
                  className={`like-btn ${playlist.isLiked ? 'liked' : ''}`}
                  onClick={() => togglePlaylistFavorite(playlist.id)}
                  title={playlist.isLiked ? 'Remove from favourites' : 'Favourite playlist'}
                >
                  <Heart size={16} fill={playlist.isLiked ? 'currentColor' : 'none'} />
                </button>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
};

export default Playlists;
