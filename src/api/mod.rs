use anyhow::Result;
use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, Query, State,
    },
    http::{header, StatusCode},
    response::{Json, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::path::{Path as FsPath, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

use crate::audio::AudioPlayer;
use crate::events::{EventBus, EventMessage, EventPayload};
use crate::library::{album_identifier, AlbumService, AlbumSummary, Library, Track};
use crate::playlist::PlaylistManager;

/// API state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    pub library: Arc<Library>,
    pub playlist_manager: Arc<PlaylistManager>,
    pub audio_player: Arc<AudioPlayer>,
    pub album_service: Arc<AlbumService>,
    pub event_bus: Arc<EventBus>,
}

/// Track response format for API
#[derive(Debug, Serialize, ToSchema)]
pub struct TrackResponse {
    /// Unique track identifier
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: String,
    /// Track title
    #[schema(example = "Bohemian Rhapsody")]
    pub title: Option<String>,
    /// Artist name
    #[schema(example = "Queen")]
    pub artist: Option<String>,
    /// Album name
    #[schema(example = "A Night at the Opera")]
    pub album: Option<String>,
    /// Stable album identifier derived from artist and album tags
    #[schema(example = "1f3870be274f6c49b3e31a0c6728957f")]
    pub album_id: Option<String>,
    /// Genre
    #[schema(example = "Rock")]
    pub genre: Option<String>,
    /// Duration in seconds
    #[schema(example = 355)]
    pub duration: Option<u64>,
    /// File size in bytes
    #[schema(example = 5242880)]
    pub file_size: u64,
    /// Full file path
    #[schema(example = "/path/to/track.mp3")]
    pub path: String,
}

impl From<&Track> for TrackResponse {
    fn from(track: &Track) -> Self {
        let album_id = track
            .metadata
            .album
            .as_ref()
            .map(|album| album_identifier(track.metadata.artist.as_deref(), album));

        Self {
            id: track.id.clone(),
            title: track.metadata.title.clone(),
            artist: track.metadata.artist.clone(),
            album: track.metadata.album.clone(),
            album_id,
            genre: track.metadata.genre.clone(),
            duration: track.metadata.duration,
            file_size: track.metadata.file_size,
            path: track.metadata.file_path.to_string_lossy().to_string(),
        }
    }
}

/// Album response format for API
#[derive(Debug, Serialize, ToSchema)]
pub struct AlbumResponse {
    /// Stable album identifier derived from artist and title
    #[schema(example = "1f3870be274f6c49b3e31a0c6728957f")]
    pub id: String,
    /// Album title
    #[schema(example = "A Night at the Opera")]
    pub title: String,
    /// Primary artist (if available)
    #[schema(example = "Queen")]
    pub primary_artist: Option<String>,
    /// All contributing artists discovered in the library
    #[schema(example = r#"["Queen"]"#)]
    pub artists: Vec<String>,
    /// Number of tracks in the album
    #[schema(example = 12)]
    pub track_count: usize,
    /// Artwork endpoint URL if cached
    #[schema(example = "/api/library/albums/1f3870be274f6c49b3e31a0c6728957f/artwork")]
    pub artwork_url: Option<String>,
}

/// API response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    /// Whether the request was successful
    pub success: bool,
    /// Response data (present if success is true)
    pub data: Option<T>,
    /// Error message (present if success is false)
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    #[allow(dead_code)]
    fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

/// Scan library request
#[derive(Debug, Deserialize, ToSchema)]
pub struct ScanRequest {
    /// List of directory paths to scan for music files
    #[schema(example = r#"["/home/user/Music", "/home/user/Documents/Music"]"#)]
    pub directories: Vec<String>,
}

/// Search query parameters
#[derive(Debug, Deserialize, ToSchema)]
pub struct SearchQuery {
    /// Search query string
    #[schema(example = "rock")]
    pub q: String,
}

/// Album search query parameters
#[derive(Debug, Deserialize, ToSchema)]
pub struct AlbumSearchQuery {
    /// Optional search query string
    #[schema(example = "opera")]
    pub q: Option<String>,
}

/// OpenAPI documentation structure
#[derive(OpenApi)]
#[openapi(
    components(schemas(
        TrackResponse,
        AlbumResponse,
        ApiResponseString,
        ApiResponseTracks,
        ApiResponseStats,
        ApiResponsePlaylists,
        ApiResponseUsize,
        ScanRequest,
        SearchQuery,
        AlbumSearchQuery,
        LibraryStats,
        PlaylistResponse,
        PlayRequest,
        AudioStatusResponse,
        VolumeRequest
    )),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Library", description = "Music library management endpoints"),
        (name = "Playlists", description = "Playlist management endpoints")
    ),
    info(
        title = "Hexendrum API",
        description = "API for Hexendrum Music Player - Manage music library, playlists, and track playback.

## Available Endpoints

### Health
- `GET /api/health` - Health check

### Library
- `GET /api/library/tracks` - Get all tracks from library
- `POST /api/library/scan` - Scan directories for music files
- `GET /api/library/search?q={query}` - Search tracks
- `GET /api/library/stats` - Get library statistics

### Playlists
- `GET /api/playlists` - Get all playlists
- `POST /api/playlists/{id}/cleanup` - Cleanup specific playlist
- `POST /api/playlists/cleanup` - Cleanup all playlists

### Audio Playback
- `POST /api/audio/play` - Play audio file
- `POST /api/audio/pause` - Pause playback
- `POST /api/audio/resume` - Resume playback
- `POST /api/audio/stop` - Stop playback
- `GET /api/audio/status` - Get playback status
- `POST /api/audio/volume` - Set volume

See Swagger UI at `/swagger-ui` for interactive API documentation.",
        version = "1.0.0",
        contact(
            name = "From Abyss Studio",
            url = "https://github.com/fromabyssstudio/Hexendrum"
        )
    ),
    servers(
        (url = "http://127.0.0.1:3030", description = "Local development server")
    )
)]
struct ApiDoc;

/// Create API router
pub fn create_router(state: AppState) -> Router {
    let openapi = ApiDoc::openapi();

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", openapi.clone()))
        .route("/api/health", get(health_check))
        .route("/api/library/tracks", get(get_all_tracks))
        .route("/api/library/scan", post(scan_library))
        .route("/api/library/search", get(search_tracks))
        .route("/api/library/albums/search", get(search_albums))
        .route("/api/library/albums/:id/artwork", get(get_album_artwork))
        .route("/api/events/ws", get(events_ws_handler))
        .route("/api/library/stats", get(get_library_stats))
        .route("/api/playlists", get(get_playlists))
        .route("/api/playlists/:id/cleanup", post(cleanup_playlist))
        .route("/api/playlists/cleanup", post(cleanup_all_playlists))
        .route("/api/audio/play", post(play_audio))
        .route("/api/audio/pause", post(pause_audio))
        .route("/api/audio/resume", post(resume_audio))
        .route("/api/audio/stop", post(stop_audio))
        .route("/api/audio/status", get(get_audio_status))
        .route("/api/audio/volume", post(set_audio_volume))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state)
}

/// Health check endpoint
///
/// Returns the health status of the API server
async fn health_check() -> Json<ApiResponse<&'static str>> {
    Json(ApiResponse::success("OK"))
}

/// Get all tracks from library
///
/// Returns a list of all tracks currently in the music library.
/// Tracks are loaded from cache if available, otherwise the library may be empty.
async fn get_all_tracks(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<TrackResponse>>>, StatusCode> {
    let tracks = state.library.get_tracks();
    let track_responses: Vec<TrackResponse> = tracks.iter().map(TrackResponse::from).collect();
    Ok(Json(ApiResponse::success(track_responses)))
}

/// Scan library directories
///
/// Scans the specified directories for music files and adds them to the library.
/// Supported formats: MP3, FLAC, OGG, WAV, M4A, AAC
///
/// After scanning, the library is automatically cached for faster loading on next startup.
async fn scan_library(
    State(state): State<AppState>,
    Json(request): Json<ScanRequest>,
) -> Result<Json<ApiResponse<usize>>, StatusCode> {
    let directories: Vec<PathBuf> = request
        .directories
        .iter()
        .map(|s| PathBuf::from(s))
        .collect();

    state
        .event_bus
        .emit(EventPayload::library_scan("started", None, None));

    match state.library.scan_directories(&directories) {
        Ok(_) => {
            let count = state.library.track_count();
            info!("Library scan completed: {} tracks", count);
            state
                .event_bus
                .emit(EventPayload::library_scan("completed", None, None));
            state.event_bus.emit(EventPayload::library_updated(count));
            Ok(Json(ApiResponse::success(count)))
        }
        Err(e) => {
            error!("Failed to scan library: {}", e);
            state
                .event_bus
                .emit(EventPayload::library_scan("failed", None, None));
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Search tracks
///
/// Searches the library for tracks matching the query string.
/// Searches in track title, artist, and album fields.
async fn search_tracks(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<ApiResponse<Vec<TrackResponse>>>, StatusCode> {
    let tracks = state.library.search_tracks(&query.q);
    let track_responses: Vec<TrackResponse> = tracks.iter().map(TrackResponse::from).collect();
    Ok(Json(ApiResponse::success(track_responses)))
}

/// Search albums
///
/// Aggregates albums from the library and returns matching entries with cached artwork information.
async fn search_albums(
    State(state): State<AppState>,
    Query(query): Query<AlbumSearchQuery>,
) -> Result<Json<ApiResponse<Vec<AlbumResponse>>>, StatusCode> {
    let albums = state
        .album_service
        .search_albums(state.library.as_ref(), query.q.as_deref())
        .await;

    let album_responses: Vec<AlbumResponse> = albums
        .into_iter()
        .map(|album: AlbumSummary| {
            let AlbumSummary {
                id,
                title,
                primary_artist,
                artists,
                track_count,
                artwork_path,
            } = album;

            let artwork_url = artwork_path.map(|_| format!("/api/library/albums/{}/artwork", id));

            AlbumResponse {
                id,
                title,
                primary_artist,
                artists,
                track_count,
                artwork_url,
            }
        })
        .collect();

    Ok(Json(ApiResponse::success(album_responses)))
}

/// Subscribe to backend events (playback, library updates) using WebSocket.
async fn events_ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(move |socket| handle_events_socket(socket, state))
}

async fn handle_events_socket(mut socket: WebSocket, state: AppState) {
    if let Err(err) = send_initial_events(&mut socket, &state).await {
        tracing::warn!("Failed to send initial event snapshot: {}", err);
    }

    let mut receiver = state.event_bus.subscribe();

    loop {
        tokio::select! {
            event = receiver.recv() => {
                match event {
                    Ok(message) => {
                        match serde_json::to_string(&message) {
                            Ok(payload) => {
                                if socket.send(Message::Text(payload)).await.is_err() {
                                    break;
                                }
                            }
                            Err(err) => tracing::warn!("Failed to serialise event message: {}", err),
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(skipped)) => {
                        tracing::warn!("Event stream lagged, skipped {} events", skipped);
                    }
                }
            }
            socket_msg = socket.recv() => {
                match socket_msg {
                    Some(Ok(Message::Ping(payload))) => {
                        if socket.send(Message::Pong(payload)).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(_)) => {
                        // Ignore other incoming messages from the client
                    }
                    Some(Err(err)) => {
                        tracing::warn!("WebSocket error: {}", err);
                        break;
                    }
                }
            }
        }
    }
}

async fn send_initial_events(socket: &mut WebSocket, state: &AppState) -> Result<(), String> {
    let current_state = state.audio_player.get_state();
    let track_path = state.audio_player.get_current_track();
    let (track_id, track_duration) = track_path
        .as_deref()
        .map(|path| lookup_track_metadata(state.library.as_ref(), FsPath::new(path)))
        .unwrap_or((None, None));

    let playback_payload = EventPayload::playback_state(
        format!("{:?}", current_state).to_lowercase(),
        track_path.clone(),
        track_id,
        Some(state.audio_player.get_volume()),
        track_duration,
    );

    send_event(socket, playback_payload).await?;

    let library_count = state.library.track_count();
    send_event(socket, EventPayload::library_updated(library_count)).await?;

    Ok(())
}

async fn send_event(socket: &mut WebSocket, payload: EventPayload) -> Result<(), String> {
    let message = EventMessage::new(payload);
    let serialized = serde_json::to_string(&message).map_err(|err| err.to_string())?;
    socket
        .send(Message::Text(serialized))
        .await
        .map_err(|err| err.to_string())
}

/// Retrieve cached artwork for a specific album
async fn get_album_artwork(
    State(state): State<AppState>,
    Path(album_id): Path<String>,
) -> Result<Response, StatusCode> {
    if let Some(path) = state.album_service.cached_artwork_path(&album_id) {
        match fs::read(&path).await {
            Ok(bytes) => Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "image/jpeg")
                .body(Body::from(bytes))
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
            Err(error) => {
                error!("Failed to read artwork for album {}: {}", album_id, error);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Get library statistics
#[derive(Debug, Serialize, ToSchema)]
pub struct LibraryStats {
    /// Total number of tracks in library
    #[schema(example = 150)]
    pub total_tracks: usize,
    /// Total number of unique artists
    #[schema(example = 25)]
    pub total_artists: usize,
    /// Total number of unique albums
    #[schema(example = 45)]
    pub total_albums: usize,
    /// Approximate cache size
    #[schema(example = 150)]
    pub cache_size: usize,
}

/// Get library statistics
///
/// Returns statistics about the music library including total tracks, artists, albums, and cache size.
async fn get_library_stats(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<LibraryStats>>, StatusCode> {
    let total_tracks = state.library.track_count();
    let artists = state.library.get_artists();
    let albums = state.library.get_albums();

    let stats = LibraryStats {
        total_tracks,
        total_artists: artists.len(),
        total_albums: albums.len(),
        cache_size: total_tracks, // Could be enhanced to check actual cache file size
    };

    Ok(Json(ApiResponse::success(stats)))
}

/// Get all playlists
#[derive(Debug, Serialize, ToSchema)]
pub struct PlaylistResponse {
    /// Playlist unique identifier
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: String,
    /// Playlist name
    #[schema(example = "My Favorites")]
    pub name: String,
    /// Optional playlist description
    #[schema(example = "Best tracks collection")]
    pub description: Option<String>,
    /// Number of tracks in playlist
    #[schema(example = 25)]
    pub track_count: usize,
    /// Creation timestamp (RFC3339)
    #[schema(example = "2024-01-15T10:30:00Z")]
    pub created_at: String,
    /// Last modification timestamp (RFC3339)
    #[schema(example = "2024-01-20T14:45:00Z")]
    pub modified_at: String,
}

// Helper types for OpenAPI schema generation
#[allow(dead_code)]
#[derive(ToSchema)]
pub struct ApiResponseString {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>,
}

#[allow(dead_code)]
#[derive(ToSchema)]
pub struct ApiResponseTracks {
    pub success: bool,
    #[schema(value_type = Vec<TrackResponse>)]
    pub data: Option<Vec<TrackResponse>>,
    pub error: Option<String>,
}

#[allow(dead_code)]
#[derive(ToSchema)]
pub struct ApiResponseStats {
    pub success: bool,
    pub data: Option<LibraryStats>,
    pub error: Option<String>,
}

#[allow(dead_code)]
#[derive(ToSchema)]
pub struct ApiResponsePlaylists {
    pub success: bool,
    #[schema(value_type = Vec<PlaylistResponse>)]
    pub data: Option<Vec<PlaylistResponse>>,
    pub error: Option<String>,
}

#[allow(dead_code)]
#[derive(ToSchema)]
pub struct ApiResponseUsize {
    pub success: bool,
    pub data: Option<usize>,
    pub error: Option<String>,
}

/// Get all playlists
///
/// Returns a list of all playlists in the system.
async fn get_playlists(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<PlaylistResponse>>>, StatusCode> {
    let playlists = state.playlist_manager.get_playlists();
    let responses: Vec<PlaylistResponse> = playlists
        .iter()
        .map(|p| PlaylistResponse {
            id: p.id.clone(),
            name: p.name.clone(),
            description: p.description.clone(),
            track_count: p.track_count(),
            created_at: p.created_at.to_rfc3339(),
            modified_at: p.modified_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(ApiResponse::success(responses)))
}

/// Cleanup a specific playlist
///
/// Removes tracks from the specified playlist that no longer exist in the library.
/// Returns the number of tracks removed.
async fn cleanup_playlist(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<usize>>, StatusCode> {
    match state.playlist_manager.cleanup_playlist(&id, &state.library) {
        Ok(removed) => Ok(Json(ApiResponse::success(removed))),
        Err(e) => {
            error!("Failed to cleanup playlist {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Cleanup all playlists
///
/// Removes missing tracks from all playlists in the system.
/// Returns the total number of tracks removed across all playlists.
async fn cleanup_all_playlists(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<usize>>, StatusCode> {
    match state
        .playlist_manager
        .cleanup_missing_tracks(&state.library)
    {
        Ok(removed) => Ok(Json(ApiResponse::success(removed))),
        Err(e) => {
            error!("Failed to cleanup playlists: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn lookup_track_metadata(library: &Library, track_path: &FsPath) -> (Option<String>, Option<u64>) {
    if let Some(track) = library.get_track_by_path(track_path) {
        (Some(track.id), track.metadata.duration)
    } else {
        (None, None)
    }
}

fn emit_playback_event(
    state: &AppState,
    playback_state: &str,
    track_path: Option<String>,
    track_id: Option<String>,
    track_duration: Option<u64>,
) {
    state.event_bus.emit(EventPayload::playback_state(
        playback_state.to_string(),
        track_path,
        track_id,
        Some(state.audio_player.get_volume()),
        track_duration,
    ));
}

/// Play audio request
#[derive(Debug, Deserialize, ToSchema)]
pub struct PlayRequest {
    /// File path to audio file
    #[schema(example = "/path/to/track.mp3")]
    pub file_path: String,
}

/// Audio status response
#[derive(Debug, Serialize, ToSchema)]
pub struct AudioStatusResponse {
    /// Current playback state
    #[schema(example = "Playing")]
    pub state: String,
    /// Current track path
    #[schema(example = "/path/to/track.mp3")]
    pub current_track: Option<String>,
    /// Current volume (0.0 to 1.0)
    #[schema(example = 0.7)]
    pub volume: f32,
}

/// Play audio file
async fn play_audio(
    State(state): State<AppState>,
    Json(request): Json<PlayRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let file_path = FsPath::new(&request.file_path);

    match state.audio_player.play(file_path) {
        Ok(_) => {
            info!("Started playing: {}", request.file_path);
            let (track_id, track_duration) =
                lookup_track_metadata(state.library.as_ref(), file_path);
            emit_playback_event(
                &state,
                "playing",
                Some(request.file_path.clone()),
                track_id,
                track_duration,
            );
            Ok(Json(ApiResponse::success("Playback started".to_string())))
        }
        Err(e) => {
            error!("Failed to play audio: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Pause audio playback
async fn pause_audio(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match state.audio_player.pause() {
        Ok(_) => {
            info!("Audio paused");
            let track_path = state.audio_player.get_current_track();
            let (track_id, track_duration) = track_path
                .as_deref()
                .map(|path| lookup_track_metadata(state.library.as_ref(), FsPath::new(path)))
                .unwrap_or((None, None));
            emit_playback_event(&state, "paused", track_path, track_id, track_duration);
            Ok(Json(ApiResponse::success("Playback paused".to_string())))
        }
        Err(e) => {
            error!("Failed to pause audio: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Resume audio playback
async fn resume_audio(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match state.audio_player.resume() {
        Ok(_) => {
            info!("Audio resumed");
            let track_path = state.audio_player.get_current_track();
            let (track_id, track_duration) = track_path
                .as_deref()
                .map(|path| lookup_track_metadata(state.library.as_ref(), FsPath::new(path)))
                .unwrap_or((None, None));
            emit_playback_event(&state, "playing", track_path, track_id, track_duration);
            Ok(Json(ApiResponse::success("Playback resumed".to_string())))
        }
        Err(e) => {
            error!("Failed to resume audio: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Stop audio playback
async fn stop_audio(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let track_path_before_stop = state.audio_player.get_current_track();
    let (track_id_before_stop, track_duration_before_stop) = track_path_before_stop
        .as_deref()
        .map(|path| lookup_track_metadata(state.library.as_ref(), FsPath::new(path)))
        .unwrap_or((None, None));

    match state.audio_player.stop() {
        Ok(_) => {
            info!("Audio stopped");
            emit_playback_event(
                &state,
                "stopped",
                track_path_before_stop,
                track_id_before_stop,
                track_duration_before_stop,
            );
            Ok(Json(ApiResponse::success("Playback stopped".to_string())))
        }
        Err(e) => {
            error!("Failed to stop audio: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get audio playback status
async fn get_audio_status(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<AudioStatusResponse>>, StatusCode> {
    let audio_state = state.audio_player.get_state();
    let current_track = state.audio_player.get_current_track();
    let volume = state.audio_player.get_volume();

    let status = AudioStatusResponse {
        state: format!("{:?}", audio_state),
        current_track,
        volume,
    };

    Ok(Json(ApiResponse::success(status)))
}

/// Set volume request
#[derive(Debug, Deserialize, ToSchema)]
pub struct VolumeRequest {
    /// Volume level (0.0 to 1.0)
    #[schema(example = 0.7)]
    pub volume: f32,
}

/// Set audio volume
async fn set_audio_volume(
    State(state): State<AppState>,
    Json(request): Json<VolumeRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let volume = request.volume.max(0.0).min(1.0);

    match state.audio_player.set_volume(volume) {
        Ok(_) => {
            info!("Volume set to {}", volume);
            state.event_bus.emit(EventPayload::volume_changed(volume));
            Ok(Json(ApiResponse::success(format!(
                "Volume set to {}",
                volume
            ))))
        }
        Err(e) => {
            error!("Failed to set volume: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Start the API server
pub async fn start_server(state: AppState, port: u16) -> Result<()> {
    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    info!("API server started on http://127.0.0.1:{}", port);

    axum::serve(listener, app).await?;
    Ok(())
}
