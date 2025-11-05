use hexendrum::api::{
    ApiResponsePlaylists, ApiResponseStats, ApiResponseString, ApiResponseTracks, ApiResponseUsize,
    AudioStatusResponse, LibraryStats, PlaylistResponse, TrackResponse,
};

#[test]
fn api_response_structs_support_field_access() {
    let track = TrackResponse {
        id: "id".into(),
        title: Some("Song".into()),
        artist: Some("Artist".into()),
        album: Some("Album".into()),
        album_id: Some("album-id".into()),
        genre: Some("Genre".into()),
        duration: Some(123),
        file_size: 42,
        path: "/tmp/song.mp3".into(),
    };

    let playlist = PlaylistResponse {
        id: "playlist".into(),
        name: "Mix".into(),
        description: Some("desc".into()),
        track_count: 1,
        created_at: "2024-01-01T00:00:00Z".into(),
        modified_at: "2024-01-01T00:00:00Z".into(),
    };

    let stats = LibraryStats {
        total_tracks: 1,
        total_artists: 1,
        total_albums: 1,
        cache_size: 1,
    };

    let response_tracks = ApiResponseTracks {
        success: true,
        data: Some(vec![track]),
        error: None,
    };
    assert!(response_tracks.success);
    assert_eq!(response_tracks.data.unwrap().len(), 1);

    let response_string = ApiResponseString {
        success: true,
        data: Some("ok".into()),
        error: None,
    };
    assert_eq!(response_string.data.as_deref(), Some("ok"));

    let response_stats = ApiResponseStats {
        success: true,
        data: Some(stats),
        error: None,
    };
    assert_eq!(response_stats.data.unwrap().total_tracks, 1);

    let response_playlists = ApiResponsePlaylists {
        success: true,
        data: Some(vec![playlist]),
        error: None,
    };
    assert_eq!(response_playlists.data.unwrap().len(), 1);

    let response_usize = ApiResponseUsize {
        success: true,
        data: Some(10),
        error: None,
    };
    assert_eq!(response_usize.data, Some(10));
}

#[test]
fn audio_status_response_fields_are_accessible() {
    let status = AudioStatusResponse {
        state: "Stopped".into(),
        current_track: None,
        volume: 0.5,
    };

    assert_eq!(status.state, "Stopped");
    assert!(status.current_track.is_none());
    assert_eq!(status.volume, 0.5);
}
