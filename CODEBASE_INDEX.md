# VDownloader Codebase Index

Complete reference guide to the VDownloader project structure, modules, and
components.

## Quick Navigation

- **[ARCHITECTURE.md](ARCHITECTURE.md)** - System design and data flow
- **[SETUP.md](SETUP.md)** - Development setup and prerequisites
- **[DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)** - How to extend and
  modify the codebase
- **[API_REFERENCE.md](API_REFERENCE.md)** - Public API documentation

## Project Structure

```text
VDownloader/
├── Cargo.toml                    # Project manifest with dependencies
├── Cargo.lock                    # Dependency lock file
├── src/
│   ├── main.rs                  # Application entry point and GTK initialization
│   ├── core/                    # Business logic layer (platform-agnostic)
│   │   ├── mod.rs              # Module exports
│   │   ├── downloader.rs       # Video download engine with yt-dlp integration
│   │   ├── search.rs           # Video search service with yt-dlp CLI
│   │   ├── queue.rs            # Download queue management
│   │   └── error.rs            # Error types and definitions
│   └── ui/                      # GTK4 user interface layer
│       ├── mod.rs              # Module exports
│       ├── window.rs           # Main application window
│       └── components/         # Reusable UI components
│           ├── mod.rs
│           ├── download_queue.rs   # Download queue UI widget
│           ├── search_view.rs      # Video search UI component
│           └── preview_window.rs   # Video preview/details window
├── docs/                         # Documentation (this file and others)
├── scripts/                      # Build and automation scripts
├── dist/                         # Pre-built binaries and releases
├── .github/                      # GitHub Actions CI/CD workflows
├── .gitignore                    # Git ignore rules
├── README.md                     # Main project documentation
├── LICENSE                       # MIT License
├── BUILD.md                      # Detailed build instructions
├── DISTRIBUTION.md               # Distribution and release guidelines
└── WINDOWS_PORTABLE.md           # Windows portable build documentation

```

## Module Catalog

### Core Layer (`src/core/`)

The core layer contains all business logic independent of the UI framework.

#### **downloader.rs** - Video Download Engine

- **Purpose**: Handles video download operations and platform detection
- **Key Types**:
  - `Platform` (enum): Supported video platforms
    - YouTube, TikTok, Twitter, Instagram, Reddit, VK, Rutube, Dzen, Other
  - `DownloadRequest` (struct): Request parameters for a video download
  - `DownloadStatus` (enum): Current status of a download
  - `VideoDownloader` (struct): Main download service
- **Key Methods**:
  - `new(output_directory)` - Create downloader instance
  - `detect_platform(url)` - Auto-detect platform from URL
  - `validate_url(url)` - Validate URL format
  - `download(request)` - Execute download asynchronously
- **Integration**: Uses `youtube_dl` crate (wrapper for yt-dlp CLI)
- **Error Handling**: Returns `Result<T, DownloadError>`

#### **search.rs** - Video Search Service

- **Purpose**: Search for videos across multiple platforms
- **Key Types**:
  - `SearchResult` (struct): Structured search result with metadata
    - Fields: id, title, url, thumbnail, duration, uploader, view_count, platform
  - `SearchError` (enum): Search-specific error types
  - `SearchService` (struct): Main search orchestrator with caching
- **Key Methods**:
  - `new(default_limit)` - Create search service
  - `async fn search(query, limit)` - Execute search across platforms
- **Features**:
  - Multi-platform search (YouTube, Dzen, Rutube)
  - Async thumbnail loading with caching
  - URL vs keyword search detection
  - Rate limit detection (HTTP 429)

- **Integration**: Spawns yt-dlp CLI for searches, parses JSON output

#### **queue.rs** - Download Queue Management

- **Purpose**: Manage pending, active, and completed downloads
- **Key Types**:
  - `QueueItem` (struct): Individual queue entry
  - `DownloadQueue` (struct): Thread-safe queue container
- **Key Methods**:
  - `new()` - Create queue
  - `async fn add(request)` - Add item to queue
  - `async fn get(id)` - Retrieve item by ID
  - `async fn update_status(id, status)` - Update item status
  - `async fn list_all()` - Get all items
  - `async fn remove(id)` - Remove item
- **Implementation**: Uses `Arc<RwLock<T>>` for thread-safe access

#### **error.rs** - Error Type Definitions

- **Purpose**: Define and categorize error types across the application

- **Error Types**:

  - `InvalidUrl` - URL format validation failed

  - `DownloadFailed` - Download operation failed

  - `UnsupportedPlatform` - Platform not supported

  - `IoError` - File I/O error

  - `ExtractionError` - Video metadata extraction failed
  - `NetworkError` - Network request failed
  - `VideoNotFound` - Video unavailable or deleted
  - `InvalidOutputDirectory` - Output path invalid or inaccessible
  - `Cancelled` - User cancelled operation

- **Traits**: All error types implement `Clone` for use in async contexts

### UI Layer (`src/ui/`)

GTK4-based user interface using the builder pattern and reactive component architecture.

#### **window.rs** - Main Application Window

- **Purpose**: Root UI container and main layout orchestration
- **Features**:
  - Tab switcher between Download and Search modes
  - URL input with automatic platform detection
  - Output directory selection with file dialog
  - Download queue visualization
  - Real-time progress updates
- **Key Components**:
  - Header section with title and tab switcher
  - Download tab: URL input, directory selection, download button
  - Search tab: Embedded SearchView component
- **Integration Points**:
  - Creates `SearchService` instance
  - Manages `VideoDownloader` and `DownloadQueue`
  - Handles GTK signals and callbacks

#### **components/download_queue.rs** - Queue UI Widget

- **Purpose**: Display and manage downloads in progress
- **Features**:
  - List view of queued downloads
  - Progress bars with percentage
  - Status labels (Pending, Downloading, Completed, Failed)
  - Cancel button for active downloads
  - Auto-refresh on status changes

#### **components/search_view.rs** - Search Interface Component

- **Purpose**: Reusable search interface with result cards
- **Features**:
  - Search input with enter-to-search
  - Loading spinner during search
  - Paginated result cards
  - Thumbnail loading and caching
  - Download button on each result
  - Error message display
- **Architecture**:
  - Internal state: `Rc<RefCell<T>>` for GTK closures
  - Async operations: `gtk4::glib::spawn_future_local()`
  - Callbacks: User-registered closure for download actions
- **Result Card Layout**:
  - Thumbnail (120x90px with placeholder)
  - Title, uploader, duration, view count, platform
  - Download button

#### **components/preview_window.rs** - Video Preview Window

- **Purpose**: Display detailed video information before download
- **Features**:
  - Thumbnail preview
  - Metadata display (title, duration, uploader)

  - Video description/URL

  - Download confirmation

## Dependencies and Versions

### Core Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| `gtk4` | 0.9 | GTK4 UI framework with v4_10 features |
| `glib` | 0.20 | Core GLib types and event loop |
| `gio` | 0.20 | I/O and resource management |
| `gdk-pixbuf` | 0.20 | Image loading and manipulation |
| `tokio` | 1.35 | Async runtime with full features |
| `youtube_dl` | 0.9 | Wrapper for yt-dlp CLI tool |

### Utility Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| `reqwest` | 0.11 | HTTP client (rustls-tls, for thumbnails) |
| `serde` | 1.0 | Serialization framework |
| `serde_json` | 1.0 | JSON parsing |
| `thiserror` | 1.0 | Error type derivation macros |
| `anyhow` | 1.0 | Flexible error handling |
| `log` | 0.4 | Logging facade |
| `env_logger` | 0.11 | Logging implementation |
| `regex` | 1.12 | Regular expression support |
| `urlencoding` | 2.1 | URL encoding utilities |

### Runtime Dependencies

| Tool | Purpose | Installation |
|------|---------|--------------|
| `yt-dlp` | Video extraction | `pip3 install yt-dlp` or package manager |

## Platform Support

VDownloader can download from and search on the following platforms:

| Platform | Detection Pattern | Status |
|----------|-------------------|--------|
| YouTube | youtube.com, youtu.be | ✅ Fully supported |
| TikTok | tiktok.com | ✅ Fully supported |
| Twitter/X | twitter.com, x.com | ✅ Fully supported |
| Instagram | instagram.com | ✅ Fully supported |
| Reddit | reddit.com | ✅ Fully supported |
| VK Video | vk.com, vkvideo.ru | ✅ Fully supported |
| Rutube | rutube.ru | ✅ Fully supported |
| Dzen | dzen.ru | ✅ Fully supported |
| Other | Any valid URL | ⚠️ Partial (yt-dlp dependent) |

## Configuration Reference

### Environment Variables

| Variable | Purpose | Default | Example |
|----------|---------|---------|---------|
| `RUST_LOG` | Logging level | (none) | `RUST_LOG=info` |

### Compile-time Constants

| Location | Name | Value | Purpose |
|----------|------|-------|---------|
| `src/main.rs` | `APP_ID` | `com.vdownloader.VDownloader` | GTK application ID |

### Runtime Configuration

| Service | Configuration | Default |
|---------|---------------|---------|
| `SearchService` | `default_limit` | 10 results |
| `VideoDownloader` | `output_directory` | User-selected path |

## Build Configuration

### Release Profile

Located in `Cargo.toml`:

```toml
[profile.release]
opt-level = "z"      # Optimize for size
lto = true           # Link Time Optimization
codegen-units = 1    # Better optimization
strip = true         # Strip debug symbols
panic = "abort"      # Reduce binary size

```

## Key Architectural Patterns

### Separation of Concerns

- **Core layer** (`core/`): Business logic independent of UI framework

- **UI layer** (`ui/`): GTK4-specific presentation logic

- **Clear boundaries**: UI calls core services, core doesn't depend on UI

### Async Architecture

- **Tokio runtime**: Initialized in `main.rs`

- **Async/await**: All I/O operations are non-blocking

- **GTK integration**: `gtk4::glib::spawn_future_local()` for async callbacks

### Error Handling

- **Custom error types**: Specific error enums for different modules

- **Thiserror macros**: Auto-implements `Display` and `Error` traits

- **Clone trait**: All errors implement `Clone` for async contexts

### Type Safety

- **Platform enum**: Type-safe platform representation

- **DownloadRequest/DownloadStatus**: Structured data types

- **Result<T, E>**: Explicit error handling throughout

## Testing Strategy

### Test Organization

- **Location**: `#[cfg(test)]` modules within source files

- **Scope**: Unit tests for individual components

- **Coverage**: Error handling, validation, parsing logic

### Running Tests

```bash
cargo test                    # Run all tests
cargo test -- --nocapture    # Show output
cargo test platform_detect   # Run specific test

```

## Development Workflow

### Building

```bash
cargo build                   # Debug build (faster)
cargo build --release        # Optimized release build

```

### Running

```bash
cargo run                     # Run with debug logging
RUST_LOG=info cargo run       # With info-level logging
RUST_LOG=debug cargo run      # With debug-level logging

```

### Code Quality

```bash
cargo fmt                     # Format code
cargo fmt --check            # Check formatting
cargo clippy                  # Lint and suggestions
cargo test                    # Run tests

```

## Extension Points

### Adding a New Download Platform

1. Add variant to `Platform` enum in `downloader.rs`
1. Update `Platform::detect_platform()` with URL pattern
1. Update `platform_from_hint()` in `search.rs`
1. yt-dlp will automatically handle the new platform

### Adding a New Search Feature

1. Implement search logic in `search.rs`
1. Add to `SearchService::search()` aggregation
1. Spawn as tokio task for parallel execution
1. Handle platform detection from results

### Extending the UI

1. Create component in `ui/components/`
1. Implement widget builder pattern
1. Use `Rc<RefCell<T>>` for shared mutable state
1. Use `gtk4::Box` for layouts (conflicts with `std::boxed::Box`)

## Documentation Files

| File | Purpose |
|------|---------|
| `README.md` | End-user documentation and quick start |
| `CODEBASE_INDEX.md` | This file - structural overview |
| `ARCHITECTURE.md` | System design and data flow |
| `SETUP.md` | Development environment setup |
| `DEVELOPMENT_GUIDE.md` | How to modify and extend the code |
| `API_REFERENCE.md` | Public API documentation |
| `BUILD.md` | Platform-specific build instructions |
| `DISTRIBUTION.md` | Release and distribution procedures |
| `WINDOWS_PORTABLE.md` | Windows portable build details |

## Git Workflow

### Branch Strategy

- `main` - Stable release branch

- `docs/` - Documentation branches

- Feature branches for development

### Commit Message Format

Follow conventional commits when possible:

- `feat: ...` - New feature

- `fix: ...` - Bug fix

- `docs: ...` - Documentation

- `refactor: ...` - Code refactoring

- `test: ...` - Test additions

## Common Commands Reference

| Command | Purpose |
|---------|---------|
| `cargo build` | Build debug binary |
| `cargo run` | Run application |
| `cargo test` | Run all tests |
| `cargo fmt` | Format code |
| `cargo clippy` | Lint code |
| `./build.sh` | Build Linux release |
| `./build-appimage.sh` | Build Linux AppImage |
| `./build-macos.sh` | Build macOS release |
| `./build-windows-portable.bat` | Build Windows portable |
