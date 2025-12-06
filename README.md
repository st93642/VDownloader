# VDownloader

VDownloader is a cross-platform video downloading web application with a Node.js/Express backend and a modern, responsive frontend. This project provides a complete solution for downloading videos from multiple platforms including YouTube, TikTok, X (Twitter), Instagram, and Reddit.

## Project Structure

```
├── .env.example
├── .gitignore
├── backend
│   ├── adapters
│   │   ├── baseAdapter.js           # Abstract base adapter class
│   │   ├── adapterFactory.js       # Factory for creating platform adapters
│   │   ├── youtubeAdapter.js       # YouTube video extraction
│   │   ├── tiktokAdapter.js        # TikTok video extraction
│   │   ├── twitterAdapter.js       # X/Twitter video extraction
│   │   ├── instagramAdapter.js     # Instagram video extraction
│   │   └── redditAdapter.js        # Reddit video extraction
│   ├── app.js
│   ├── controllers
│   │   ├── downloadController.js    # Download management endpoints
│   │   ├── healthController.js
│   │   └── platformController.js
│   ├── middleware
│   │   ├── errorHandler.js          # Centralized error handling
│   │   └── rateLimiter.js           # Rate limiting middleware
│   ├── routes
│   │   ├── downloadRoutes.js        # Download API routes
│   │   ├── healthRoutes.js
│   │   ├── index.js
│   │   └── platformRoutes.js
│   ├── server.js
│   ├── services
│   │   ├── downloadService.js       # Download tracking service
│   │   ├── platformService.js       # Platform configuration service
│   │   └── platformDownloadService.js # Platform download operations
│   └── utils
│       └── urlValidator.js          # URL validation utilities
├── config
│   ├── index.js
│   └── platforms.js
├── frontend
│   ├── index.html      # Main frontend application
│   ├── styles.css      # Complete CSS styling
│   ├── script.js       # JavaScript application logic
│   └── README.md       # Frontend documentation
├── LICENSE
├── PLATFORM_INTEGRATIONS.md # Multi-platform integration documentation
├── README.md
├── package.json
└── package-lock.json
```

## Key Dependencies

- **Express** – HTTP server framework used to build the REST API.
- **cors** – Enables controlled Cross-Origin Resource Sharing for the frontend.
- **dotenv** – Loads environment variables from `.env` files for local development.
- **@distube/ytdl-core** – Provides YouTube download and metadata capabilities.
- **axios** – HTTP client for web scraping and API requests.
- **cheerio** – HTML parser for web scraping platforms.
- **node-fetch** – Modern fetch implementation for HTTP requests.
- **fluent-ffmpeg** – Adapter around FFmpeg for advanced media processing.
- **express-rate-limit** – Middleware for rate limiting API requests to prevent abuse.
- **nodemon** – Development dependency that reloads the server as files change.

## Getting Started

1. **Install dependencies**
   ```bash
   npm install
   ```
2. **Run the development server**
   ```bash
   npm run dev
   ```
3. **Run the production server**
   ```bash
   npm start
   ```

The API will default to `http://localhost:4000` unless the `PORT` environment variable is set.

## Frontend Development

The frontend is a complete, responsive web application that works with the backend API. To run the frontend:

1. **Start the backend server** (required for API endpoints):
   ```bash
   npm run dev
   ```

2. **Access the frontend**: Open `http://localhost:4000` in your browser

The frontend includes:
- Modern, responsive UI with gradient design
- Multi-platform support (YouTube, TikTok, X, Instagram, Reddit)
- Real-time download progress tracking
- Download history management
- Format and quality selection
- Mobile-optimized interface
- Metadata extraction for all supported platforms

For detailed frontend documentation, see `frontend/README.md`.

## Configuration

Environment variables are loaded with [dotenv](https://github.com/motdotla/dotenv). Duplicate `.env.example` to `.env` and adjust as needed. You may supply any of the following:

| Variable | Default | Description |
| --- | --- | --- |
| `PORT` | `4000` | Port the Express server listens on |
| `NODE_ENV` | `development` | Runtime environment label |
| `APP_BASE_URL` | `http://localhost:4000` | External URL for generated links |
| `DOWNLOAD_TEMP_DIR` | `.tmp/downloads` | Temporary directory for downloaded files |
| `MAX_CONCURRENT_JOBS` | `2` | Limits concurrent download jobs |

Supported platforms and their metadata are centralized in `config/platforms.js`. Toggle the `enabled` flag or extend the list to allow additional sources.

## API Routes

### Health & Platform Information
| Method | Route | Description |
| --- | --- | --- |
| `GET` | `/api/health` | Returns service health metadata and enabled platforms |
| `GET` | `/api/platforms` | Lists all configured platforms |
| `GET` | `/api/platforms/supported` | Lists only currently enabled platforms |
| `GET` | `/api/platforms/capabilities` | Lists platform capabilities and support status |

### Download Management
| Method | Route | Description |
| --- | --- | --- |
| `POST` | `/api/validate` | Validate a video URL and extract metadata |
| `POST` | `/api/download` | Initiate a new video download |
| `GET` | `/api/status/:downloadId` | Check the status of an ongoing/completed download |
| `DELETE` | `/api/cancel/:downloadId` | Cancel an ongoing download |
| `GET` | `/api/formats/:platform` | Get available formats and quality options for a platform |
| `POST` | `/api/metadata` | Extract metadata for a video URL |

### Rate Limiting
- **Download requests**: 10 per hour (prevents abuse)
- **Validation requests**: 30 per minute (allows URL checking)
- **Status checks**: 100 per minute (allows frequent polling)

## Frontend Features

The complete frontend application provides:

### 🎯 Core Functionality
- **Multi-platform Support**: Download from YouTube, TikTok, X, Instagram, and more
- **Format Selection**: Choose between video and audio-only downloads
- **Quality Options**: Select from various quality presets (144p to 1080p)
- **Smart URL Validation**: Automatic platform detection and URL validation
- **Real-time Progress**: Live download progress with speed and time estimates

### 🎨 User Interface
- **Modern Design**: Clean, professional interface with gradient backgrounds
- **Responsive Layout**: Fully responsive design for mobile, tablet, and desktop
- **Interactive Elements**: Smooth animations and hover effects
- **Status Indicators**: Clear visual feedback for all actions

### 📊 Download Management
- **Download History**: Persistent history of recent downloads (stored locally)
- **Progress Tracking**: Real-time progress bars with percentage completion
- **Speed Monitoring**: Display current download speed and estimated time remaining
- **Download Again**: Quick re-download option from history
