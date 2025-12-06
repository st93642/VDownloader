# VDownloader Quick Start Guide

Get VDownloader up and running in minutes!

## 🚀 5-Minute Setup

### Option 1: Docker (Easiest)

```bash
# 1. Clone the repository
git clone https://github.com/st93642/VDownloader.git
cd VDownloader

# 2. Start with Docker Compose
docker-compose up -d

# 3. Open your browser
open http://localhost:4000
```

That's it! 🎉

### Option 2: Node.js

```bash
# 1. Clone the repository
git clone https://github.com/st93642/VDownloader.git
cd VDownloader

# 2. Install dependencies
npm install

# 3. Start the application
npm start

# 4. Open your browser
open http://localhost:4000
```

## 📖 Basic Usage

1. **Select a platform** (YouTube, TikTok, Twitter, Instagram, or Reddit)
2. **Paste the video URL**
3. **Choose format** (Video or Audio)
4. **Select quality** (144p - 1080p depending on platform)
5. **Click Download**
6. **Watch the progress** in real-time!

## 🎯 Example URLs

Try these to test the application:

- **YouTube**: `https://www.youtube.com/watch?v=dQw4w9WgXcQ`
- **TikTok**: `https://www.tiktok.com/@username/video/123456789`
- **Twitter**: `https://twitter.com/username/status/123456789`
- **Instagram**: `https://www.instagram.com/reel/ABC123/`
- **Reddit**: `https://www.reddit.com/r/videos/comments/abc123/`

## 🔧 Configuration (Optional)

Create a `.env` file to customize settings:

```env
PORT=4000
NODE_ENV=development
APP_BASE_URL=http://localhost:4000
MAX_CONCURRENT_JOBS=2
```

## 🐛 Troubleshooting

### Port Already in Use

The app automatically tries the next available port. Check the console output.

### Can't Connect to WebSocket

The app automatically falls back to HTTP polling. No action needed!

### Download Fails

- Check your internet connection
- Verify the URL is correct and accessible
- Some videos may be region-restricted or private

## 📚 Learn More

- **Full Documentation**: See [README.md](./README.md)
- **API Documentation**: See [API.md](./API.md)
- **Deployment Guide**: See [DEPLOYMENT.md](./DEPLOYMENT.md)
- **Platform Details**: See [PLATFORM_INTEGRATIONS.md](./PLATFORM_INTEGRATIONS.md)

## 🤝 Need Help?

- [GitHub Issues](https://github.com/st93642/VDownloader/issues)
- [Documentation](https://github.com/st93642/VDownloader)

---

**Happy Downloading! 🎉**
