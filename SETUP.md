# VDownloader Development Setup Guide

Complete guide for setting up a development environment to build and work with VDownloader.

## Prerequisites

### System Requirements

**Minimum**:
- 2GB RAM
- 1GB disk space for build artifacts
- Internet connection for dependency downloads

**Recommended**:
- 4GB RAM
- 5GB disk space
- Modern CPU (for faster builds)

### Required Software

#### Rust (Required)
VDownloader requires Rust 2021 edition (stable).

**Install Rust:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Verify installation:**
```bash
rustc --version  # Should be 1.70 or newer
cargo --version
```

#### yt-dlp (Required at Runtime)
Video extraction and search functionality depend on yt-dlp.

**Linux (Ubuntu/Debian)**:
```bash
sudo apt install yt-dlp
# OR via pip:
pip3 install yt-dlp
```

**Linux (Fedora)**:
```bash
sudo dnf install yt-dlp
```

**macOS**:
```bash
brew install yt-dlp
```

**Windows**:
```bash
# Via pip (requires Python 3.8+)
pip install yt-dlp
# OR download from: https://github.com/yt-dlp/yt-dlp/releases
```

**Verify installation:**
```bash
yt-dlp --version  # Should show version info
```

### Platform-Specific Dependencies

#### Linux (Ubuntu/Debian)
GTK4 development libraries are required for building:

```bash
sudo apt update
sudo apt install libgtk-4-dev build-essential pkg-config
```

Optional but recommended for faster builds:
```bash
sudo apt install mold  # Fast linker
```

#### Linux (Fedora)
```bash
sudo dnf install gtk4-devel gcc pkg-config
```

#### macOS
```bash
brew install gtk4 pkg-config
```

Note: Requires Xcode Command Line Tools:
```bash
xcode-select --install
```

#### Windows
VDownloader can be built on Windows using MSYS2.

**Install MSYS2**: https://www.msys2.org/

**In MSYS2 UCRT64 terminal**:
```bash
pacman -S mingw-w64-ucrt-x86_64-gtk4 mingw-w64-ucrt-x86_64-toolchain
```

Note: Ensure you're in the UCRT64 environment:
- Launch "MSYS2 UCRT64" (not MinGW64 or MINGW32)
- Or in any MSYS2 terminal: `source /etc/profile`

## Installation

### Clone the Repository

```bash
git clone https://github.com/st93642/VDownloader.git
cd VDownloader
```

### Check Out Development Branch

```bash
git branch -a  # List all branches
git checkout <branch-name>  # or stay on main
```

### Verify Build Environment

```bash
# Test Rust toolchain
rustc --version
cargo --version

# Test GTK4 development headers
pkg-config --cflags gtk4-4.0  # Should output -I paths

# Test yt-dlp availability
yt-dlp --version
```

If any command fails, review the platform-specific dependencies above.

## Building the Project

### Standard Development Build

```bash
# Navigate to project directory
cd VDownloader

# Build in debug mode (faster compilation)
cargo build

# Binary is created at:
# ./target/debug/vdownloader
```

Build time: ~2-5 minutes depending on system.

### Optimized Release Build

```bash
# Build with optimizations (slower compilation, smaller binary)
cargo build --release

# Binary is created at:
# ./target/release/vdownloader
```

Build time: ~5-15 minutes depending on system.

### Clean Build

If you encounter build issues, clean and rebuild:

```bash
# Remove all build artifacts
cargo clean

# Rebuild from scratch
cargo build --release
```

### Troubleshooting Build Failures

**"Package not found" for GTK4**:
- Ensure GTK4 development libraries are installed (see platform-specific above)
- Run: `pkg-config --cflags gtk4-4.0`
- If not found, install the GTK4 dev package for your distribution

**"Link error" / "undefined reference"**:
- Clean and rebuild: `cargo clean && cargo build`
- On Linux, try using the `mold` linker: `RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=mold" cargo build`

**"yt-dlp not found"**:
- This only affects runtime, not compilation
- Install yt-dlp using your package manager (see Prerequisites)

## Running the Application

### Running Debug Build

```bash
# Run from project root
cargo run

# With logging output
RUST_LOG=info cargo run
RUST_LOG=debug cargo run
```

### Running Release Build

```bash
# Run the compiled binary directly
./target/release/vdownloader

# With logging
RUST_LOG=info ./target/release/vdownloader
```

### Setting Logging Levels

VDownloader uses the standard Rust logging framework. Control output with `RUST_LOG`:

```bash
RUST_LOG=error cargo run     # Only errors
RUST_LOG=warn cargo run      # Warnings and errors
RUST_LOG=info cargo run      # Info, warnings, errors
RUST_LOG=debug cargo run     # All messages (verbose)
RUST_LOG=trace cargo run     # Ultra-verbose (rarely needed)

# Module-specific logging
RUST_LOG=vdownloader::core::downloader=debug cargo run
RUST_LOG=vdownloader::ui=info cargo run
```

### Environment Variables

| Variable | Purpose | Example |
|----------|---------|---------|
| `RUST_LOG` | Logging level | `RUST_LOG=info` |
| `RUST_BACKTRACE` | Stack traces on panic | `RUST_BACKTRACE=1` |

## Development Workflow

### Code Quality Tools

VDownloader requires code to pass formatting and linting checks.

#### Code Formatting

```bash
# Check if code is formatted
cargo fmt --check

# Automatically format code
cargo fmt
```

Format code before committing!

#### Linting

```bash
# Run clippy linter for warnings and suggestions
cargo clippy

# Fix clippy warnings automatically (when possible)
cargo clippy --fix
```

Address all clippy warnings before committing.

#### Type Checking

```bash
# Check types without building
cargo check

# Fast way to validate code without compilation
cargo check --release
```

### Running Tests

VDownloader includes comprehensive unit tests.

```bash
# Run all tests
cargo test

# Run with output from successful tests
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in a specific module
cargo test core::downloader::

# Run tests in release mode (slower compilation, faster execution)
cargo test --release
```

**Test Coverage**:
- Platform detection (YouTube, TikTok, Twitter, etc.)
- URL validation
- Output directory validation
- Queue operations
- Search result parsing
- Error handling

### Common Development Tasks

#### Modifying Core Logic

Example: Adding support for a new platform

1. Edit `src/core/downloader.rs` - Add to `Platform` enum
2. Edit `src/core/downloader.rs` - Update `detect_platform()`
3. Edit `src/core/search.rs` - Update `platform_from_hint()`
4. Add tests in same file with `#[cfg(test)]`

```bash
# Verify your changes compile
cargo check

# Run tests
cargo test

# Format and lint
cargo fmt
cargo clippy
```

#### Modifying UI

Example: Adding a new button to the search view

1. Edit `src/ui/components/search_view.rs`
2. Update `SearchView` struct and methods
3. Rebuild and test visually

```bash
# Rebuild and run with logging
RUST_LOG=info cargo run
```

#### Adding a Dependency

Example: Adding a new crate

1. Add to `Cargo.toml`:
   ```toml
   [dependencies]
   new_crate = "1.0"
   ```

2. Use in code:
   ```rust
   use new_crate::SomeType;
   ```

3. Build to fetch and verify:
   ```bash
   cargo build
   ```

### Git Workflow

#### Before Committing

```bash
# 1. Format code
cargo fmt

# 2. Lint and check
cargo clippy

# 3. Run tests
cargo test

# 4. Verify compile
cargo build
```

#### Commit Message Format

Follow conventional commits:
```
feat: Add search result pagination
fix: Correct platform detection for YouTube shorts
docs: Update ARCHITECTURE.md
refactor: Simplify error handling in downloader
test: Add platform detection tests
```

## Advanced Setup

### Using a Faster Linker (Linux)

The Mold linker can significantly speed up linking:

```bash
# Install mold
sudo apt install mold  # Ubuntu/Debian
sudo dnf install mold  # Fedora

# Build with mold
RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=mold" cargo build
```

### IDE Setup

#### VS Code

1. Install Rust Analyzer extension
2. Install CodeLLDB for debugging
3. Create `.vscode/settings.json`:
   ```json
   {
     "rust-analyzer.checkOnSave.command": "clippy"
   }
   ```

#### CLion / IntelliJ IDEA

1. Install Rust plugin
2. Open project root
3. Configure Rust toolchain: Settings → Languages & Frameworks → Rust

### Debugging

#### Using RUST_BACKTRACE

On panic, get full stack trace:
```bash
RUST_BACKTRACE=full cargo run
```

#### Using a Debugger (GDB on Linux)

```bash
# Build with debug symbols
cargo build

# Run with GDB
gdb ./target/debug/vdownloader
(gdb) run
(gdb) bt  # Full backtrace on crash
```

#### Using LLDB (macOS)

```bash
# Run with LLDB
lldb ./target/release/vdownloader
(lldb) run
(lldb) bt
```

## Cross-Platform Building

### Building for Different Targets

```bash
# List installed toolchains
rustup toolchain list

# Add a new target
rustup target add x86_64-unknown-linux-musl

# Build for specific target
cargo build --release --target x86_64-unknown-linux-musl
```

See [BUILD.md](BUILD.md) for platform-specific build instructions.

## Continuous Integration

VDownloader uses GitHub Actions for CI/CD. The workflow:

1. **On push**: Format check, clippy, tests
2. **On PR**: Same checks plus coverage
3. **On release**: Build binaries for all platforms

Check `.github/workflows/` for workflow definitions.

### Running CI Locally

Use `act` to run GitHub Actions locally:

```bash
# Install act (requires Docker)
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | bash

# Run workflow
act

# Run specific job
act -j test
```

## Documentation

### Generating Documentation

```bash
# Generate and open HTML docs for dependencies
cargo doc --open

# Document only this project
cargo doc --no-deps --open
```

### Reading Codebase

Start with:
1. [README.md](README.md) - Overview and quick start
2. [ARCHITECTURE.md](ARCHITECTURE.md) - System design
3. [CODEBASE_INDEX.md](CODEBASE_INDEX.md) - File and module reference
4. [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md) - How to modify code

## Troubleshooting Development Issues

### Issue: "error: linker `cc` not found"

**Solution**: Install build tools
- Ubuntu: `sudo apt install build-essential pkg-config`
- Fedora: `sudo dnf install gcc pkg-config`
- macOS: `xcode-select --install`

### Issue: "cannot find -lgtk-4"

**Solution**: Install GTK4 development package
- Ubuntu: `sudo apt install libgtk-4-dev`
- Fedora: `sudo dnf install gtk4-devel`
- macOS: `brew install gtk4`

### Issue: "yt-dlp: command not found" at runtime

**Solution**: Install yt-dlp
```bash
pip3 install yt-dlp  # Works on all platforms
```

### Issue: Tests pass locally but fail in CI

**Solutions**:
- Ensure `cargo fmt` and `cargo clippy` pass
- Check that tests use `#[tokio::test]` for async tests
- Verify no hardcoded file paths

### Issue: UI doesn't appear when running via `cargo run`

**Solution**: VDownloader requires a display server
- **Linux**: Check `echo $DISPLAY` (requires X11 or Wayland)
- **macOS**: Should work out of box
- **Windows**: Should work out of box

For headless testing, build succeeds but can't run GUI.

### Issue: Slow builds

**Solutions**:
1. Use debug build for development: `cargo build` (not `--release`)
2. Use faster linker: `mold` (Linux)
3. Use incremental compilation: enabled by default
4. Close other applications to free RAM

### Issue: "This application failed to start because it could not find or load the Qt platform plugin"

**Solution**: Not a Qt application - this shouldn't occur with GTK4 VDownloader.

## Getting Help

- Check [CODEBASE_INDEX.md](CODEBASE_INDEX.md) for module reference
- Review [ARCHITECTURE.md](ARCHITECTURE.md) for design decisions
- Open an issue on GitHub with:
  - Error message or unexpected behavior
  - Build environment (OS, Rust version)
  - Steps to reproduce
  - Output of `cargo --version` and `rustc --version`

## Next Steps

After setup is complete:

1. **Explore the codebase**:
   - Read [CODEBASE_INDEX.md](CODEBASE_INDEX.md)
   - Look at `src/main.rs` as entry point

2. **Run the application**:
   - `cargo run` to build and execute
   - Use search and download features
   - Check logs: `RUST_LOG=info cargo run`

3. **Make a change**:
   - Try adding a new platform (see [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md))
   - Run tests: `cargo test`
   - Format code: `cargo fmt`

4. **Submit improvements**:
   - Fork the repository
   - Create feature branch
   - Make changes and test
   - Submit pull request

## Summary

| Task | Command |
|------|---------|
| Build (debug) | `cargo build` |
| Build (release) | `cargo build --release` |
| Run | `cargo run` or `./target/release/vdownloader` |
| Test | `cargo test` |
| Format | `cargo fmt` |
| Lint | `cargo clippy` |
| Clean | `cargo clean` |
| Check only | `cargo check` |
| Run with logs | `RUST_LOG=info cargo run` |
| Generate docs | `cargo doc --open` |
