# VDownloader

A cross-platform desktop video downloader application built with Rust and GTK4.

## Planned Features

- 🎥 **Multi-Platform Support**: Download from YouTube, TikTok, X (Twitter), VK Video, Rutube, Instagram, and Reddit
- 🖥️ **Native Desktop App**: GTK4-based GUI for Linux, Windows, and macOS
- 🦀 **Rust-Powered**: Fast, safe, and reliable video downloads
- 📊 **Download Queue**: Manage multiple simultaneous downloads
- 🎨 **Modern UI**: Clean, intuitive interface following GNOME HIG guidelines
- 📝 **Download History**: Track and manage previous downloads
- ⚡ **Fast & Efficient**: Async/await for non-blocking downloads

## Tested Platforms

The following platforms have been verified to work:

- ✅ YouTube
- ✅ X (Twitter)
- ✅ VK Video
- ✅ Rutube
- ⏳ TikTok (In Progress)
- ⏳ Instagram (In Progress)
- ⏳ Reddit (In Progress)

## Technology Stack

- **Language**: Rust 🦀
- **GUI Framework**: GTK4
- **Video Extraction**: yt-dlp (via youtube_dl Rust crate)
- **Async Runtime**: Tokio

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

- Install Rust from <https://rustup.rs/>
- Install GTK4 via MSYS2
- Install yt-dlp: `pip install yt-dlp`

### Building and Running

```bash
# Clone the repository
git clone https://github.com/st93642/VDownloader.git
cd VDownloader

# Build the project
cargo build --release

# Run the application
cargo run

# Run with logging enabled
RUST_LOG=info cargo run

# Run tests
cargo test

# Check code formatting
cargo fmt --check

# Run linter
cargo clippy
```

**Note:** Make sure you have installed GTK4 development libraries before building. See Prerequisites section above.

### Project Structure

```text
VDownloader/
├── src/
│   ├── main.rs              # Application entry point
│   ├── ui/                  # GTK4 user interface
│   │   ├── mod.rs
│   │   ├── window.rs        # Main application window
│   │   └── components/      # Reusable UI components
│   │       ├── mod.rs
│   │       └── download_queue.rs
│   └── core/                # Business logic
│       ├── mod.rs
│       ├── downloader.rs    # Video download service
│       ├── queue.rs         # Download queue management
│       └── error.rs         # Error types
├── Cargo.toml              # Project dependencies
├── README.md               # This file
└── docs/                   # Additional documentation
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome!
