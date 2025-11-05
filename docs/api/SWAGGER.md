# Swagger/OpenAPI Documentation

## Overview

The Hexendrum API includes interactive Swagger UI documentation for exploring and testing the API endpoints.

## Accessing Swagger UI

1. **Start the Rust backend:**
   ```bash
   cargo run
   ```

2. **Open Swagger UI in your browser:**
   Navigate to: `http://127.0.0.1:3030/swagger-ui`

3. **View OpenAPI JSON specification:**
   Navigate to: `http://127.0.0.1:3030/api-doc/openapi.json`

## Features

The Swagger UI provides:

- **Interactive API Testing**: Try out API endpoints directly from the browser
- **Schema Documentation**: View request/response schemas with examples
- **Endpoint List**: Browse all available API endpoints
- **Request Builder**: Build and test requests with proper formatting

## API Endpoints Documented

### Health Endpoints
- `GET /api/health` - Health check

### Library Endpoints
- `GET /api/library/tracks` - Get all tracks
- `POST /api/library/scan` - Scan directories
- `GET /api/library/search?q={query}` - Search tracks
- `GET /api/library/stats` - Get statistics

### Playlist Endpoints
- `GET /api/playlists` - Get all playlists
- `POST /api/playlists/{id}/cleanup` - Cleanup specific playlist
- `POST /api/playlists/cleanup` - Cleanup all playlists

## Schema Documentation

All request and response schemas are documented with:

- Field descriptions
- Example values
- Data types
- Required/optional fields

### Example Schemas

- `TrackResponse` - Track information
- `ApiResponseTracks` - Response wrapper for tracks
- `ScanRequest` - Library scan request
- `SearchQuery` - Search parameters
- `LibraryStats` - Library statistics
- `PlaylistResponse` - Playlist information

## Usage Example

### Using Swagger UI

1. Open `http://127.0.0.1:3030/swagger-ui`
2. Expand any endpoint section
3. Click "Try it out"
4. Fill in parameters (if any)
5. Click "Execute"
6. View the response

### Example: Scanning Library

1. Go to Swagger UI
2. Find `POST /api/library/scan`
3. Click "Try it out"
4. Enter request body:
   ```json
   {
     "directories": ["/home/user/Music"]
   }
   ```
5. Click "Execute"
6. View the response showing number of tracks found

## Technical Details

- **OpenAPI Version**: 3.0
- **Framework**: utoipa + utoipa-swagger-ui
- **Integration**: Axum HTTP server
- **Location**: `/swagger-ui` endpoint
- **OpenAPI Spec**: `/api-doc/openapi.json`

## Notes

- The Swagger UI automatically includes CORS headers
- All schemas are automatically generated from Rust structs
- Example values are provided in schema annotations
- The documentation stays in sync with code changes

