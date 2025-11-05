# Prototype Status Notes

## Frontend Mock Data

The frontend interface is currently using **mock/placeholder data** for UI development and prototyping. This means the music tracks, artists, and albums you see in the interface are not real and do not come from your actual music library.

### Files Using Mock Data

1. **`renderer/pages/Library.js`**
   - Uses hardcoded `mockTracks` array with example tracks (Queen, Eagles, etc.)
   - Line 69: `const [tracks, setTracks] = useState(mockTracks);`
   - These tracks are not connected to the actual backend

2. **`main.js` (IPC Handlers)**
   - `scan-library` handler returns mock tracks
   - `get-library` handler returns mock tracks
   - `get-library-tracks` handler returns mock tracks
   - All marked with `⚠️ PROTOTYPE` comments

3. **Other Pages**
   - `NowPlaying.js` uses mock track data
   - `Playlists.js` uses mock playlist data

## Backend Status

The **Rust backend** has fully implemented library functionality:

✅ **Implemented Features:**
- Library scanning (`Library::scan_directories()`)
- Cache system (`load_from_cache()`, `save_to_cache()`)
- Playlist cleanup (`cleanup_missing_tracks()`)
- Track metadata extraction (ID3 tags, file info)

❌ **Missing:**
- Connection between Electron main process and Rust backend
- IPC/HTTP API for frontend to request library data
- Real-time communication between frontend and backend

## Next Steps to Connect Real Data

1. **Create IPC Bridge**: Set up communication between Electron and Rust backend
   - Options: HTTP server, stdio communication, or shared file communication
   - Or integrate Rust directly into Electron process

2. **Update IPC Handlers**: Replace mock data returns with actual backend calls
   - `scan-library` → Call `Library::scan_directories()`
   - `get-library` → Call `Library::get_tracks()`
   - `search-library` → Call `Library::search_tracks()`

3. **Update Frontend**: Replace mock data with actual API calls
   - Remove hardcoded `mockTracks` in `Library.js`
   - Load data from `window.electronAPI.getLibrary()`
   - Update all components to use real data

## Current Behavior

When you see tracks like:
- "Bohemian Rhapsody" by Queen
- "Hotel California" by Eagles
- "Sample Track 1" by Sample Artist

These are **prototype/placeholder data** and do not represent your actual music library.

## Status

🔴 **Frontend**: Using mock data (prototype)
🟢 **Backend**: Fully implemented (ready to use)
🟡 **Integration**: Not yet connected

