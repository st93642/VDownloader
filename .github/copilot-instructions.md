# VDownloader AI Instructions

## Architecture Overview

**Core/UI Split** (strict separation):
- `src/core/`: Platform-agnostic business logic - no GTK dependencies
  - `downloader.rs`: `VideoDownloader`, `DownloadRequest` (URL validation, platform detection)
  - `search.rs`: `SearchService`, `SearchResult` (integration with yt-dlp, thumbnail loading)
  - `queue.rs`: `DownloadQueue` (async queue with `tokio::sync::RwLock`, infrastructure-ready)
  - `error.rs`: `DownloadError` enum (using `thiserror` crate)
- `src/ui/`: GTK4 interface - uses core, cannot be used outside GTK context
  - `window.rs`: Main window layout, tab switching, event wiring
  - `components/`: Reusable UI components (`SearchView`, `DownloadQueue`, `PreviewWindow`)

**Data Flow**: UI creates `DownloadRequest` → `VideoDownloader.download()` → streams status updates (via logging/UI callbacks) → updates `DownloadStatus`.

**Key Pattern**: Core functions are async (using `tokio`); UI runs on main GTK thread and invokes core via `glib::MainContext::default().invoke_local()` for thread-safe updates.

## Tech Stack & Critical Patterns
- **Language**: Rust 2021 edition
- **GUI**: GTK4 - Builder pattern for UI construction (`ApplicationWindow::builder().build()`)
  - Dark theme enabled by default in `main.rs`
  - CSS classes: `title-1`, `title-2`, `dim-label`, `monospace`, `suggested-action` for styling
  - State: Use `Rc<RefCell<T>>` for shared mutable state in closures (within main thread)
- **Async**: Tokio runtime initialized in `main.rs` with `let _guard = runtime.enter()` - critical for spawning tasks
- **Video Extraction**: `youtube_dl` crate → wraps `yt-dlp` CLI tool (must be in PATH at runtime)
  - Platform detection: `VideoDownloader::detect_platform(url: &str) → Platform` (YouTube, TikTok, Twitter, Instagram, Reddit, Vk, Rutube, Dzen)
- **Search**: Runs `yt-dlp --dump-json` subprocess, parses JSON output with `serde_json`
  - Caches thumbnails in `SearchView` using `HashMap<String, Pixbuf>` 
  - Supports both URL and search query input with heuristic detection
- **Error Types**: 
  - `DownloadError` (typed variants: InvalidUrl, DownloadFailed, UnsupportedPlatform, etc.)
  - `SearchError` (typed variants: InvalidQuery, CommandFailed, MissingYtDlp, RateLimited, etc.)
- **HTTP**: `reqwest` with `rustls-tls` for thumbnail loading (no blocking, async only)
- **Logging**: `log` + `env_logger` - structured via `log::info!`, `debug!`, `warn!`, `error!` macros

## File Structure
```
src/
├── main.rs              # Application entry point, Tokio runtime setup
├── core/                # Business logic (platform-agnostic)
│   ├── mod.rs
│   ├── downloader.rs    # VideoDownloader, DownloadRequest, platform detection
│   ├── queue.rs         # Download queue management
│   ├── search.rs        # Video search functionality
│   └── error.rs         # Custom error types
└── ui/                  # GTK4 user interface
    ├── mod.rs
    ├── window.rs        # Main application window
    └── components/      # Reusable UI components
        ├── mod.rs
        ├── download_queue.rs
        └── search_view.rs
```

## Developer Workflow

### Running the Application
```bash
# Standard run
cargo run

# With info logging (recommended for development)
RUST_LOG=info cargo run

# With debug logging for detailed tracing
RUST_LOG=debug cargo run

# With specific module logging
RUST_LOG=vdownloader::core::search=debug cargo run
```

### Building (use provided scripts, not bare `cargo build --release`)
```bash
./build.sh           # Linux (creates vdownloader-linux)
./build-macos.sh     # macOS (creates vdownloader-macos)
build.bat            # Windows standard build (creates vdownloader-windows.exe)
build-windows-portable.bat  # Windows portable with bundled GTK4, yt-dlp, ffmpeg (~150-200 MB)
./build-appimage.sh  # Linux AppImage (creates .appimage binary)
```

### Testing
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture --test-threads=1

# Run specific test module
cargo test downloader
cargo test search
cargo test queue
```

### Code Quality
```bash
# Format check
cargo fmt --check

# Apply formatting
cargo fmt

# Lint (check before commits)
cargo clippy -- -W clippy::all

# Run all checks before push
cargo fmt && cargo clippy && cargo test
```

## Key Files & Components
- `src/main.rs`: Application entry point, initializes logging, Tokio runtime (`runtime.enter()`), and GTK app. Dark theme enabled here.
- `src/core/downloader.rs`: Core download logic, `VideoDownloader` struct, `DownloadRequest`, `Platform` enum (YouTube, TikTok, Twitter, Instagram, Reddit, Vk, Rutube, Dzen, Other). Platform detection via URL string matching.
- `src/core/queue.rs`: Download queue management with `Arc<RwLock<HashMap>>` for thread-safe state. All operations are async.
- `src/core/search.rs`: Video search functionality, `SearchService::search(query, limit)` runs `yt-dlp --dump-json`, parses results. Handles both URLs and search queries.
- `src/core/error.rs`: Custom `DownloadError` and `SearchError` enums with typed variants for specific error cases.
- `src/ui/window.rs`: Main application window layout with tab switcher (`Stack`), download form, URL entry, directory picker. Uses `FileDialog` for folder selection.
- `src/ui/components/search_view.rs`: Search UI component with `SearchEntry`, `ListBox` for results, thumbnail caching via `HashMap<String, Pixbuf>`. Spawns async search tasks.
- `src/ui/components/preview_window.rs`: Modal preview window showing search result details (thumbnail, title, uploader). Used from search results.
- `src/ui/components/download_queue.rs`: Download queue UI component (infrastructure - currently shows pending/completed downloads).

## Critical Integration Patterns

### UI-to-Core Communication
1. **Spawning Async Tasks**: UI creates closures that call core async functions via `tokio::spawn()` or `glib::spawn_future_local()`.
2. **Thread-Safe Updates**: UI callbacks use `glib::MainContext::default().invoke_local()` to marshal results back to GTK main thread.
3. **State Sharing**: Use `Rc<RefCell<T>>` for shared mutable state in UI closures (e.g., `thumbnail_cache` in `SearchView`).
4. **Example Pattern** (from `search_view.rs`):
   ```rust
   let spinner = spinner.clone();
   tokio::spawn(async move {
       match search_service.search(query, None).await {
           Ok(results) => {
               glib::MainContext::default().invoke_local(move || {
                   spinner.set_visible(false);
                   // Update UI with results
               });
           }
           Err(e) => { /* handle error */ }
       }
   });
   ```

### Platform Detection & Search
- `VideoDownloader::detect_platform()` uses simple string matching on URLs (contains "youtube.com", "tiktok.com", etc.)
- Search query detection: if query lacks whitespace and contains '.', treat as URL; otherwise use as search term
- `SearchService::search()` delegates to `yt-dlp --dump-json` subprocess, parses output with `serde_json`
- Thumbnails are loaded asynchronously and cached locally in `HashMap<String, Pixbuf>`

### File Dialogs & Path Handling
- Use `gtk4::FileDialog` for folder selection (see `window.rs` browse button example)
- Default path: `$HOME/Videos` if it exists, else `$HOME` or `USERPROFILE` on Windows
- Always validate paths before use; check with `Path::exists()` and use `std::fs::create_dir_all()` for creation


### General Style & File Headers
- **File Headers**: Use TSI header template (see all source files for exact format):
  ```rust
  /*****************************************************************************/
  /*  filename.rs                                      TTTTTTTT SSSSSSS II    */
  /*  By: st93642@students.tsi.lv                        TT    SSSSSSS II    */
  /*  Created: Dec 07 2025 HH:MM st93642                 TT    SSSSSSS II    */
  /*  Updated: [date if modified]                                             */
  /*   Transport and Telecommunication Institute - Riga, Latvia                */
  /*****************************************************************************/
  ```
- **Naming**: `snake_case` for functions/variables, `PascalCase` for types, `SCREAMING_SNAKE_CASE` for constants
- **Comments**: Prefer self-documenting code; add comments only for non-obvious logic or complex algorithms

### Logging Best Practices
- **At startup**: Log service initialization (SearchService, VideoDownloader, queue)
- **State changes**: URL validation results, platform detection, search operations, file creation
- **Error context**: Always log before returning errors from core functions
- **Examples**: `info!("Creating VideoDownloader with output directory: {}", dir);`
- **Log levels**: 
  - `debug!()` for traces (yt-dlp subprocess calls, HTTP requests, JSON parsing)
  - `info!()` for important events (app startup, download started, search completed)
  - `warn!()` for recoverable issues (fallback parsing, missing thumbnails)
  - `error!()` for failures (platform detection failed, command execution failed)

### Error Handling Pattern
- Use `DownloadError` for download operations, `SearchError` for search operations
- **Always add context**: `Err(DownloadError::DownloadFailed(format!("Failed to extract from {}: {}", url, reason)))`
- Log errors before returning: `warn!("Error: {:?}", err); return Err(err);`
- Never `.unwrap()` in production - use `.map_err()` or `?` operator

### UI Development
- **Thread Safety Critical**: Use `glib::MainContext::default().invoke_local()` for all GTK updates from async code
- **State Sharing**: Use `Rc<RefCell<T>>` pattern for mutable state in closures - example from `search_view.rs`:
  ```rust
  let selected_path = Rc::new(RefCell::new(default_path.clone()));
  let path_clone = selected_path.clone();
  button.connect_clicked(move |_| {
      let path = path_clone.borrow();
      // use path
  });
  ```
- **Builder Pattern**: Always use `element::builder().property(value).build()` for UI construction
- **CSS Classes**: Apply standard GTK classes: `title-1`, `title-2`, `dim-label`, `monospace`, `suggested-action`, `boxed-list`

### Async Patterns
- Spawn with `tokio::spawn()` for background work (searches, downloads)
- For UI updates from async: wrap in `glib::MainContext::default().invoke_local(move || { /* update UI */ })`
- Never block the UI thread - always use async/await for I/O operations
- Channels (`tokio::sync::mpsc`) for task-to-task communication (not often used, but queue infrastructure ready)

### Path Handling
- Always use `std::path::Path` and `PathBuf` - never construct paths as strings
- Validate before use: `Path::exists()`, `Path::is_dir()`
- Create missing directories: `std::fs::create_dir_all()`
- Cross-platform: `HOME` on Unix, `USERPROFILE` on Windows (see `window.rs` for pattern)

### Testing
- Add tests in `#[cfg(test)]` modules at file bottom (see `downloader.rs`, `search.rs`)
- For async tests: use `#[tokio::test]` attribute
- Test core logic thoroughly - UI tests less critical
- Use descriptive test names: `test_detect_platform_youtube`, `test_invalid_url_rejected`

## Dependencies
- **Runtime**: Requires `yt-dlp` installed on the system (not bundled in standard builds)
  - Standard build: Users must install yt-dlp separately (`pip install yt-dlp` or via package manager)
  - Portable Windows build: Both yt-dlp and ffmpeg are bundled (see `build-windows-portable.bat`)
- **Build-time**: GTK4 development libraries required for compilation
- **Platform-specific**: See `BUILD.md` or platform-specific build scripts for detailed requirements

## Common Pitfalls & How to Avoid Them
- **GTK Thread Safety**: Never call GTK methods from non-main threads. Always use `glib::MainContext::default().invoke_local()` to marshal updates back to main thread.
- **Reference Cycles**: Be careful with `Rc<RefCell<T>>` patterns to avoid memory leaks - ensure closures don't create cycles.
- **Clone Semantics**: Explicitly clone when moving data into closures: `let var_clone = var.clone()` - Rust's ownership requires this.
- **Tokio Runtime**: The Tokio runtime must be initialized early in `main.rs` with `let _guard = runtime.enter()` before spawning any async tasks.
- **yt-dlp Dependency**: Application will fail at runtime if `yt-dlp` is not in PATH. Check in `SearchService::search()` and `VideoDownloader::download()`.
- **Path Construction**: Never build paths as strings - always use `PathBuf::new()` and `path.join()`.
- **Error Logging**: Log errors before returning them - helps debugging. Example: `warn!("Error: {}", err);`
- **FileDialog Futures**: FileDialog operations return futures - must spawn on Tokio and marshal back to GTK main thread.

## Platform Support & Distribution
- **Linux**: Primary development platform, fully supported
  - Standard binary: ~4 MB, requires GTK4 runtime
  - AppImage: ~36 MB, self-contained (run with `./build-appimage.sh`)
- **macOS**: Supported via Homebrew dependencies
- **Windows**: Supported via MSYS2 and GTK4 runtime
  - Standard: `build.bat` → ~4 MB exe (requires GTK4 installation)
  - Portable: `build-windows-portable.bat` → ~150-200 MB with all deps bundled (includes yt-dlp + ffmpeg)
  - Use portable build for distribution to end-users (no setup required)
- See `BUILD.md`, `WINDOWS_PORTABLE.md` for detailed platform instructions

## Security Considerations
- **Path Traversal**: Validate user-provided paths to prevent directory traversal attacks - use `Path::canonicalize()` if needed
- **URL Validation**: Validate URLs before passing to `yt-dlp` to prevent command injection - use `VideoDownloader::validate_url()`
- **Safe Unwrapping**: Avoid `.unwrap()` and `.expect()` in production code - always use proper error handling with `?` operator
- **Input Sanitization**: Sanitize user input, especially URLs and file paths - rely on yt-dlp's validation as secondary layer
- **Subprocess Safety**: Never interpolate user input into yt-dlp commands - always pass URLs as arguments directly

## CI/CD & Release Process
- **GitHub Actions workflow**: `.github/workflows/build.yml` builds for all platforms (Linux, Windows, macOS)
- **Trigger**: Runs on push to `main`/`master` branches and pull requests
- **Release automation**: Creates releases automatically for version tags (format: `v*`, e.g., `v0.1.0`)
- **Artifacts generated**: 
  - `vdownloader-linux` (binary, ~4 MB)
  - `vdownloader-windows.exe` (binary, ~4 MB)
  - `vdownloader-windows-portable.zip` (~150-200 MB with yt-dlp + ffmpeg bundled)
  - `vdownloader-macos` (binary, ~4 MB)
  - `vdownloader-*.AppImage` (Linux AppImage, ~36 MB)
- See `RELEASE.md` and `docs/creating-first-release.md` for release workflow details
