# VDownloader Architecture

## Overview

VDownloader is a cross-platform desktop video downloader built with Rust and GTK4.

## Technology Stack

### Core Technologies
- **Language**: Rust (latest stable)
- **GUI Framework**: GTK4
- **Video Extraction**: yt-dlp (via youtube_dl Rust crate)
- **Async Runtime**: Tokio

### Key Dependencies

```toml
[dependencies]
gtk4 = { version = "0.9", package = "gtk4" }
youtube_dl = "0.10.0"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
anyhow = "1"
```

## Architecture Decision: Video Extraction

**Decision**: Use `youtube_dl` Rust crate (wrapper around yt-dlp CLI)

**Rationale**:
- Multi-platform support (YouTube, TikTok, Twitter, Instagram, Reddit, 1000+ sites)
- Leverages industry-standard yt-dlp (137K+ GitHub stars)
- MIT/Apache-2.0 license compatible
- Platform API changes handled by yt-dlp team
- Clean Rust API with async support

**Trade-offs**:
- Requires yt-dlp installation on user system
- External dependency management

See [RESEARCH_VIDEO_EXTRACTION.md](../RESEARCH_VIDEO_EXTRACTION.md) for detailed analysis.

## Application Structure

```
VDownloader/
├── src/
│   ├── main.rs              # Entry point, GTK application setup
│   ├── ui/
│   │   ├── mod.rs           # UI module
│   │   ├── window.rs        # Main application window
│   │   ├── download_row.rs  # Download queue row widget
│   │   └── settings.rs      # Settings dialog
│   ├── services/
│   │   ├── mod.rs
│   │   ├── downloader.rs    # Video download service
│   │   └── metadata.rs      # Video metadata fetching
│   ├── models/
│   │   ├── mod.rs
│   │   ├── download.rs      # Download model
│   │   └── video.rs         # Video metadata model
│   └── utils/
│       ├── mod.rs
│       └── validators.rs    # URL validation
├── resources/               # GTK resources (UI files, icons)
├── Cargo.toml
└── README.md
```

## Component Design

### 1. UI Layer (GTK4)
- Main window with URL input
- Download queue list
- Progress tracking
- Settings dialog

### 2. Service Layer
- **VideoDownloader**: Wraps youtube_dl crate
- **MetadataFetcher**: Fetches video info before download
- **DownloadQueue**: Manages multiple simultaneous downloads

### 3. Model Layer
- **Video**: Metadata (title, thumbnail, duration, formats)
- **Download**: Download state, progress, output path

## Data Flow

```
User Input (URL) 
    ↓
[URL Validator]
    ↓
[Metadata Fetcher] → Display video info
    ↓
User confirms download
    ↓
[Download Queue] → Add to queue
    ↓
[Video Downloader] → Start download
    ↓
[Progress Updates] → Update UI
    ↓
[Completion] → Notify user
```

## Async Operations

Using Tokio for non-blocking operations:
- Video metadata fetching
- File downloads
- Progress updates

GTK main thread communication via channels or `glib::MainContext`.

## Error Handling

- Result/Option types throughout
- User-friendly error messages in UI
- Logging for debugging
- Graceful degradation

## Platform Support

### Target Platforms
- Linux (primary)
- Windows
- macOS

### Platform-Specific Considerations
- GTK4 installation requirements
- yt-dlp installation methods
- Default download directories
- File system paths

## Future Enhancements

### Phase 1 (MVP)
- Basic URL input and download
- Single video downloads
- Progress display
- Error handling

### Phase 2
- Download queue
- Multiple simultaneous downloads
- Format selection
- Download history

### Phase 3
- Playlist support
- Subtitle download
- Thumbnail embedding
- Download scheduling

### Phase 4
- Browser integration
- Batch downloads
- Custom output templates
- Download presets

## Performance Considerations

- Async/await for non-blocking operations
- Efficient progress tracking
- Memory management for large files
- Background downloads without UI freezing

## Security Considerations

- URL validation
- Safe file path handling
- Terms of Service compliance disclaimer
- No credential storage (future: secure keychain integration)

## Testing Strategy

- Unit tests for services and utilities
- Integration tests for download workflow
- UI tests (future)
- Manual testing on all platforms

## Build & Distribution

### Development Build
```bash
cargo build
cargo run
```

### Release Build
```bash
cargo build --release
```

### Distribution (Future)
- Linux: AppImage, Flatpak, .deb, .rpm
- Windows: Installer (.exe, .msi)
- macOS: .app bundle, .dmg

## Dependencies Installation

### Development Requirements
- Rust (latest stable)
- GTK4 development libraries
- pkg-config
- C compiler (gcc/clang)

### Runtime Requirements
- GTK4 runtime
- yt-dlp (user installs or bundled)
- Python (for yt-dlp)

## License

MIT License - See LICENSE file for details.
