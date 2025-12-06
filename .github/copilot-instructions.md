# VDownloader AI Instructions

## Architecture Overview
- **Core/UI Split**: Strict separation between business logic (`src/core/`) and interface (`src/ui/`).
  - `src/core/`: Platform-agnostic logic, `youtube_dl` integration, queue management.
  - `src/ui/`: GTK4 components, window management, event handling.
- **Entry Point**: `src/main.rs` initializes `env_logger` and the GTK `Application`.
- **Data Flow**: UI creates `DownloadRequest` -> passed to `VideoDownloader` -> updates `DownloadStatus`.

## Tech Stack & Patterns
- **Language**: Rust (2021 edition).
- **GUI**: GTK4 (Rust bindings).
  - **Construction**: Programmatic UI building using Builder pattern (e.g., `ApplicationWindow::builder()`). Avoid `.ui` XML files unless necessary.
  - **State**: `Rc<RefCell<T>>` for shared mutable state within UI closures.
  - **Styling**: Use standard GTK CSS classes (e.g., `title-1`, `dim-label`).
- **Async**: `tokio` runtime.
- **Video Extraction**: `youtube_dl` crate (wrapper for `yt-dlp`).
- **Error Handling**: `thiserror` for library errors, `anyhow` for application errors.

## Key Files
- `src/ui/window.rs`: Main window layout and event wiring.
- `src/core/downloader.rs`: `VideoDownloader` struct, `DownloadRequest` definition, platform detection.
- `src/core/queue.rs`: Manages the list of active/pending downloads.

## Developer Workflow
- **Run**: `cargo run`
- **Logging**: `env_logger` is initialized. Run with `RUST_LOG=info cargo run` to see logs.
- **Dependencies**: Requires `yt-dlp` installed on the system (runtime dependency).

## Coding Conventions
- **Logging**: Use `log::info!`, `debug!`, `warn!` extensively for tracing execution flow.
- **UI Updates**: Ensure UI updates happen on the main thread (GTK requirement).
- **Path Handling**: Use `std::path::Path` and `PathBuf` for file system operations.
