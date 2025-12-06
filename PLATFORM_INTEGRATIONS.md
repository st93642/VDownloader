# Multi-Platform Video Downloader Integration

This document outlines the implementation of multi-platform video downloading support in VDownloader.

## Supported Platforms

### 1. YouTube
- **Domains**: `youtube.com`, `youtu.be`
- **Formats**: Video, Audio
- **Qualities**: 144p, 240p, 360p, 480p, 720p, 1080p
- **Library**: `@distube/ytdl-core`

### 2. TikTok
- **Domains**: `tiktok.com`, `vm.tiktok.com`
- **Formats**: Video, Audio
- **Qualities**: 360p, 480p, 720p, 1080p
- **Method**: Web scraping with Cheerio

### 3. X/Twitter
- **Domains**: `twitter.com`, `x.com`
- **Formats**: Video, Audio
- **Qualities**: 360p, 480p, 720p, 1080p
- **Method**: Web scraping with Cheerio

### 4. Instagram
- **Domains**: `instagram.com`, `instagr.am`
- **Formats**: Video, Audio
- **Qualities**: 360p, 480p, 720p, 1080p
- **Method**: Web scraping with Cheerio

### 5. Reddit
- **Domains**: `reddit.com`, `redd.it`
- **Formats**: Video, Audio
- **Qualities**: 360p, 480p, 720p, 1080p
- **Method**: Web scraping with Cheerio

## Architecture

### Adapter Pattern
The system uses an adapter pattern to handle different platforms:

```
BaseAdapter (Abstract)
├── YouTubeAdapter
├── TikTokAdapter
├── TwitterAdapter
├── InstagramAdapter
└── RedditAdapter
```

### Key Components

1. **BaseAdapter** (`backend/adapters/baseAdapter.js`)
   - Abstract base class defining the interface
   - Common utility methods
   - Error handling patterns

2. **Platform Adapters** (`backend/adapters/`)
   - Platform-specific implementations
   - Metadata extraction
   - Download URL resolution
   - Stream handling

3. **AdapterFactory** (`backend/adapters/adapterFactory.js`)
   - Factory pattern for adapter instantiation
   - Platform validation
   - Supported platform management

4. **PlatformDownloadService** (`backend/services/platformDownloadService.js`)
   - High-level service for platform operations
   - Error handling and validation
   - Interface for controllers

## API Endpoints

### Validate URL with Metadata
```http
POST /api/validate
Content-Type: application/json

{
  "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "valid": true,
    "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
    "platform": "youtube",
    "platformLabel": "YouTube",
    "metadata": {
      "title": "Video Title",
      "duration": 210,
      "uploader": "Channel Name",
      "description": "Video description",
      "thumbnail": "https://...",
      "viewCount": 1234567,
      "uploadDate": "2023-01-01T00:00:00.000Z",
      "videoId": "dQw4w9WgXcQ"
    },
    "supportedFormats": ["video", "audio"],
    "supportedQualities": ["144p", "240p", "360p", "480p", "720p", "1080p"]
  }
}
```

### Get Metadata Only
```http
POST /api/metadata
Content-Type: application/json

{
  "url": "https://www.tiktok.com/@user/video/1234567890"
}
```

### Get Platform Capabilities
```http
GET /api/platforms/capabilities
```

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "key": "youtube",
      "label": "YouTube",
      "domains": ["youtube.com", "youtu.be"],
      "enabled": true,
      "downloader": "ytdl-core",
      "supports": ["video", "audio"],
      "qualityOptions": ["144p", "240p", "360p", "480p", "720p", "1080p"],
      "isSupported": true
    },
    {
      "key": "tiktok",
      "label": "TikTok",
      "domains": ["tiktok.com", "vm.tiktok.com"],
      "enabled": true,
      "downloader": "tiktok",
      "supports": ["video", "audio"],
      "qualityOptions": ["360p", "480p", "720p", "1080p"],
      "isSupported": true
    }
  ]
}
```

## Error Handling

The system implements comprehensive error handling for:

1. **Invalid URLs** - URL format validation
2. **Unsupported Platforms** - Platform not recognized or not enabled
3. **Region Restrictions** - Content not available in user's region
4. **Private/Deleted Content** - Video not accessible
5. **Network Issues** - Connection problems
6. **Parsing Errors** - Changes in platform HTML structure

### Error Response Format
```json
{
  "success": false,
  "error": {
    "message": "Failed to extract metadata: Video not found",
    "code": "METADATA_EXTRACTION_ERROR"
  }
}
```

## Adding New Platforms

To add support for a new platform:

1. **Add Platform Configuration**
   ```javascript
   // config/platforms.js
   {
     key: "newplatform",
     label: "New Platform",
     domains: ["newplatform.com"],
     enabled: true,
     downloader: "newplatform",
     supports: ["video", "audio"],
     qualityOptions: ["360p", "480p", "720p", "1080p"]
   }
   ```

2. **Create Adapter**
   ```javascript
   // backend/adapters/newPlatformAdapter.js
   class NewPlatformAdapter extends BaseAdapter {
     extractVideoId(url) { /* implementation */ }
     async getMetadata(url) { /* implementation */ }
     async getDownloadInfo(url, format, quality) { /* implementation */ }
     async getStream(url, format, quality) { /* implementation */ }
   }
   ```

3. **Register Adapter**
   ```javascript
   // backend/adapters/adapterFactory.js
   static adapters = new Map([
     // ... existing adapters
     ['newplatform', NewPlatformAdapter]
   ]);
   ```

## Rate Limiting

Different rate limits are applied to prevent abuse:
- **Download requests**: 10 per hour
- **Validation requests**: 30 per minute  
- **Status checks**: 100 per minute

## Dependencies

Key dependencies added for multi-platform support:
- `@distube/ytdl-core` - YouTube video extraction
- `axios` - HTTP requests for web scraping
- `cheerio` - HTML parsing for web scraping
- `node-fetch` - Modern fetch implementation

## Security Considerations

1. **Input Validation** - All URLs are validated before processing
2. **Rate Limiting** - Prevents abuse and API overuse
3. **User-Agent Spoofing** - Mimics browser requests to avoid blocking
4. **Error Sanitization** - Sensitive information is not exposed in error messages

## Testing

The implementation includes comprehensive error handling and validation. Each platform adapter is tested to ensure:
- Proper URL parsing
- Metadata extraction
- Download URL resolution
- Error handling for edge cases

## Future Enhancements

Potential future improvements:
1. **Authentication Support** - For private content
2. **Batch Downloads** - Multiple videos at once
3. **Quality Selection** - Automatic quality selection based on bandwidth
4. **Progress Tracking** - Real-time download progress
5. **Caching** - Metadata caching for repeated requests
6. **Webhook Support** - Download completion notifications