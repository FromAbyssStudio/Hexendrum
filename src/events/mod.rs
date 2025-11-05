use chrono::{DateTime, Utc};
use serde::Serialize;
use tokio::sync::broadcast;

const DEFAULT_EVENT_CAPACITY: usize = 128;

/// Broadcast bus for backend events.
#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<EventMessage>,
}

impl EventBus {
    /// Create a new event bus with default capacity.
    pub fn new(capacity: Option<usize>) -> Self {
        let (sender, _) = broadcast::channel(capacity.unwrap_or(DEFAULT_EVENT_CAPACITY));
        Self { sender }
    }

    /// Subscribe to the event stream.
    pub fn subscribe(&self) -> broadcast::Receiver<EventMessage> {
        self.sender.subscribe()
    }

    /// Emit a new event to all subscribers.
    pub fn emit(&self, payload: EventPayload) {
        let message = EventMessage::new(payload);
        let _ = self.sender.send(message);
    }
}

/// Envelope for broadcast events, including timestamp metadata.
#[derive(Debug, Clone, Serialize)]
pub struct EventMessage {
    pub timestamp: DateTime<Utc>,
    #[serde(flatten)]
    pub payload: EventPayload,
}

impl EventMessage {
    pub fn new(payload: EventPayload) -> Self {
        Self {
            timestamp: Utc::now(),
            payload,
        }
    }
}

/// Event payloads emitted by the backend.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EventPayload {
    PlaybackState {
        state: String,
        track_path: Option<String>,
        track_id: Option<String>,
        volume: Option<f32>,
        track_duration: Option<u64>,
    },
    VolumeChanged {
        volume: f32,
    },
    LibraryScan {
        status: String,
        processed: Option<usize>,
        total: Option<usize>,
    },
    LibraryUpdated {
        total_tracks: usize,
    },
}

impl EventPayload {
    pub fn playback_state(
        state: impl Into<String>,
        track_path: Option<String>,
        track_id: Option<String>,
        volume: Option<f32>,
        track_duration: Option<u64>,
    ) -> Self {
        Self::PlaybackState {
            state: state.into(),
            track_path,
            track_id,
            volume,
            track_duration,
        }
    }

    pub fn volume_changed(volume: f32) -> Self {
        Self::VolumeChanged { volume }
    }

    pub fn library_scan(
        status: impl Into<String>,
        processed: Option<usize>,
        total: Option<usize>,
    ) -> Self {
        Self::LibraryScan {
            status: status.into(),
            processed,
            total,
        }
    }

    pub fn library_updated(total_tracks: usize) -> Self {
        Self::LibraryUpdated { total_tracks }
    }
}
