const artworkCache = new Map();

const LAST_FM_IMAGE_PRIORITY = ['mega', 'extralarge', 'large', 'medium', 'small'];

const selectLastFmImage = (images = []) => {
  if (!Array.isArray(images)) return null;
  for (const size of LAST_FM_IMAGE_PRIORITY) {
    const match = images.find((image) => image?.size === size && image['#text']);
    if (match?.['#text']) {
      return match['#text'];
    }
  }
  return null;
};

const buildSearchQuery = (track) => {
  const parts = [];
  if (track.artist && track.artist !== 'Unknown Artist') {
    parts.push(track.artist);
  }
  if (track.album && track.album !== 'Unknown Album') {
    parts.push(track.album);
  }
  if (parts.length === 0 && track.title) {
    parts.push(track.title);
  }
  return parts.join(' ');
};

const buildLastFmUrl = (method, params, apiKey) => {
  const searchParams = new URLSearchParams({
    method,
    api_key: apiKey,
    format: 'json',
    ...params,
  });
  return `https://ws.audioscrobbler.com/2.0/?${searchParams.toString()}`;
};

const fetchFromLastFm = async (track, apiKey) => {
  if (!apiKey || !track?.artist) {
    return null;
  }

  const attempts = [];

  if (track.album && track.album !== 'Unknown Album') {
    attempts.push({
      url: buildLastFmUrl(
        'album.getinfo',
        { artist: track.artist, album: track.album },
        apiKey
      ),
      extract: (data) => selectLastFmImage(data?.album?.image),
    });
  }

  if (track.title) {
    attempts.push({
      url: buildLastFmUrl(
        'track.getInfo',
        { artist: track.artist, track: track.title },
        apiKey
      ),
      extract: (data) => selectLastFmImage(data?.track?.album?.image),
    });
  }

  // Fallback to search endpoint if nothing else is available
  attempts.push({
    url: buildLastFmUrl(
      'track.search',
      { track: buildSearchQuery(track) || track.title || '' },
      apiKey
    ),
    extract: (data) =>
      selectLastFmImage(
        data?.results?.trackmatches?.track?.[0]?.image ||
          data?.results?.trackmatches?.track?.image
      ),
  });

  for (const attempt of attempts) {
    try {
      const response = await fetch(attempt.url);
      if (!response.ok) {
        continue;
      }
      const data = await response.json();
      if (data?.error) {
        continue;
      }

      const imageUrl = attempt.extract(data);
      if (imageUrl) {
        return imageUrl;
      }
    } catch (error) {
      console.warn('[artwork] Failed to fetch from Last.fm:', error);
    }
  }

  return null;
};

export async function fetchArtwork(track, options = {}) {
  if (typeof window === 'undefined' || !window.navigator?.onLine) {
    return null;
  }

  const cacheKey = `${track.artist || ''}|${track.album || ''}|${track.title || ''}`.toLowerCase();
  if (artworkCache.has(cacheKey)) {
    return artworkCache.get(cacheKey);
  }

  let artwork = null;

  // Source priority list
  const sources = [() => fetchFromLastFm(track, options.lastfmApiKey)];

  for (const fetcher of sources) {
    try {
      const result = await fetcher();
      if (result) {
        artwork = result;
        break;
      }
    } catch (error) {
      console.warn('[artwork] Source error:', error);
    }
  }

  artworkCache.set(cacheKey, artwork);
  return artwork;
}

export async function enrichTracksWithArtwork(tracks, limit = 40, options = {}) {
  if (!Array.isArray(tracks) || tracks.length === 0) {
    return [];
  }

  const enriched = [];
  let fetchedCount = 0;

  for (const track of tracks) {
    if (track.artwork || fetchedCount >= limit) {
      enriched.push(track);
      continue;
    }

    const artwork = await fetchArtwork(track, options);
    if (artwork) {
      fetchedCount += 1;
      enriched.push({ ...track, artwork });
    } else {
      enriched.push(track);
    }
  }

  return enriched;
}

export function clearArtworkCache() {
  artworkCache.clear();
}
