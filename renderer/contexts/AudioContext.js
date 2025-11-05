import React, {
  createContext,
  useContext,
  useReducer,
  useEffect,
  useCallback,
  useState,
  useRef,
} from 'react';
import { enrichTracksWithArtwork } from '../utils/albumArt';

const AudioContext = createContext();

const EVENTS_SOCKET_URL = 'ws://127.0.0.1:3030/api/events/ws';
const EVENTS_RECONNECT_DELAY = 2000;

// Action types
const ACTIONS = {
  SET_CURRENT_TRACK: 'SET_CURRENT_TRACK',
  SET_PLAYBACK_STATE: 'SET_PLAYBACK_STATE',
  SET_VOLUME: 'SET_VOLUME',
  SET_PROGRESS: 'SET_PROGRESS',
  SET_DURATION: 'SET_DURATION',
  SET_QUEUE: 'SET_QUEUE',
  ADD_TO_QUEUE: 'ADD_TO_QUEUE',
  REMOVE_FROM_QUEUE: 'REMOVE_FROM_QUEUE',
  CLEAR_QUEUE: 'CLEAR_QUEUE',
  SET_REPEAT_MODE: 'SET_REPEAT_MODE',
  SET_SHUFFLE: 'SET_SHUFFLE',
  SET_LIBRARY: 'SET_LIBRARY',
  SET_PLAYLISTS: 'SET_PLAYLISTS',
  SET_QUEUE_INDEX: 'SET_QUEUE_INDEX',
  SET_LOADING: 'SET_LOADING',
  SET_ERROR: 'SET_ERROR'
};

// Initial state
const initialState = {
  currentTrack: null,
  isPlaying: false,
  volume: 0.7,
  progress: 0,
  duration: 0,
  queue: [],
  currentQueueIndex: -1,
  repeatMode: 'none', // 'none', 'one', 'all'
  shuffle: false,
  library: [],
  playlists: [],
  loading: false,
  error: null
};

// Reducer
function audioReducer(state, action) {
  switch (action.type) {
    case ACTIONS.SET_CURRENT_TRACK:
      return { ...state, currentTrack: action.payload };
    
    case ACTIONS.SET_PLAYBACK_STATE:
      return { ...state, isPlaying: action.payload };
    
    case ACTIONS.SET_VOLUME:
      return { ...state, volume: action.payload };
    
    case ACTIONS.SET_PROGRESS:
      return { ...state, progress: action.payload };
    
    case ACTIONS.SET_DURATION:
      return { ...state, duration: action.payload };
    
    case ACTIONS.SET_QUEUE: {
      const startIndex =
        typeof action.startIndex === 'number'
          ? action.startIndex
          : action.payload.length > 0
            ? 0
            : -1;
      return {
        ...state,
        queue: action.payload,
        currentQueueIndex: startIndex,
      };
    }
    
    case ACTIONS.ADD_TO_QUEUE: {
      if (state.queue.some((queueTrack) => queueTrack.id === action.payload.id)) {
        return state;
      }
      const queueWasEmpty = state.queue.length === 0;
      const newQueue = [...state.queue, action.payload];
      return {
        ...state,
        queue: newQueue,
        currentQueueIndex: queueWasEmpty ? 0 : state.currentQueueIndex,
        currentTrack: queueWasEmpty ? action.payload : state.currentTrack,
      };
    }
    
    case ACTIONS.REMOVE_FROM_QUEUE:
      const newQueue = state.queue.filter((_, index) => index !== action.payload);
      return { 
        ...state, 
        queue: newQueue,
        currentQueueIndex: newQueue.length === 0 ? -1 : 
          state.currentQueueIndex >= newQueue.length ? newQueue.length - 1 : state.currentQueueIndex
      };
    
    case ACTIONS.CLEAR_QUEUE:
      return { ...state, queue: [], currentQueueIndex: -1, currentTrack: null };
    
    case ACTIONS.SET_REPEAT_MODE:
      return { ...state, repeatMode: action.payload };
    
    case ACTIONS.SET_SHUFFLE:
      return { ...state, shuffle: action.payload };
    
    case ACTIONS.SET_LIBRARY:
      return { ...state, library: action.payload };
    
    case ACTIONS.SET_PLAYLISTS:
      return { ...state, playlists: action.payload };

    case ACTIONS.SET_QUEUE_INDEX:
      return { ...state, currentQueueIndex: action.payload };

    case ACTIONS.SET_LOADING:
      return { ...state, loading: action.payload };

    case ACTIONS.SET_ERROR:
      return { ...state, error: action.payload };
    
    default:
      return state;
  }
}

// Provider component
export function AudioProvider({ children }) {
  const [state, dispatch] = useReducer(audioReducer, initialState);
  const [servicesConfig, setServicesConfig] = useState({});
  const libraryRef = useRef(state.library);
  const progressTimerRef = useRef(null);
  const progressRef = useRef(state.progress);
  const durationRef = useRef(state.duration);
  const notificationTimersRef = useRef(new Map());
  const [notifications, setNotifications] = useState([]);
  const [librarySearchQuery, setLibrarySearchQuery] = useState('');

  useEffect(() => {
    libraryRef.current = state.library;
  }, [state.library]);

  useEffect(() => {
    progressRef.current = state.progress;
  }, [state.progress]);

  useEffect(() => {
    durationRef.current = state.duration;
  }, [state.duration]);

  useEffect(() => {
    const fetchConfig = async () => {
      if (!window?.electronAPI?.getConfig) {
        return;
      }
      try {
        const response = await window.electronAPI.getConfig();
        const config = response?.config ?? response;
        if (config?.services) {
          setServicesConfig(config.services);
        }
      } catch (error) {
        console.error('Failed to load services configuration:', error);
      }
    };

    fetchConfig();
  }, []);

  const dismissNotification = useCallback((id) => {
    setNotifications((prev) => prev.filter((notification) => notification.id !== id));
    const timer = notificationTimersRef.current.get(id);
    if (timer) {
      clearTimeout(timer);
      notificationTimersRef.current.delete(id);
    }
  }, []);

  const addNotification = useCallback(
    (type, title, message, options = {}) => {
      const id = Date.now() + Math.random();
      const duration = options.duration ?? 5000;
      const notification = {
        id,
        type,
        title,
        message,
        duration,
        timestamp: Date.now(),
      };

      setNotifications((prev) => [...prev, notification]);

      if (duration > 0) {
        const timer = setTimeout(() => {
          dismissNotification(id);
        }, duration);
        notificationTimersRef.current.set(id, timer);
      }

      return id;
    },
    [dismissNotification]
  );

  const notify = useCallback(
    (type, title, message, options) => addNotification(type, title, message, options),
    [addNotification]
  );

  const notifyError = useCallback(
    (title, errorMessage) => addNotification('error', title, errorMessage),
    [addNotification]
  );

  useEffect(() => {
    window.showNotification = ({ type = 'info', title = 'Notice', message = '', duration } = {}) =>
      addNotification(type, title, message, { duration });

    return () => {
      delete window.showNotification;
    };
  }, [addNotification]);

  useEffect(() => {
    return () => {
      notificationTimersRef.current.forEach((timer) => clearTimeout(timer));
      notificationTimersRef.current.clear();
    };
  }, []);

  const loadLibrary = useCallback(
    async (showSpinner = true) => {
      if (!window?.electronAPI) {
        return;
      }
      if (showSpinner) {
        dispatch({ type: ACTIONS.SET_LOADING, payload: true });
      }
      dispatch({ type: ACTIONS.SET_ERROR, payload: null });

      try {
        const response = await window.electronAPI.getLibrary();
        const tracks = Array.isArray(response?.tracks) ? response.tracks : [];
        const enriched = await enrichTracksWithArtwork(tracks, 60, {
          lastfmApiKey:
            servicesConfig?.lastfm?.apiKey ||
            servicesConfig?.lastFm?.apiKey ||
            servicesConfig?.lastFM?.apiKey ||
            servicesConfig?.lastfmApiKey,
        });
        dispatch({ type: ACTIONS.SET_LIBRARY, payload: enriched });
      } catch (error) {
        console.error('Failed to load library:', error);
        dispatch({ type: ACTIONS.SET_ERROR, payload: error.message });
        dispatch({ type: ACTIONS.SET_LIBRARY, payload: [] });
        notifyError('Unable to load library', error.message);
      } finally {
        if (showSpinner) {
          dispatch({ type: ACTIONS.SET_LOADING, payload: false });
        }
      }
    },
    [servicesConfig, notifyError]
  );

  const loadPlaylists = useCallback(async () => {
    if (!window?.electronAPI) {
      return;
    }
    try {
      const response = await window.electronAPI.getPlaylists();
      const playlists = Array.isArray(response?.playlists) ? response.playlists : [];
      const normalised = playlists.map((playlist, index) => ({
        id: playlist.id || String(index),
        name: playlist.name || 'Untitled Playlist',
        description: playlist.description || '',
        trackCount:
          playlist.trackCount ??
          (Array.isArray(playlist.tracks) ? playlist.tracks.length : 0),
        tracks: playlist.tracks || [],
        isLiked: Boolean(playlist.isLiked),
      }));
      dispatch({ type: ACTIONS.SET_PLAYLISTS, payload: normalised });
    } catch (error) {
      console.error('Failed to load playlists:', error);
      dispatch({ type: ACTIONS.SET_ERROR, payload: error.message });
      dispatch({ type: ACTIONS.SET_PLAYLISTS, payload: [] });
      notifyError('Unable to load playlists', error.message);
    }
  }, [notifyError]);

  useEffect(() => {
    loadLibrary();
    loadPlaylists();
  }, [loadLibrary, loadPlaylists]);

  useEffect(() => {
    const clearTimer = () => {
      if (progressTimerRef.current) {
        clearInterval(progressTimerRef.current);
        progressTimerRef.current = null;
      }
    };

    clearTimer();

    if (!state.isPlaying || !state.currentTrack) {
      return;
    }

    progressRef.current = state.progress;
    progressTimerRef.current = setInterval(() => {
      const duration = durationRef.current || 0;
      const next = duration
        ? Math.min(progressRef.current + 1, duration)
        : progressRef.current + 1;

      progressRef.current = next;
      dispatch({ type: ACTIONS.SET_PROGRESS, payload: next });

      if (duration && next >= duration) {
        clearTimer();
      }
    }, 1000);

    return clearTimer;
  }, [state.isPlaying, state.currentTrack, state.duration, dispatch]);

  useEffect(() => {
    let socket = null;
    let reconnectTimer = null;
    let intentionalClose = false;

    const findTrack = (trackId, trackPath) => {
      const library = libraryRef.current || [];
      if (trackId) {
        const match = library.find((item) => item.id === trackId);
        if (match) {
          return match;
        }
      }
      if (trackPath) {
        return library.find((item) => item.path === trackPath);
      }
      return null;
    };

    const handlePlaybackEvent = (payload = {}) => {
      const playbackState = String(payload.state || '').toLowerCase();
      const trackPath = payload.track_path || payload.trackPath || null;
      const trackId = payload.track_id || payload.trackId || null;
      const track = findTrack(trackId, trackPath);
      const eventDuration =
        typeof payload.track_duration === 'number'
          ? payload.track_duration
          : typeof payload.trackDuration === 'number'
            ? payload.trackDuration
            : null;
      const effectiveDuration = track?.duration ?? eventDuration ?? null;

      if (payload.volume !== undefined && payload.volume !== null) {
        dispatch({ type: ACTIONS.SET_VOLUME, payload: payload.volume });
      }

      switch (playbackState) {
        case 'playing':
          dispatch({ type: ACTIONS.SET_PLAYBACK_STATE, payload: true });
          if (track) {
            dispatch({ type: ACTIONS.SET_CURRENT_TRACK, payload: track });
          }
          if (effectiveDuration !== null) {
            dispatch({ type: ACTIONS.SET_DURATION, payload: effectiveDuration || 0 });
          }
          break;
        case 'paused':
          dispatch({ type: ACTIONS.SET_PLAYBACK_STATE, payload: false });
          if (track) {
            dispatch({ type: ACTIONS.SET_CURRENT_TRACK, payload: track });
          }
          if (effectiveDuration !== null) {
            dispatch({ type: ACTIONS.SET_DURATION, payload: effectiveDuration || 0 });
          }
          break;
        case 'stopped':
          dispatch({ type: ACTIONS.SET_PLAYBACK_STATE, payload: false });
          dispatch({ type: ACTIONS.SET_PROGRESS, payload: 0 });
          if (track) {
            dispatch({ type: ACTIONS.SET_CURRENT_TRACK, payload: track });
          } else {
            dispatch({ type: ACTIONS.SET_CURRENT_TRACK, payload: null });
          }
          if (effectiveDuration !== null) {
            dispatch({ type: ACTIONS.SET_DURATION, payload: effectiveDuration || 0 });
          }
          break;
        default:
          break;
      }
    };

    const handleVolumeEvent = (payload = {}) => {
      if (payload.volume !== undefined && payload.volume !== null) {
        dispatch({ type: ACTIONS.SET_VOLUME, payload: payload.volume });
      }
    };

    const handleLibraryScanEvent = (payload = {}) => {
      const status = String(payload.status || '').toLowerCase();
      if (status === 'started') {
        dispatch({ type: ACTIONS.SET_LOADING, payload: true });
      } else if (status === 'completed' || status === 'failed') {
        dispatch({ type: ACTIONS.SET_LOADING, payload: false });
      }
    };

    const handleLibraryUpdatedEvent = async () => {
      await loadLibrary(false);
    };

    const handleEventMessage = async (rawMessage) => {
      let parsed;
      try {
        parsed = JSON.parse(rawMessage);
      } catch (error) {
        console.warn('[events] Failed to parse event payload:', error);
        return;
      }

      const { type, ...payload } = parsed || {};

      switch (type) {
        case 'playback_state':
          handlePlaybackEvent(payload);
          break;
        case 'volume_changed':
          handleVolumeEvent(payload);
          break;
        case 'library_scan':
          handleLibraryScanEvent(payload);
          break;
        case 'library_updated':
          await handleLibraryUpdatedEvent();
          break;
        default:
          break;
      }
    };

    const connect = () => {
      try {
        socket = new WebSocket(EVENTS_SOCKET_URL);
      } catch (error) {
        console.error('[events] Failed to open WebSocket:', error);
        reconnectTimer = setTimeout(connect, EVENTS_RECONNECT_DELAY);
        return;
      }

      socket.onmessage = (event) => {
        if (typeof event.data === 'string') {
          void handleEventMessage(event.data);
        }
      };

      socket.onerror = (error) => {
        console.error('[events] WebSocket error:', error);
        socket?.close();
      };

      socket.onclose = () => {
        if (!intentionalClose) {
          reconnectTimer = setTimeout(connect, EVENTS_RECONNECT_DELAY);
        }
      };
    };

    connect();

    return () => {
      intentionalClose = true;
      if (reconnectTimer) {
        clearTimeout(reconnectTimer);
      }
      if (socket && socket.readyState === WebSocket.OPEN) {
        socket.close();
      }
    };
  }, [loadLibrary, dispatch]);

  const setVolume = useCallback(
    async (volume) => {
      try {
        if (window?.electronAPI) {
          await window.electronAPI.setVolume(volume);
        }
        dispatch({ type: ACTIONS.SET_VOLUME, payload: volume });
      } catch (error) {
        console.error('Failed to set volume:', error);
        dispatch({ type: ACTIONS.SET_ERROR, payload: error.message });
        notifyError('Volume update failed', error.message);
      }
    },
    [notifyError]
  );

  const adjustVolume = useCallback(
    (delta) => {
      const newVolume = Math.max(0, Math.min(1, state.volume + delta));
      setVolume(newVolume);
    },
    [setVolume, state.volume]
  );

  const getAlbumOverride = useCallback(async (albumId) => {
    if (!albumId || !window?.electronAPI?.getAlbumOverride) {
      return null;
    }

    try {
      const response = await window.electronAPI.getAlbumOverride(albumId);
      if (!response) {
        return null;
      }

      if (response.success) {
        return response.data ?? null;
      }

      throw new Error(response.error || 'Failed to load album override');
    } catch (error) {
      console.error('Failed to load album override:', error);
      throw error;
    }
  }, []);

  const setAlbumOverride = useCallback(async (albumId, payload) => {
    if (!albumId) {
      throw new Error('Album identifier is required');
    }
    if (!window?.electronAPI?.setAlbumOverride) {
      throw new Error('Manual album editing is unavailable');
    }

    try {
      const response = await window.electronAPI.setAlbumOverride(albumId, payload);
      if (response?.success) {
        return response.data ?? null;
      }

      throw new Error(response?.error || 'Failed to save album override');
    } catch (error) {
      console.error('Failed to save album override:', error);
      throw error;
    }
  }, []);

  const playTrack = useCallback(
    async (track) => {
      if (!track || !track.path) {
        return;
      }
      if (state.loading) {
        return;
      }
      if (state.currentTrack?.id === track.id && state.isPlaying) {
        return;
      }

      const existingIndex = state.queue.findIndex((queued) => queued.id === track.id);
      const queuePayload = existingIndex === -1 ? [...state.queue, track] : state.queue;
      const targetIndex =
        existingIndex === -1 ? queuePayload.length - 1 : existingIndex;

      dispatch({ type: ACTIONS.SET_ERROR, payload: null });
      dispatch({ type: ACTIONS.SET_LOADING, payload: true });

      try {
        if (window?.electronAPI) {
          await window.electronAPI.playTrack(track.path);
        }

        if (existingIndex === -1) {
          dispatch({
            type: ACTIONS.SET_QUEUE,
            payload: queuePayload,
            startIndex: targetIndex,
          });
        } else {
          dispatch({ type: ACTIONS.SET_QUEUE_INDEX, payload: targetIndex });
        }

        dispatch({ type: ACTIONS.SET_CURRENT_TRACK, payload: track });
        dispatch({ type: ACTIONS.SET_PLAYBACK_STATE, payload: true });
        dispatch({ type: ACTIONS.SET_PROGRESS, payload: 0 });
        dispatch({ type: ACTIONS.SET_DURATION, payload: track.duration || 0 });
      } catch (error) {
        console.error('Failed to play track:', error);
        dispatch({ type: ACTIONS.SET_ERROR, payload: error.message });
        notifyError('Playback error', error.message);
      } finally {
        dispatch({ type: ACTIONS.SET_LOADING, payload: false });
      }
    },
    [state.queue, state.currentTrack, state.isPlaying, state.loading, notifyError]
  );

  const pausePlayback = useCallback(async () => {
    try {
      if (window?.electronAPI) {
        await window.electronAPI.pausePlayback();
      }
      dispatch({ type: ACTIONS.SET_PLAYBACK_STATE, payload: false });
    } catch (error) {
      console.error('Failed to pause playback:', error);
      dispatch({ type: ACTIONS.SET_ERROR, payload: error.message });
      notifyError('Pause failed', error.message);
    }
  }, [notifyError]);

  const resumePlayback = useCallback(async () => {
    try {
      if (window?.electronAPI) {
        await window.electronAPI.resumePlayback();
      }
      dispatch({ type: ACTIONS.SET_PLAYBACK_STATE, payload: true });
    } catch (error) {
      console.error('Failed to resume playback:', error);
      dispatch({ type: ACTIONS.SET_ERROR, payload: error.message });
      notifyError('Resume failed', error.message);
    }
  }, [notifyError]);

  const togglePlayPause = useCallback(() => {
    if (state.isPlaying) {
      pausePlayback();
    } else if (state.currentTrack) {
      resumePlayback();
    }
  }, [state.isPlaying, state.currentTrack, pausePlayback, resumePlayback]);

  const nextTrack = useCallback(async () => {
    if (state.queue.length === 0) {
      return;
    }

    let nextIndex = state.currentQueueIndex;

    if (state.repeatMode === 'one') {
      nextIndex = state.currentQueueIndex;
    } else if (state.shuffle) {
      if (state.queue.length > 1) {
        do {
          nextIndex = Math.floor(Math.random() * state.queue.length);
        } while (nextIndex === state.currentQueueIndex);
      }
    } else {
      nextIndex = state.currentQueueIndex + 1;
      if (nextIndex >= state.queue.length) {
        if (state.repeatMode === 'all') {
          nextIndex = 0;
        } else {
          return;
        }
      }
    }

    const next = state.queue[nextIndex];
    if (!next) {
      return;
    }

    try {
      if (window?.electronAPI) {
        await window.electronAPI.playTrack(next.path);
      }
      dispatch({ type: ACTIONS.SET_QUEUE_INDEX, payload: nextIndex });
      dispatch({ type: ACTIONS.SET_CURRENT_TRACK, payload: next });
      dispatch({ type: ACTIONS.SET_PROGRESS, payload: 0 });
      dispatch({ type: ACTIONS.SET_DURATION, payload: next.duration || 0 });
      dispatch({ type: ACTIONS.SET_PLAYBACK_STATE, payload: true });
    } catch (error) {
      console.error('Failed to skip to next track:', error);
      dispatch({ type: ACTIONS.SET_ERROR, payload: error.message });
      notifyError('Next track failed', error.message);
    }
  }, [
    state.queue,
    state.currentQueueIndex,
    state.repeatMode,
    state.shuffle,
    notifyError,
  ]);

  const previousTrack = useCallback(async () => {
    if (state.queue.length === 0) {
      return;
    }

    let prevIndex = state.currentQueueIndex - 1;
    if (prevIndex < 0) {
      prevIndex =
        state.repeatMode === 'all' ? state.queue.length - 1 : state.currentQueueIndex;
    }

    const previous = state.queue[prevIndex];
    if (!previous) {
      return;
    }

    try {
      if (window?.electronAPI) {
        await window.electronAPI.playTrack(previous.path);
      }
      dispatch({ type: ACTIONS.SET_QUEUE_INDEX, payload: prevIndex });
      dispatch({ type: ACTIONS.SET_CURRENT_TRACK, payload: previous });
      dispatch({ type: ACTIONS.SET_PROGRESS, payload: 0 });
      dispatch({ type: ACTIONS.SET_DURATION, payload: previous.duration || 0 });
      dispatch({ type: ACTIONS.SET_PLAYBACK_STATE, payload: true });
    } catch (error) {
      console.error('Failed to play previous track:', error);
      dispatch({ type: ACTIONS.SET_ERROR, payload: error.message });
      notifyError('Previous track failed', error.message);
    }
  }, [state.queue, state.currentQueueIndex, state.repeatMode, notifyError]);

  const addToQueue = useCallback(
    (track) => {
      if (!track) {
        return;
      }
      dispatch({ type: ACTIONS.ADD_TO_QUEUE, payload: track });
    },
    []
  );

  const removeFromQueue = useCallback(
    (index) => {
      const newQueue = state.queue.filter((_, idx) => idx !== index);
      const nextIndex =
        newQueue.length === 0
          ? -1
          : Math.min(state.currentQueueIndex, newQueue.length - 1);
      dispatch({ type: ACTIONS.SET_QUEUE, payload: newQueue, startIndex: nextIndex });
      dispatch({
        type: ACTIONS.SET_CURRENT_TRACK,
        payload: nextIndex >= 0 ? newQueue[nextIndex] : null,
      });
    },
    [state.queue, state.currentQueueIndex]
  );

  const clearQueue = useCallback(() => {
    dispatch({ type: ACTIONS.CLEAR_QUEUE });
  }, []);

  const setRepeatMode = useCallback((mode) => {
    dispatch({ type: ACTIONS.SET_REPEAT_MODE, payload: mode });
  }, []);

  const toggleShuffle = useCallback(() => {
    dispatch({ type: ACTIONS.SET_SHUFFLE, payload: !state.shuffle });
  }, [state.shuffle]);

  const scanLibrary = useCallback(
    async (directories) => {
      if (!window?.electronAPI) {
        return;
      }
      dispatch({ type: ACTIONS.SET_LOADING, payload: true });
      dispatch({ type: ACTIONS.SET_ERROR, payload: null });

      try {
        const result = await window.electronAPI.scanLibrary(directories);
        const tracks = Array.isArray(result?.tracks) ? result.tracks : [];
        const enriched = await enrichTracksWithArtwork(tracks, 60, {
          lastfmApiKey:
            servicesConfig?.lastfm?.apiKey ||
            servicesConfig?.lastFm?.apiKey ||
            servicesConfig?.lastFM?.apiKey ||
            servicesConfig?.lastfmApiKey,
        });
        dispatch({ type: ACTIONS.SET_LIBRARY, payload: enriched });
      } catch (error) {
        console.error('Failed to scan library:', error);
        dispatch({ type: ACTIONS.SET_ERROR, payload: error.message });
      } finally {
        dispatch({ type: ACTIONS.SET_LOADING, payload: false });
      }
    },
    [servicesConfig]
  );

  const createPlaylist = useCallback(
    async (name, trackIds = []) => {
      if (!name) return null;
      const payload = {
        id: (typeof crypto !== 'undefined' && crypto.randomUUID?.()) || Date.now().toString(),
        name,
        trackIds,
        trackCount: trackIds.length,
        isLiked: false,
        createdAt: new Date().toISOString(),
        tracks: trackIds,
      };

      try {
        if (window?.electronAPI) {
          await window.electronAPI.createPlaylist(payload);
        }
      } catch (error) {
        console.error('Failed to create playlist:', error);
        dispatch({ type: ACTIONS.SET_ERROR, payload: error.message });
      }

      dispatch({
        type: ACTIONS.SET_PLAYLISTS,
        payload: [...state.playlists, payload],
      });

      return payload;
    },
    [state.playlists]
  );

  const deletePlaylist = useCallback(
    async (playlistId) => {
      if (!playlistId) return;
      try {
        if (window?.electronAPI) {
          await window.electronAPI.deletePlaylist(playlistId);
        }
      } catch (error) {
        console.error('Failed to delete playlist:', error);
        dispatch({ type: ACTIONS.SET_ERROR, payload: error.message });
      }

      dispatch({
        type: ACTIONS.SET_PLAYLISTS,
        payload: state.playlists.filter((playlist) => playlist.id !== playlistId),
      });
    },
    [state.playlists]
  );

  const togglePlaylistFavorite = useCallback(
    (playlistId) => {
      dispatch({
        type: ACTIONS.SET_PLAYLISTS,
        payload: state.playlists.map((playlist) =>
          playlist.id === playlistId
            ? { ...playlist, isLiked: !playlist.isLiked }
            : playlist
        ),
      });
    },
    [state.playlists]
  );

  const refreshLibrary = useCallback(() => loadLibrary(true), [loadLibrary]);
  const refreshPlaylists = useCallback(() => loadPlaylists(), [loadPlaylists]);

  useEffect(() => {
    const handleKeyPress = (event) => {
      if (event.target.tagName === 'INPUT' || event.target.tagName === 'TEXTAREA') {
        return;
      }

      switch (event.code) {
        case 'Space':
          event.preventDefault();
          togglePlayPause();
          break;
        case 'ArrowRight':
          event.preventDefault();
          nextTrack();
          break;
        case 'ArrowLeft':
          event.preventDefault();
          previousTrack();
          break;
        case 'ArrowUp':
          event.preventDefault();
          adjustVolume(0.1);
          break;
        case 'ArrowDown':
          event.preventDefault();
          adjustVolume(-0.1);
          break;
        default:
          break;
      }
    };

    document.addEventListener('keydown', handleKeyPress);
    return () => document.removeEventListener('keydown', handleKeyPress);
  }, [togglePlayPause, nextTrack, previousTrack, adjustVolume]);

  const value = {
    ...state,
    servicesConfig,
    notifications,
    notificationCount: notifications.length,
    addNotification: notify,
    dismissNotification,
    librarySearchQuery,
    setLibrarySearchQuery,
    playTrack,
    pausePlayback,
    resumePlayback,
    togglePlayPause,
    nextTrack,
    previousTrack,
    setVolume,
    adjustVolume,
    addToQueue,
    removeFromQueue,
    clearQueue,
    setRepeatMode,
    toggleShuffle,
    scanLibrary,
    refreshLibrary,
    refreshPlaylists,
    createPlaylist,
    deletePlaylist,
    togglePlaylistFavorite,
    loadLibrary,
    loadPlaylists,
    getAlbumOverride,
    setAlbumOverride,
  };

  return <AudioContext.Provider value={value}>{children}</AudioContext.Provider>;
}

// Hook to use audio context
export function useAudio() {
  const context = useContext(AudioContext);
  if (!context) {
    throw new Error('useAudio must be used within an AudioProvider');
  }
  return context;
}
