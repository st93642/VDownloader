# VDownloader

VDownloader is a cross-platform video downloading web application with a Node.js/Express backend and a modern, responsive frontend. This project provides a complete solution for downloading videos from multiple platforms including YouTube, TikTok, X (Twitter), Instagram, and Reddit.

## Features

- 🎥 **Multi-Platform Support**: Download from YouTube, TikTok, X (Twitter), Instagram, and Reddit
- 🔄 **Real-Time Progress**: WebSocket-based real-time download progress updates with polling fallback
- 📊 **Download Queue**: Manage multiple simultaneous downloads
- 📱 **Responsive Design**: Fully responsive UI optimized for desktop, tablet, and mobile
- 🎨 **Modern UI**: Clean, professional interface with gradient design
- 📝 **Download History**: Track and replay previous downloads
- 🔒 **Rate Limiting**: Built-in API rate limiting to prevent abuse
- 🐳 **Docker Ready**: Easy deployment with Docker and Docker Compose
- ⚡ **Fast & Efficient**: Optimized for performance with asynchronous operations

## Quick Start

### Prerequisites

- Node.js 18 or higher
- npm or yarn
- FFmpeg (optional, for advanced video processing)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/st93642/VDownloader.git
   cd VDownloader
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Configure environment variables**
   ```bash
   cp .env.example .env
   ```
   Edit `.env` and adjust values as needed (see [Configuration](#configuration) section)

4. **Start the development server**
   ```bash
   npm run dev
   ```

5. **Access the application**
   Open your browser and navigate to `http://localhost:4000`

### Production Deployment

For production, use:
```bash
npm start
```

## Docker Deployment

### Using Docker Compose (Recommended)

1. **Build and start the application**
   ```bash
   docker-compose up -d
   ```

2. **Access the application**
   Open your browser and navigate to `http://localhost:4000`

3. **Stop the application**
   ```bash
   docker-compose down
   ```

4. **View logs**
   ```bash
   docker-compose logs -f
   ```

### Using Docker directly

1. **Build the image**
   ```bash
   docker build -t vdownloader .
   ```

2. **Run the container**
   ```bash
   docker run -d \
     -p 4000:4000 \
     -e NODE_ENV=production \
     -e PORT=4000 \
     --name vdownloader-app \
     vdownloader
   ```

3. **Stop the container**
   ```bash
   docker stop vdownloader-app
   docker rm vdownloader-app
   ```

### Docker Environment Variables

You can customize the Docker deployment by creating a `.env` file or setting environment variables:

```bash
# Copy the Docker environment template
cp .env.docker .env

# Edit as needed
nano .env

# Start with environment variables
docker-compose up -d
```

## Project Structure

```
├── backend/
│   ├── adapters/              # Platform-specific video extractors
│   │   ├── baseAdapter.js     # Abstract base adapter
│   │   ├── adapterFactory.js  # Factory for creating adapters
│   │   ├── youtubeAdapter.js  # YouTube implementation
│   │   ├── tiktokAdapter.js   # TikTok implementation
│   │   ├── twitterAdapter.js  # Twitter/X implementation
│   │   ├── instagramAdapter.js # Instagram implementation
│   │   └── redditAdapter.js   # Reddit implementation
│   ├── controllers/           # Request handlers
│   ├── middleware/            # Express middleware
│   ├── routes/                # API routes
│   ├── services/              # Business logic
│   ├── utils/                 # Helper functions
│   ├── app.js                 # Express app setup
│   ├── server.js              # Server bootstrap
│   └── websocket.js           # WebSocket server setup
├── config/
│   ├── index.js               # Main configuration
│   └── platforms.js           # Platform definitions
├── frontend/
│   ├── index.html             # Main HTML file
│   ├── styles.css             # Styling
│   └── script.js              # Frontend application logic
├── .env.example               # Environment variables template
├── .env.docker                # Docker environment template
├── Dockerfile                 # Docker image definition
├── docker-compose.yml         # Docker Compose configuration
├── package.json               # Node.js dependencies
└── README.md                  # This file
```

## Configuration

Environment variables can be set in a `.env` file in the project root:

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `4000` | Port the Express server listens on |
| `NODE_ENV` | `development` | Runtime environment (`development`, `production`) |
| `APP_BASE_URL` | `http://localhost:4000` | Base URL for the application |
| `DOWNLOAD_TEMP_DIR` | `.tmp/downloads` | Temporary directory for downloaded files |
| `MAX_CONCURRENT_JOBS` | `2` | Maximum number of concurrent download jobs |

## API Documentation

### Base URL
```
http://localhost:4000/api
```

### Endpoints

#### Health & Platform Information

##### Get Service Health
```http
GET /api/health
```

**Response:**
```json
{
  "success": true,
  "data": {
    "status": "ok",
    "service": "VDownloader",
    "version": "0.1.0",
    "timestamp": "2024-01-01T00:00:00.000Z",
    "supportedPlatforms": ["youtube", "tiktok", "twitter", "instagram", "reddit"]
  }
}
```

##### List All Platforms
```http
GET /api/platforms
```

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

##### Get Platform Capabilities
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
      "enabled": true,
      "isSupported": true,
      "supports": ["video", "audio"],
      "qualityOptions": ["144p", "240p", "360p", "480p", "720p", "1080p"]
    }
  ]
}
```

#### Download Operations

##### Validate URL
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
      "duration": 213,
      "uploader": "Channel Name",
      "thumbnail": "https://..."
    },
    "supportedFormats": ["video", "audio"],
    "supportedQualities": ["144p", "240p", "360p", "480p", "720p", "1080p"]
  }
}
```

##### Initiate Download
```http
POST /api/download
Content-Type: application/json

{
  "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
  "format": "video",
  "quality": "720p"
}
```

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
    "createdAt": "2024-01-01T00:00:00.000Z"
  }
}
```

##### Get Download Status
```http
GET /api/status/:downloadId
```

**Response:**
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

##### Cancel Download
```http
DELETE /api/cancel/:downloadId
```

**Response:**
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

##### Get Video Metadata
```http
POST /api/metadata
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
    "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
    "platform": "youtube",
    "platformLabel": "YouTube",
    "metadata": {
      "title": "Video Title",
      "duration": 213,
      "uploader": "Channel Name",
      "description": "Video description...",
      "thumbnail": "https://..."
    }
  }
}
```

### Rate Limits

The API implements rate limiting to prevent abuse:

- **Download requests** (`POST /api/download`): 10 per hour
- **Validation requests** (`POST /api/validate`): 30 per minute
- **Status checks** (`GET /api/status/:id`): 100 per minute

### WebSocket Events

The application uses Socket.IO for real-time progress updates.

#### Client → Server

```javascript
// Subscribe to download updates
socket.emit('subscribe', downloadId);

// Unsubscribe from updates
socket.emit('unsubscribe', downloadId);
```

#### Server → Client

```javascript
// Progress update
socket.on('download:progress', (data) => {
  console.log(data);
  // {
  //   downloadId: "a1b2c3d4...",
  //   progress: 45,
  //   speed: 1024,
  //   bytesDownloaded: 5242880,
  //   totalBytes: 11534336,
  //   status: "downloading"
  // }
});

// Download complete
socket.on('download:complete', (data) => {
  console.log(data);
  // {
  //   downloadId: "a1b2c3d4...",
  //   status: "completed",
  //   completedAt: "2024-01-01T00:00:10.000Z"
  // }
});

// Download error
socket.on('download:error', (data) => {
  console.log(data);
  // {
  //   downloadId: "a1b2c3d4...",
  //   error: "Error message"
  // }
});
```

## Frontend Usage

The frontend is a complete Single Page Application (SPA) that integrates with the backend API:

1. **Platform Selection**: Choose from available platforms
2. **URL Input**: Enter the video URL
3. **Format Selection**: Choose between video or audio download
4. **Quality Selection**: Select quality based on platform capabilities
5. **Download**: Click download to start the process
6. **Progress Tracking**: Watch real-time progress with WebSocket updates
7. **History**: View and replay previous downloads

### Features

- **Real-time Updates**: WebSocket connection for instant progress updates
- **Polling Fallback**: Automatic fallback to HTTP polling if WebSockets unavailable
- **Multiple Downloads**: Support for simultaneous downloads with queue management
- **Persistent History**: Local storage of download history
- **Responsive Design**: Mobile-first design that works on all devices
- **Toast Notifications**: User-friendly success and error messages

## Supported Platforms

| Platform | Video | Audio | Quality Options |
|----------|-------|-------|-----------------|
| YouTube | ✅ | ✅ | 144p - 1080p |
| TikTok | ✅ | ✅ | 360p - 1080p |
| X (Twitter) | ✅ | ✅ | 360p - 1080p |
| Instagram | ✅ | ✅ | 360p - 1080p |
| Reddit | ✅ | ✅ | 360p - 1080p |

See [PLATFORM_INTEGRATIONS.md](./PLATFORM_INTEGRATIONS.md) for detailed platform documentation.

## Development

### Running in Development Mode

```bash
npm run dev
```

This starts the server with nodemon for automatic reloading on file changes.

### Project Dependencies

#### Core Dependencies
- **express** - Web server framework
- **socket.io** - WebSocket server for real-time updates
- **cors** - Cross-Origin Resource Sharing
- **dotenv** - Environment variable management
- **express-rate-limit** - API rate limiting

#### Platform Integration
- **@distube/ytdl-core** - YouTube video extraction
- **axios** - HTTP client for API requests
- **cheerio** - HTML parser for web scraping
- **node-fetch** - Modern fetch implementation

#### Media Processing
- **fluent-ffmpeg** - FFmpeg wrapper for media processing

### Adding New Platforms

To add support for a new platform:

1. Create a new adapter in `backend/adapters/` extending `BaseAdapter`
2. Implement required methods: `getVideoInfo()`, `getDownloadUrl()`, `getMetadata()`
3. Add platform configuration to `config/platforms.js`
4. Register the adapter in `backend/adapters/adapterFactory.js`

See existing adapters for reference implementation.

## Troubleshooting

### Common Issues

**Port already in use**
```bash
# The server will automatically try the next available port
# Or specify a different port in .env
PORT=4001
```

**WebSocket connection fails**
- Check that port 4000 (or your configured port) is accessible
- Ensure no firewall is blocking WebSocket connections
- The app will automatically fall back to HTTP polling

**Downloads fail**
- Check internet connectivity
- Verify the video URL is correct and accessible
- Some videos may be region-restricted or private
- Check server logs for detailed error messages

**Docker container won't start**
```bash
# Check logs
docker-compose logs -f

# Rebuild if needed
docker-compose down
docker-compose build --no-cache
docker-compose up -d
```

## Security Considerations

- Rate limiting is enabled by default to prevent API abuse
- CORS is configured to allow cross-origin requests
- Input validation is performed on all API endpoints
- Error messages are sanitized to prevent information leakage
- WebSocket connections are validated

## Performance Tips

- Use Docker for production deployments
- Adjust `MAX_CONCURRENT_JOBS` based on server capacity
- Monitor memory usage when handling multiple simultaneous downloads
- Consider implementing a Redis cache for frequently accessed metadata
- Use a reverse proxy (nginx, Apache) for SSL/TLS termination

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with Express.js and Socket.IO
- UI design inspired by modern web applications
- Video extraction powered by community-maintained libraries

## Support

For issues, questions, or contributions, please visit the [GitHub repository](https://github.com/st93642/VDownloader).

---

**Note**: This application is for educational purposes. Please respect copyright laws and platform terms of service when downloading content.
