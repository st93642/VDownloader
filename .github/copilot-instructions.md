# VDownloader AI Instructions

## Architecture Overview
- **Core/UI Split**: Strict separation between business logic (`src/core/`) and interface (`src/ui/`).
  - `src/core/`: Platform-agnostic logic, `youtube_dl` integration, queue management, search functionality.
  - `src/ui/`: GTK4 components, window management, event handling, UI components.
- **Entry Point**: `src/main.rs` initializes `env_logger`, Tokio runtime, and the GTK `Application`.
- **Data Flow**: UI creates `DownloadRequest` -> passed to `VideoDownloader` -> updates `DownloadStatus`.
- **Error Handling**: Custom `DownloadError` type in `src/core/error.rs` with `thiserror` for typed errors.

## Tech Stack & Patterns
- **Language**: Rust (2021 edition).
- **GUI**: GTK4 (Rust bindings).
  - **Construction**: Programmatic UI building using Builder pattern (e.g., `ApplicationWindow::builder()`). Avoid `.ui` XML files unless necessary.
  - **State**: `Rc<RefCell<T>>` for shared mutable state within UI closures.
  - **Styling**: Use standard GTK CSS classes (e.g., `title-1`, `dim-label`). Dark theme is enabled by default.
- **Async**: `tokio` runtime initialized at startup.
- **Video Extraction**: `youtube_dl` crate (wrapper for `yt-dlp`).
- **Error Handling**: `thiserror` for library errors in `src/core/error.rs`, `anyhow` for application errors.
- **HTTP**: `reqwest` with `rustls-tls` for thumbnail loading and API requests.
- **Logging**: `log` crate with `env_logger` for runtime logging.

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

# With debug logging
RUST_LOG=debug cargo run

# With info logging (recommended for development)
RUST_LOG=info cargo run
```

### Building
```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Platform-specific builds
./build.sh           # Linux
./build-macos.sh     # macOS
build.bat            # Windows
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocturnal --show-output

# Run specific test module
cargo test downloader
cargo test queue
```

### Code Quality
```bash
# Check formatting
cargo fmt --check

# Apply formatting
cargo fmt

# Run linter
cargo clippy

# Run linter with all warnings
cargo clippy -- -W clippy::all
```

## Key Files & Components
- `src/main.rs`: Application entry point, initializes logging, Tokio runtime, and GTK app.
- `src/core/downloader.rs`: Core download logic, `VideoDownloader` struct, `DownloadRequest`, platform detection.
- `src/core/queue.rs`: Download queue management with state tracking.
- `src/core/search.rs`: Video search functionality integration.
- `src/core/error.rs`: Custom `DownloadError` enum with typed error variants.
- `src/ui/window.rs`: Main application window layout and event wiring.
- `src/ui/components/download_queue.rs`: Download queue UI component.
- `src/ui/components/search_view.rs`: Search interface component.

## Coding Conventions

### General Style
- **File Headers**: Use TSI header template with file info, author, creation date, and institution.
- **Naming**: Follow Rust conventions - `snake_case` for functions/variables, `PascalCase` for types.
- **Comments**: Prefer self-documenting code; add comments for complex logic or non-obvious behavior.

### Logging
- Use `log::info!`, `debug!`, `warn!`, and `error!` macros extensively for tracing execution flow.
- Log important state changes, user actions, and errors.
- Use appropriate log levels: `debug` for detailed tracing, `info` for important events, `warn` for recoverable issues, `error` for failures.

### Error Handling
- Use `DownloadError` enum for all download-related errors in `src/core/`.
- Propagate errors with `?` operator when appropriate.
- Convert errors using `.map_err()` when needed.
- Log errors before returning them to provide context.

### UI Development
- **Thread Safety**: Ensure all UI updates happen on the main thread (GTK requirement).
  - Use `glib::MainContext::default().invoke()` for cross-thread UI updates.
- **State Management**: Use `Rc<RefCell<T>>` for shared mutable state within UI closures.
- **Builder Pattern**: Construct UI elements using the builder pattern for clarity.
- **CSS Classes**: Use standard GTK CSS classes for consistent styling.

### Async Programming
- Use `tokio` runtime for async operations (file I/O, downloads, API calls).
- Spawn tasks with `tokio::spawn` for background work.
- Use channels (`tokio::sync::mpsc`) for communication between async tasks and UI.

### Path Handling
- Always use `std::path::Path` and `PathBuf` for file system operations.
- Handle cross-platform path differences appropriately.
- Validate paths before use, especially user-provided paths.

### Testing
- Add tests in `#[cfg(test)]` modules at the bottom of source files.
- Test core business logic thoroughly, especially in `src/core/`.
- Use descriptive test names that explain what is being tested.
- Mock external dependencies when testing (e.g., file system, network).

## Dependencies
- **Runtime**: Requires `yt-dlp` installed on the system (not bundled).
- **Build-time**: GTK4 development libraries required for compilation.
- **Platform-specific**: See `BUILD.md` for detailed platform requirements.

## Common Pitfalls
- **GTK Thread Safety**: Never call GTK methods from non-main threads. Always use `glib::MainContext::default().invoke()`.
- **Reference Cycles**: Be careful with `Rc<RefCell<T>>` to avoid memory leaks from reference cycles.
- **Clone Semantics**: Use `.clone()` explicitly when moving data into closures; Rust's ownership model requires this.
- **Tokio Runtime**: The Tokio runtime must be initialized before any async operations. This happens in `main.rs`.
- **yt-dlp Dependency**: Application will fail at runtime if `yt-dlp` is not in PATH.

## Platform Support
- **Linux**: Primary development platform, fully supported.
- **macOS**: Supported via Homebrew dependencies.
- **Windows**: Supported via MSYS2 and GTK4 runtime.
- See `BUILD.md` for platform-specific build instructions.

## Security Considerations
- **Path Traversal**: Validate user-provided paths to prevent directory traversal attacks.
- **URL Validation**: Validate URLs before passing to `yt-dlp` to prevent command injection.
- **Safe Unwrapping**: Avoid `.unwrap()` in production code; use proper error handling.
- **Input Sanitization**: Sanitize user input, especially URLs and file paths.

## CI/CD
- GitHub Actions workflow at `.github/workflows/build.yml` builds for all platforms.
- Runs on push to `main`/`master` branches and pull requests.
- Creates releases automatically for version tags (format: `v*`).
