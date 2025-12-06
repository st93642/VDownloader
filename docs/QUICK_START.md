# Quick Start Guide

This guide will help you get started with developing VDownloader.

## Prerequisites

Before you begin, ensure you have the following installed:

### Required Development Tools

1. **Rust** (latest stable version)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **GTK4 Development Libraries**

   **Linux (Ubuntu/Debian):**
   ```bash
   sudo apt update
   sudo apt install libgtk-4-dev build-essential pkg-config
   ```

   **Linux (Fedora):**
   ```bash
   sudo dnf install gtk4-devel gcc
   ```

   **macOS:**
   ```bash
   brew install gtk4 pkg-config
   ```

   **Windows:**
   - Install [MSYS2](https://www.msys2.org/)
   - In MSYS2 terminal:
     ```bash
     pacman -S mingw-w64-x86_64-gtk4 mingw-w64-x86_64-toolchain
     ```

3. **yt-dlp** (Runtime dependency)

   **Linux:**
   ```bash
   sudo apt install yt-dlp
   # OR
   pip3 install yt-dlp
   ```

   **macOS:**
   ```bash
   brew install yt-dlp
   ```

   **Windows:**
   ```bash
   pip install yt-dlp
   ```

## Building the Project

1. **Clone the repository:**
   ```bash
   git clone https://github.com/st93642/VDownloader.git
   cd VDownloader
   ```

2. **Build the project:**
   ```bash
   cargo build
   ```

3. **Run the application:**
   ```bash
   cargo run
   ```

4. **Build for release:**
   ```bash
   cargo build --release
   ./target/release/vdownloader
   ```

## Development Workflow

### Running with Logging

Enable logging to see debug information:

```bash
RUST_LOG=info cargo run
RUST_LOG=debug cargo run  # More verbose
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_detect_platform_youtube
```

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting without modifying
cargo fmt --check

# Run linter
cargo clippy

# Run linter with all warnings
cargo clippy -- -W clippy::all
```

### Development Build vs Release Build

- **Development build** (`cargo build`):
  - Faster compilation
  - Includes debug symbols
  - No optimizations
  - Larger binary size
  - Use during development

- **Release build** (`cargo build --release`):
  - Slower compilation
  - Optimized for performance
  - Smaller binary size (with strip = true)
  - Use for distribution

## Project Structure

```
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
└── docs/                   # Documentation
```

## Understanding the Codebase

### Main Entry Point (`src/main.rs`)

The application starts by initializing the GTK4 application and setting up logging:

```rust
fn main() -> glib::ExitCode {
    env_logger::init();
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(build_ui);
    app.run()
}
```

### UI Layer (`src/ui/`)

- **window.rs**: Builds the main application window with URL input, download button, progress bar, and download queue
- **components/**: Reusable UI components like the download queue display

### Core Layer (`src/core/`)

- **downloader.rs**: Contains the `VideoDownloader` service for handling downloads and platform detection
- **queue.rs**: Manages the download queue with async operations
- **error.rs**: Custom error types for better error handling

## Adding Features

### Adding a New UI Component

1. Create a new file in `src/ui/components/`
2. Add module declaration in `src/ui/components/mod.rs`
3. Implement the component using GTK4 widgets
4. Use the component in `window.rs`

Example:
```rust
// src/ui/components/settings_panel.rs
use gtk4::{prelude::*, Box, Label, Orientation};

pub fn create_settings_panel() -> Box {
    let panel = Box::new(Orientation::Vertical, 6);
    let label = Label::new(Some("Settings"));
    panel.append(&label);
    panel
}
```

### Adding a New Core Service

1. Create a new file in `src/core/`
2. Add module declaration in `src/core/mod.rs`
3. Implement the service with proper error handling
4. Add tests

## Common Issues

### GTK4 Not Found

**Error:** `Package gtk4 was not found in the pkg-config search path`

**Solution:** Install GTK4 development libraries (see Prerequisites)

### yt-dlp Not Found

**Error:** Runtime error when trying to download

**Solution:** Install yt-dlp system-wide (see Prerequisites)

### Compilation is Slow

**Tip:** Use `cargo build` (debug build) during development. Only use `cargo build --release` for testing final performance or distribution.

### Application Window Doesn't Open

**On Linux:** Ensure you're running in an environment with a display server (X11 or Wayland).

**Testing without display:**
```bash
# Build succeeds but running requires display
cargo build  # This will work
cargo run    # This requires a display server
```

## Next Steps

- Read the [Architecture Document](./ARCHITECTURE.md) to understand the system design
- Check out [RESEARCH_VIDEO_EXTRACTION.md](../RESEARCH_VIDEO_EXTRACTION.md) for details on the video extraction library choice
- Start implementing download functionality using the `youtube_dl` crate
- Add more UI components for better user experience

## Getting Help

- Check the [GitHub Issues](https://github.com/st93642/VDownloader/issues) for known problems
- Review the main [README](../README.md) for general information
- Look at the inline documentation in the source code

Happy coding! 🦀
