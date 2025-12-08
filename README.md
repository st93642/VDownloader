
# VDownloader

A modern, cross-platform video downloader with GTK4 interface. Download videos from YouTube, TikTok, X, VK, Rutube, Reddit, Dzen, and more.

## Quick Start

### For End Users

**Windows (Portable - No Installation):**

1. Download `vdownloader-windows-portable.zip` from [Releases](https://github.com/st93642/VDownloader/releases) or [dist/](dist/)
2. Extract and run `VDownloader.bat`
3. Done! Everything is bundled (GTK4 + yt-dlp)

**Linux (AppImage):**

1. Download `VDownloader-linux.AppImage`
2. Make executable: `chmod +x VDownloader-linux.AppImage`
3. Run: `./VDownloader-linux.AppImage`

**Linux (Binary):**

```bash
# Install GTK4
sudo apt install libgtk-4-1 yt-dlp  # Ubuntu/Debian
# OR
sudo dnf install gtk4 yt-dlp        # Fedora

# Download and run
./vdownloader-linux
```

**macOS:**

```bash
brew install gtk4 yt-dlp
./vdownloader-macos
```

### For Developers

**Prerequisites:**

- Rust (stable)
- GTK4 development libraries
- yt-dlp

**Build:**

```bash
git clone https://github.com/st93642/VDownloader.git
cd VDownloader
cargo build --release
```

**Platform-specific build instructions:** See [BUILD.md](BUILD.md)

## Features

- 🎥 Multi-platform support (YouTube, TikTok, X, VK, Rutube, Reddit, Dzen, Instagram)
- 🔍 Integrated video search
- 📊 Download queue management
- 🎨 Modern GTK4 interface
- ⚡ Fast, async Rust implementation

## Technology Stack

- **Language**: Rust 🦀
- **GUI**: GTK4
- **Engine**: yt-dlp
- **Runtime**: Tokio

## Distribution Formats

| Platform | Format | Size | Dependencies |
|----------|--------|------|--------------|
| Windows | Portable ZIP | ~150 MB | None (all bundled) |
| Windows | Standard EXE | ~4 MB | GTK4 + yt-dlp |
| Linux | AppImage | ~36 MB | None (all bundled) |
| Linux | Binary | ~4 MB | GTK4 + yt-dlp |
| macOS | Binary | ~4 MB | GTK4 + yt-dlp |

## Building

### Prerequisites

**Linux:**

```bash
sudo apt install libgtk-4-dev build-essential pkg-config yt-dlp
```

**macOS:**

```bash
brew install gtk4 yt-dlp
```

**Windows:**

```bash
# MSYS2 UCRT64
pacman -S mingw-w64-ucrt-x86_64-gtk4 mingw-w64-ucrt-x86_64-toolchain
```

### Build Commands

```bash
# Standard build
cargo build --release

# Run with logging
RUST_LOG=info cargo run

# Run tests
cargo test

# Platform-specific builds
./build.sh                    # Linux
./build-appimage.sh           # Linux AppImage
./build-windows-portable.bat  # Windows portable
./build-macos.sh              # macOS
```

**Full documentation:** [BUILD.md](BUILD.md) | [DISTRIBUTION.md](DISTRIBUTION.md)

## Project Structure

```text
src/
├── main.rs           # Entry point
├── core/             # Business logic (platform-agnostic)
│   ├── downloader.rs # Video download engine
│   ├── queue.rs      # Queue management
│   ├── search.rs     # Video search
│   └── error.rs      # Error handling
└── ui/               # GTK4 interface
    ├── window.rs     # Main window
    └── components/   # UI components
```

## Downloads

- **Releases**: [GitHub Releases](https://github.com/st93642/VDownloader/releases)
- **Latest Builds**: [dist/](dist/) (auto-updated by CI)
- **Documentation**: [BUILD.md](BUILD.md) | [DISTRIBUTION.md](DISTRIBUTION.md)
- **Windows Portable**: [WINDOWS_PORTABLE.md](WINDOWS_PORTABLE.md)

## Troubleshooting

- **Windows "Missing DLL"**: See [WINDOWS_PORTABLE_TROUBLESHOOTING.md](WINDOWS_PORTABLE_TROUBLESHOOTING.md)
- **Build Issues**: Check [BUILD.md](BUILD.md)
- **Other Issues**: Open an [issue](https://github.com/st93642/VDownloader/issues)

## License

MIT License - see [LICENSE](LICENSE)

## Contributing

Contributions welcome! Please open an issue or PR.
