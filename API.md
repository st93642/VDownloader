# VDownloader API Documentation

Complete API reference for the VDownloader backend service.

## Base URL

```
http://localhost:4000/api
```

## Authentication

Currently, the API does not require authentication. Rate limiting is applied to prevent abuse.

## Response Format

All API responses follow a consistent format:

### Success Response
```json
{
  "success": true,
  "data": { ... }
}
```

### Error Response
```json
{
  "success": false,
  "error": {
    "message": "Error description",
    "code": "ERROR_CODE"
  }
}
```

## Error Codes

| Code | Description |
|------|-------------|
| `MISSING_URL` | URL parameter is required but not provided |
| `INVALID_URL` | Provided URL is not valid or not supported |
| `INVALID_FORMAT` | Format must be 'video' or 'audio' |
| `PLATFORM_NOT_SUPPORTED` | Requested platform is not supported |
| `DOWNLOAD_NOT_FOUND` | Download ID does not exist |
| `INVALID_STATE` | Operation not allowed in current state |
| `VALIDATION_ERROR` | URL validation failed |
| `METADATA_ERROR` | Failed to extract metadata |
| `DOWNLOAD_INIT_ERROR` | Failed to initiate download |
| `RATE_LIMIT_EXCEEDED` | Too many requests |

## HTTP Status Codes

| Status | Description |
|--------|-------------|
| `200` | Success |
| `202` | Accepted (async operation started) |
| `400` | Bad request (validation error) |
| `404` | Resource not found |
| `429` | Too many requests (rate limit) |
| `500` | Internal server error |

## Endpoints

### Health Check

#### `GET /api/health`

Get service health status and metadata.

**Response:**
```json
{
  "success": true,
  "data": {
    "status": "ok",
    "service": "VDownloader",
    "version": "0.1.0",
    "timestamp": "2024-01-01T00:00:00.000Z",
    "supportedPlatforms": [
      "youtube",
      "tiktok",
      "twitter",
      "instagram",
      "reddit"
    ]
  }
}
```

### Platform Information

#### `GET /api/platforms`

List all configured platforms.

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "key": "youtube",
      "label": "YouTube",
      "enabled": true,
      "supports": ["video", "audio"],
      "domains": ["youtube.com", "youtu.be"],
      "qualityOptions": ["144p", "240p", "360p", "480p", "720p", "1080p"]
    }
  ]
}
```

#### `GET /api/platforms/supported`

List only enabled platforms.

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "key": "youtube",
      "label": "YouTube",
      "enabled": true,
      "supports": ["video", "audio"],
      "domains": ["youtube.com", "youtu.be"],
      "qualityOptions": ["144p", "240p", "360p", "480p", "720p", "1080p"]
    }
  ]
}
```

#### `GET /api/platforms/capabilities`

Get platform capabilities and support status.

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "key": "youtube",
      "label": "YouTube",
      "enabled": true,
      "isSupported": true,
      "supports": ["video", "audio"],
      "qualityOptions": ["144p", "240p", "360p", "480p", "720p", "1080p"]
    }
  ]
}
```

#### `GET /api/formats/:platform`

Get available formats and quality options for a specific platform.

**Parameters:**
- `platform` (path) - Platform key (e.g., "youtube")

**Response:**
```json
{
  "success": true,
  "data": {
    "platform": "youtube",
    "label": "YouTube",
    "supports": ["video", "audio"],
    "qualityOptions": ["144p", "240p", "360p", "480p", "720p", "1080p"]
  }
}
```

**Error Response (404):**
```json
{
  "success": false,
  "error": {
    "message": "Platform 'unknown' is not supported",
    "code": "PLATFORM_NOT_SUPPORTED"
  }
}
```

### URL Validation

#### `POST /api/validate`

Validate a video URL and extract metadata.

**Request Body:**
```json
{
  "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
}
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "valid": true,
    "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
    "platform": "youtube",
    "platformLabel": "YouTube",
    "metadata": {
      "title": "Rick Astley - Never Gonna Give You Up",
      "duration": 213,
      "uploader": "Rick Astley",
      "description": "The official video for...",
      "thumbnail": "https://i.ytimg.com/vi/dQw4w9WgXcQ/maxresdefault.jpg"
    },
    "supportedFormats": ["video", "audio"],
    "supportedQualities": ["144p", "240p", "360p", "480p", "720p", "1080p"]
  }
}
```

**Error Response (400):**
```json
{
  "success": false,
  "error": {
    "message": "URL is required",
    "code": "MISSING_URL"
  }
}
```

**Rate Limit:** 30 requests per minute

### Metadata Extraction

#### `POST /api/metadata`

Extract metadata for a video URL without validation.

**Request Body:**
```json
{
  "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
}
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
    "platform": "youtube",
    "platformLabel": "YouTube",
    "metadata": {
      "title": "Rick Astley - Never Gonna Give You Up",
      "duration": 213,
      "uploader": "Rick Astley",
      "description": "The official video for...",
      "thumbnail": "https://i.ytimg.com/vi/dQw4w9WgXcQ/maxresdefault.jpg"
    }
  }
}
```

**Rate Limit:** 30 requests per minute

### Download Management

#### `POST /api/download`

Initiate a new download.

**Request Body:**
```json
{
  "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
  "format": "video",
  "quality": "720p"
}
```

**Parameters:**
- `url` (required) - Video URL to download
- `format` (optional) - "video" or "audio" (default: "video")
- `quality` (optional) - Quality option (e.g., "720p")

**Response (202 Accepted):**
```json
{
  "success": true,
  "data": {
    "downloadId": "a1b2c3d4e5f6g7h8",
    "status": "pending",
    "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
    "format": "video",
    "platform": "youtube",
    "quality": "720p",
    "downloadInfo": {
      "title": "Rick Astley - Never Gonna Give You Up",
      "thumbnail": "https://..."
    },
    "createdAt": "2024-01-01T00:00:00.000Z"
  }
}
```

**Error Response (400):**
```json
{
  "success": false,
  "error": {
    "message": "Invalid format. Must be 'video' or 'audio'",
    "code": "INVALID_FORMAT"
  }
}
```

**Rate Limit:** 10 requests per hour

#### `GET /api/status/:downloadId`

Check the status of a download.

**Parameters:**
- `downloadId` (path) - Download ID returned from POST /api/download

**Response (200):**
```json
{
  "success": true,
  "data": {
    "downloadId": "a1b2c3d4e5f6g7h8",
    "status": "downloading",
    "progress": 45,
    "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
    "format": "video",
    "platform": "youtube",
    "createdAt": "2024-01-01T00:00:00.000Z",
    "startedAt": "2024-01-01T00:00:01.000Z",
    "completedAt": null,
    "error": null
  }
}
```

**Status Values:**
- `pending` - Download queued but not started
- `downloading` - Download in progress
- `completed` - Download finished successfully
- `failed` - Download failed (check `error` field)
- `cancelled` - Download was cancelled

**Error Response (404):**
```json
{
  "success": false,
  "error": {
    "message": "Download not found",
    "code": "DOWNLOAD_NOT_FOUND"
  }
}
```

**Rate Limit:** 100 requests per minute

#### `DELETE /api/cancel/:downloadId`

Cancel an ongoing download.

**Parameters:**
- `downloadId` (path) - Download ID to cancel

**Response (200):**
```json
{
  "success": true,
  "data": {
    "downloadId": "a1b2c3d4e5f6g7h8",
    "status": "cancelled",
    "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
    "cancelledAt": "2024-01-01T00:00:05.000Z"
  }
}
```

**Error Response (400):**
```json
{
  "success": false,
  "error": {
    "message": "Cannot cancel a completed download",
    "code": "INVALID_STATE"
  }
}
```

**Rate Limit:** 10 requests per hour

## WebSocket API

The application uses Socket.IO for real-time progress updates.

### Connection

```javascript
const socket = io('http://localhost:4000');
```

### Events

#### Client → Server

##### Subscribe to Download Updates
```javascript
socket.emit('subscribe', downloadId);
```

##### Unsubscribe from Download Updates
```javascript
socket.emit('unsubscribe', downloadId);
```

#### Server → Client

##### Connection Events
```javascript
socket.on('connect', () => {
  console.log('Connected to WebSocket');
});

socket.on('disconnect', () => {
  console.log('Disconnected from WebSocket');
});
```

##### Progress Update
```javascript
socket.on('download:progress', (data) => {
  // {
  //   downloadId: "a1b2c3d4e5f6g7h8",
  //   progress: 45,           // Percentage (0-100)
  //   speed: 1024,            // KB/s
  //   bytesDownloaded: 5242880,
  //   totalBytes: 11534336,
  //   status: "downloading"
  // }
});
```

##### Download Complete
```javascript
socket.on('download:complete', (data) => {
  // {
  //   downloadId: "a1b2c3d4e5f6g7h8",
  //   status: "completed",
  //   completedAt: "2024-01-01T00:00:10.000Z",
  //   downloadInfo: { ... }
  // }
});
```

##### Download Error
```javascript
socket.on('download:error', (data) => {
  // {
  //   downloadId: "a1b2c3d4e5f6g7h8",
  //   error: "Video is unavailable"
  // }
});
```

## Rate Limiting

The API implements rate limiting on a per-IP basis:

| Endpoint Type | Limit | Window |
|--------------|-------|--------|
| Download operations | 10 requests | 1 hour |
| Validation operations | 30 requests | 1 minute |
| Status checks | 100 requests | 1 minute |

When rate limit is exceeded, the API returns:

**Response (429):**
```json
{
  "success": false,
  "error": {
    "message": "Too many requests, please try again later",
    "code": "RATE_LIMIT_EXCEEDED"
  }
}
```

## Example Usage

### JavaScript/Fetch

```javascript
// Validate URL
const validateUrl = async (url) => {
  const response = await fetch('http://localhost:4000/api/validate', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ url })
  });
  return response.json();
};

// Initiate download
const startDownload = async (url, format, quality) => {
  const response = await fetch('http://localhost:4000/api/download', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ url, format, quality })
  });
  return response.json();
};

// Check status
const checkStatus = async (downloadId) => {
  const response = await fetch(`http://localhost:4000/api/status/${downloadId}`);
  return response.json();
};

// WebSocket connection
const socket = io('http://localhost:4000');
socket.on('connect', () => {
  console.log('Connected');
  socket.emit('subscribe', downloadId);
});
socket.on('download:progress', (data) => {
  console.log(`Progress: ${data.progress}%`);
});
```

### cURL

```bash
# Validate URL
curl -X POST http://localhost:4000/api/validate \
  -H "Content-Type: application/json" \
  -d '{"url":"https://www.youtube.com/watch?v=dQw4w9WgXcQ"}'

# Initiate download
curl -X POST http://localhost:4000/api/download \
  -H "Content-Type: application/json" \
  -d '{"url":"https://www.youtube.com/watch?v=dQw4w9WgXcQ","format":"video","quality":"720p"}'

# Check status
curl http://localhost:4000/api/status/a1b2c3d4e5f6g7h8

# Cancel download
curl -X DELETE http://localhost:4000/api/cancel/a1b2c3d4e5f6g7h8
```

### Python

```python
import requests

# Validate URL
response = requests.post('http://localhost:4000/api/validate', json={
    'url': 'https://www.youtube.com/watch?v=dQw4w9WgXcQ'
})
data = response.json()

# Initiate download
response = requests.post('http://localhost:4000/api/download', json={
    'url': 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
    'format': 'video',
    'quality': '720p'
})
download_data = response.json()
download_id = download_data['data']['downloadId']

# Check status
response = requests.get(f'http://localhost:4000/api/status/{download_id}')
status = response.json()
```

## Best Practices

1. **Always validate URLs** before initiating downloads
2. **Use WebSockets** for real-time progress updates when possible
3. **Implement polling fallback** if WebSocket connection fails
4. **Handle rate limits** gracefully with exponential backoff
5. **Check metadata** before download to show information to users
6. **Store download IDs** for status tracking and history
7. **Handle errors** appropriately and show user-friendly messages

## Changelog

### Version 0.1.0
- Initial API release
- Multi-platform support (YouTube, TikTok, Twitter, Instagram, Reddit)
- WebSocket real-time progress updates
- Rate limiting
- Metadata extraction
- Download queue management
