# Backend Connection Guide

## Overview

The frontend is now connected to the Rust backend via HTTP API. The backend runs an HTTP server on port 3030 that the Electron frontend communicates with.

## Architecture

```
┌─────────────────┐
│  React Frontend │
│  (Library.js)   │
└────────┬────────┘
         │
         │ IPC (electronAPI)
         ▼
┌─────────────────┐
│ Electron Main   │
│  (main.js)      │
│  HTTP Client    │
└────────┬────────┘
         │
         │ HTTP (localhost:3030)
         ▼
┌─────────────────┐
│  Rust Backend   │
│  (HTTP API)     │
│  Library System │
└─────────────────┘
```

## Running the Application

### 1. Start the Rust Backend

The backend starts an HTTP server on `http://127.0.0.1:3030`:

```bash
# Start backend (starts HTTP server automatically)
cargo run
```

You should see:
```
Starting API server on port 3030...
API server running at http://127.0.0.1:3030
```

### 2. Start the Electron Frontend

In a separate terminal:

```bash
npm run dev
```

Or using the Makefile:
```bash
make run-full  # Runs both backend and frontend
```

## API Endpoints

The Rust backend provides the following HTTP endpoints:

### Library Endpoints

- **GET** `/api/library/tracks` - Get all tracks from library
- **POST** `/api/library/scan` - Scan directories for music files
  ```json
  {
    "directories": ["/path/to/music"]
  }
  ```
- **GET** `/api/library/search?q=query` - Search tracks by query
- **GET** `/api/library/stats` - Get library statistics

### Playlist Endpoints

- **GET** `/api/playlists` - Get all playlists
- **POST** `/api/playlists/:id/cleanup` - Cleanup specific playlist
- **POST** `/api/playlists/cleanup` - Cleanup all playlists

### Health Check

- **GET** `/api/health` - Check if API is running

### API Documentation

- **GET** `/swagger-ui` - Interactive Swagger UI for API documentation
- **GET** `/api-doc/openapi.json` - OpenAPI specification JSON

## Data Flow

1. **Frontend** (`Library.js`) calls `window.electronAPI.getLibrary()`
2. **Electron** (`main.js`) receives IPC message and makes HTTP request to `http://127.0.0.1:3030/api/library/tracks`
3. **Rust Backend** (`api/mod.rs`) handles request, queries `Library` instance, returns JSON
4. **Electron** transforms response and sends back to frontend via IPC
5. **Frontend** receives real track data and displays it

## Response Format

All API responses follow this format:

```json
{
  "success": true,
  "data": [...],
  "error": null
}
```

### Track Response Format

```json
{
  "id": "uuid",
  "title": "Track Title",
  "artist": "Artist Name",
  "album": "Album Name",
  "genre": "Genre",
  "duration": 180,
  "file_size": 5242880,
  "path": "/path/to/track.mp3"
}
```

## Troubleshooting

### Backend not responding

1. **Check if backend is running:**
   ```bash
   curl http://127.0.0.1:3030/api/health
   ```
   Should return: `{"success":true,"data":"OK"}`

2. **Check backend logs** for errors:
   ```bash
   cargo run
   ```

3. **Check port conflicts:**
   ```bash
   lsof -i :3030
   ```

### Frontend shows empty library

1. **Scan your music directories first:**
   - Go to Settings in the app
   - Add music directories
   - Click "Scan Library"

2. **Or use API directly:**
   ```bash
   curl -X POST http://127.0.0.1:3030/api/library/scan \
     -H "Content-Type: application/json" \
     -d '{"directories": ["/path/to/your/music"]}'
   ```

### Connection refused errors

- Make sure Rust backend is running before starting Electron
- Check firewall settings (localhost:3030 should be accessible)
- Verify API_BASE_URL in `main.js` matches backend port (3030)

## API Documentation

The API includes interactive Swagger/OpenAPI documentation with full schema documentation.

### Access Swagger UI

1. **Start the backend**: `cargo run`
2. **Open Swagger UI**: Navigate to `http://127.0.0.1:3030/swagger-ui` in your browser

The Swagger UI provides:
- Complete API schema documentation
- Interactive API testing (try endpoints directly from the browser)
- Request/response schemas with examples
- Data type definitions

### OpenAPI Specification

- **JSON Spec**: `http://127.0.0.1:3030/api-doc/openapi.json`
- Can be imported into tools like Postman, Insomnia, or other API clients

## Testing the API

You can test the API directly using curl or use the Swagger UI:

```bash
# Health check
curl http://127.0.0.1:3030/api/health

# Get all tracks
curl http://127.0.0.1:3030/api/library/tracks

# Scan library
curl -X POST http://127.0.0.1:3030/api/library/scan \
  -H "Content-Type: application/json" \
  -d '{"directories": ["~/Music"]}'

# Search tracks
curl "http://127.0.0.1:3030/api/library/search?q=rock"
```

## Next Steps

- [ ] Add authentication/authorization if needed
- [ ] Add WebSocket support for real-time updates
- [ ] Implement playlist management endpoints
- [ ] Add audio playback control endpoints
- [ ] Add streaming support for audio files

