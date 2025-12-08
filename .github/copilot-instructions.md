# VDownloader AI Instructions

## Project Overview
VDownloader is a cross-platform desktop video downloader written in **Rust** using **GTK4**. It acts as a GUI wrapper around `yt-dlp`.
- **Core Logic**: Platform-agnostic, async Rust (Tokio). Handles URL validation, platform detection, and `yt-dlp` process management.
- **UI**: GTK4 (using `gtk4-rs`). Runs on the main thread.
- **Bridge**: Async tasks spawn on Tokio runtime; UI updates marshal back to main thread via GLib.

## Architecture & Patterns

### 1. Core/UI Separation (Strict)
- `src/core/`: **NO GTK dependencies**. Pure Rust logic.
  - `downloader.rs`: `VideoDownloader`, `Platform` enum.
  - `search.rs`: `SearchService` (wraps `yt-dlp --dump-json`).
  - `queue.rs`: `DownloadQueue` (thread-safe `Arc<RwLock>`).
- `src/ui/`: GTK4 code.
  - `window.rs`: Main window layout.
  - `components/`: Reusable widgets (`SearchView`, `DownloadQueue`).

### 2. Async & Thread Safety (CRITICAL)
- **Runtime**: `tokio` runtime is initialized in `main.rs` with `runtime.enter()`. This guard MUST be alive for `tokio::spawn` to work.
- **UI Updates**: NEVER call GTK methods from a background thread. Use `glib::MainContext::default().invoke_local()`:
  ```rust
  // In an async block/task:
  let label_clone = label.clone();
  glib::MainContext::default().invoke_local(move || {
      label_clone.set_text("Updated safely");
  });
  ```
- **State Sharing**: Use `Rc<RefCell<T>>` for state shared within the UI thread (callbacks). Use `Arc<RwLock<T>>` for state shared across threads (core services).

### 3. Video Extraction & Search
- **Engine**: Relies on `yt-dlp` CLI being in the system PATH.
- **Search**: Executes `yt-dlp --dump-json "query"`.
- **Platform Detection**: Simple string matching in `VideoDownloader::detect_platform` (e.

### Error Handling
- Use `thiserror` for library errors (`DownloadError`, `SearchError`).
- Log errors with `log::warn!` or `log::error!` before returning `Err`.
- Do not use `.unwrap()` in production code; use `?` or handle `None`/`Err` explicitly.

### UI Construction
- Use the **Builder Pattern**: `Button::builder().label("Click").build()`.
- Use CSS classes for styling: `title-1`, `dim-label`, `monospace`, `suggested-action`.

## Key Files
- `g., `url.contains("youtube.com")`).

## Developer Workflow

### Build & Run
- **Run**: `cargo run` (requires `yt-dlp` installed).
- **Logging**: `RUST_LOG=info cargo run` or `RUST_LOG=debug cargo run`.
- **Build Scripts** (Prefer these over raw cargo build):
  - Linux: `./build.sh` or `./build-appimage.sh`
  - Windows: `build.bat` (standard) or `build-windows-portable.bat` (bundled deps)
  - macOS: `./build-macos.sh`

### Testing
- `cargo test` (Runs unit tests in `src/core`).
- `cargo test -- --nocapture` to see logs during tests.

### Error Handling
- Use `thiserror` for library errors (`DownloadError`, `SearchError`).
- Log errors with `log::warn!` or `log::error!` before returning `Err`.
- Do not use `.unwrap()` in production code; use `?` or handle `None`/`Err` explicitly.

### UI Construction
- Use the **Builder Pattern**: `Button::builder().label("Click").build()`.
- Use CSS classes for styling: `title-1`, `dim-label`, `monospace`, `suggested-action`.

## Key Files
- `src/main.rs`: Entry point, Tokio runtime setup, Dark theme init.
- `src/core/downloader.rs`: Platform detection and download logic.
- `src/ui/components/search_view.rs`: Complex UI component example (async search, thumbnail caching).
