# VDownloader

A cross-platform desktop video downloader application built with Rust and GTK4.

## Status

🚧 **Under Development** - This project is being rewritten from scratch using Rust and GTK4.

The previous Node.js/Express web application has been removed to make way for a native desktop application that provides:

- **Native Performance**: Built with Rust for maximum speed and safety
- **Modern UI**: GTK4 provides a clean, native look on Linux, Windows, and macOS
- **Multi-Platform Support**: Download videos from YouTube, TikTok, X (Twitter), Instagram, and Reddit
- **Offline First**: Desktop application with no server dependencies

## Planned Features

- 🎥 **Multi-Platform Support**: Download from YouTube, TikTok, X (Twitter), Instagram, and Reddit
- 🖥️ **Native Desktop App**: GTK4-based GUI for Linux, Windows, and macOS
- 🦀 **Rust-Powered**: Fast, safe, and reliable video downloads
- 📊 **Download Queue**: Manage multiple simultaneous downloads
- 🎨 **Modern UI**: Clean, intuitive interface following GNOME HIG guidelines
- 📝 **Download History**: Track and manage previous downloads
- ⚡ **Fast & Efficient**: Async/await for non-blocking downloads

## Technology Stack

- **Language**: Rust 🦀
- **GUI Framework**: GTK4
- **Video Extraction**: yt-dlp (via youtube_dl Rust crate)
- **Async Runtime**: Tokio

### Video Extraction Library

After thorough research, we've selected the **`youtube_dl` Rust crate** (wrapper around yt-dlp CLI) as our video extraction solution.

**Why this choice?**
- ✅ Supports all target platforms (YouTube, TikTok, Twitter, Instagram, Reddit) plus 1000+ sites
- ✅ Leverages industry-standard yt-dlp (137K+ GitHub stars, actively maintained)
- ✅ MIT/Apache-2.0 license compatible
- ✅ Platform API changes handled automatically by yt-dlp maintainers
- ✅ Clean Rust API with async support

📖 **See [RESEARCH_VIDEO_EXTRACTION.md](./RESEARCH_VIDEO_EXTRACTION.md)** for detailed analysis and comparison of alternatives.

## Development

This project is in early development. The architecture and technology stack have been researched and documented.

### Prerequisites

**Development Tools:**
- Rust (latest stable version)
- GTK4 development libraries
- Cargo
- pkg-config

**Runtime Dependencies:**
- yt-dlp (Python CLI tool)

**Linux (Ubuntu/Debian):**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install GTK4 development libraries
sudo apt install libgtk-4-dev build-essential pkg-config

# Install yt-dlp
sudo apt install yt-dlp
# OR: pip3 install yt-dlp
```

**macOS:**
```bash
# Install dependencies via Homebrew
brew install rust gtk4 yt-dlp
```

**Windows:**
- Install Rust from https://rustup.rs/
- Install GTK4 via MSYS2
- Install yt-dlp: `pip install yt-dlp`

### Building

```bash
# Clone the repository
git clone https://github.com/st93642/VDownloader.git
cd VDownloader

# Build the project (when implemented)
cargo build --release

# Run the application (when implemented)
cargo run
```

### Documentation

- 📋 [Research Document](./RESEARCH_VIDEO_EXTRACTION.md) - Detailed library selection analysis
- 🏗️ [Architecture](./docs/ARCHITECTURE.md) - System design and structure
- 🚀 [Quick Start Guide](./docs/QUICK_START.md) - Development setup and examples

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! This is a fresh start, so there's plenty of opportunity to help shape the direction of the project.

## Acknowledgments

This project was previously a Node.js/Express web application. It has been completely rewritten to provide a better native desktop experience using Rust and GTK4.
