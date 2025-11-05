import React from 'react';
import { X, Loader2, RefreshCcw } from 'lucide-react';
import './AlbumEditorModal.css';

const AlbumEditorModal = ({
  isOpen,
  albumName,
  albumArtist,
  values,
  defaults,
  loading,
  saving,
  refreshingArtwork = false,
  error,
  metadata,
  onChange,
  onSubmit,
  onClose,
  onReset,
  onRefreshArtwork,
}) => {
  if (!isOpen) {
    return null;
  }

  const handleBackdropClick = (event) => {
    if (event.target === event.currentTarget && !saving) {
      onClose();
    }
  };

  const handleSubmit = (event) => {
    event.preventDefault();
    if (!saving) {
      onSubmit();
    }
  };

  return (
    <div
      className="album-editor-backdrop"
      role="dialog"
      aria-modal="true"
      aria-labelledby="album-editor-title"
      onClick={handleBackdropClick}
    >
      <div className="album-editor-modal" onClick={(event) => event.stopPropagation()}>
        <header className="album-editor-header">
          <div>
            <h2 id="album-editor-title">{albumName || 'Album metadata'}</h2>
            {albumArtist && <p className="album-editor-subtitle">{albumArtist}</p>}
          </div>
          <button
            className="album-editor-close"
            type="button"
            onClick={onClose}
            disabled={saving}
            aria-label="Close album editor"
          >
            <X size={18} />
          </button>
        </header>

        <div className="album-editor-body">
          {loading ? (
            <div className="album-editor-loading">
              <Loader2 size={22} className="album-editor-spinner" />
              <span>Loading current manual metadata…</span>
            </div>
          ) : (
            <form onSubmit={handleSubmit} className="album-editor-form">
              <div className="album-editor-fields">
                <div className="album-editor-field">
                  <label htmlFor="album-editor-title-input">Display title</label>
                  <input
                    id="album-editor-title-input"
                    type="text"
                    value={values.title}
                    onChange={(event) => onChange('title', event.target.value)}
                    placeholder={defaults.title || 'Album title'}
                    disabled={saving}
                  />
                  <p className="album-editor-help">Leave blank to keep the automatic title.</p>
                </div>

                <div className="album-editor-field">
                  <label htmlFor="album-editor-primary-artist">Primary artist</label>
                  <input
                    id="album-editor-primary-artist"
                    type="text"
                    value={values.primaryArtist}
                    onChange={(event) => onChange('primaryArtist', event.target.value)}
                    placeholder={defaults.primaryArtist || 'Album artist'}
                    disabled={saving}
                  />
                  <p className="album-editor-help">
                    Leave blank to use the first artist Hexendrum finds in your files.
                  </p>
                </div>

                <div className="album-editor-field">
                  <label htmlFor="album-editor-search-album">Search album title</label>
                  <input
                    id="album-editor-search-album"
                    type="text"
                    value={values.searchAlbum}
                    onChange={(event) => onChange('searchAlbum', event.target.value)}
                    placeholder={defaults.searchAlbum || 'Lookup album title'}
                    disabled={saving}
                  />
                  <p className="album-editor-help">
                    Used when asking online services for artwork and metadata.
                  </p>
                </div>

                <div className="album-editor-field">
                  <label htmlFor="album-editor-search-artist">Search artist</label>
                  <input
                    id="album-editor-search-artist"
                    type="text"
                    value={values.searchArtist}
                    onChange={(event) => onChange('searchArtist', event.target.value)}
                    placeholder={defaults.searchArtist || 'Lookup artist'}
                    disabled={saving}
                  />
                  <p className="album-editor-help">
                    Leave blank to fall back to the display artist above.
                  </p>
                </div>
              </div>

              {metadata && (
                <section className="album-editor-metadata">
                  <div className="album-editor-metadata-header">
                    <h3>Cached metadata</h3>
                    {metadata.source && (
                      <span className="album-editor-chip">{metadata.source}</span>
                    )}
                    {onRefreshArtwork && (
                      <button
                        type="button"
                        className="btn btn-ghost album-editor-refresh-art"
                        onClick={onRefreshArtwork}
                        disabled={saving || loading || refreshingArtwork}
                      >
                        {refreshingArtwork ? (
                          <Loader2 size={16} className="album-editor-spinner" />
                        ) : (
                          <RefreshCcw size={16} />
                        )}
                        <span>Refresh artwork</span>
                      </button>
                    )}
                  </div>
                  <div className="album-editor-metadata-grid">
                    {metadata.release_date && (
                      <div className="album-editor-metadata-item">
                        <span className="label">Release date</span>
                        <span className="value">{metadata.release_date}</span>
                      </div>
                    )}
                    {metadata.tags && metadata.tags.length > 0 && (
                      <div className="album-editor-metadata-item">
                        <span className="label">Tags</span>
                        <span className="value">{metadata.tags.join(', ')}</span>
                      </div>
                    )}
                    {metadata.url && (
                      <div className="album-editor-metadata-item">
                        <span className="label">Source</span>
                        <a
                          className="value link"
                          href={metadata.url}
                          target="_blank"
                          rel="noreferrer"
                        >
                          {metadata.url}
                        </a>
                      </div>
                    )}
                  </div>
                  {metadata.summary && (
                    <p className="album-editor-metadata-summary">{metadata.summary}</p>
                  )}
                </section>
              )}

              {error && <div className="album-editor-error">{error}</div>}

              <footer className="album-editor-footer">
                <div className="album-editor-footer-left">
                  <button
                    type="button"
                    className="album-editor-reset"
                    onClick={onReset}
                    disabled={saving || loading}
                  >
                    Reset to auto values
                  </button>
                  {onRefreshArtwork && !metadata && (
                    <button
                      type="button"
                      className="btn btn-ghost"
                      onClick={onRefreshArtwork}
                      disabled={saving || loading || refreshingArtwork}
                    >
                      {refreshingArtwork ? (
                        <Loader2 size={16} className="album-editor-spinner" />
                      ) : (
                        <RefreshCcw size={16} />
                      )}
                      <span>Refresh artwork</span>
                    </button>
                  )}
                </div>
                <div className="album-editor-footer-actions">
                  <button
                    type="button"
                    className="btn btn-ghost"
                    onClick={onClose}
                    disabled={saving}
                  >
                    Cancel
                  </button>
                  <button type="submit" className="btn btn-primary" disabled={saving}>
                    {saving ? (
                      <>
                        <Loader2 size={16} className="album-editor-spinner" />
                        Saving…
                      </>
                    ) : (
                      'Save changes'
                    )}
                  </button>
                </div>
              </footer>
            </form>
          )}
        </div>
      </div>
    </div>
  );
};

export default AlbumEditorModal;
