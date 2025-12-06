# Quick Start Guide - VDownloader Development

## Selected Technology Stack

### Video Extraction Library
**Choice**: `youtube_dl` Rust crate v0.10.0

**Why**: 
- Wraps yt-dlp CLI (137K+ stars, actively maintained)
- Supports all required platforms: YouTube, TikTok, Twitter, Instagram, Reddit, and 1000+ more
- MIT/Apache-2.0 license (compatible)
- Clean Rust API with async support
- Platform API changes handled automatically by yt-dlp team

## Prerequisites

### System Requirements
1. **Rust** (latest stable)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **GTK4 Development Libraries**
   
   **Linux (Ubuntu/Debian):**
   ```bash
   sudo apt install libgtk-4-dev build-essential pkg-config
   ```
   
   **Linux (Fedora):**
   ```bash
   sudo dnf install gtk4-devel gcc pkg-config
   ```
   
   **macOS:**
   ```bash
   brew install gtk4
   ```
   
   **Windows:**
   Follow GTK4 installation guide for MSYS2

3. **yt-dlp** (Runtime Dependency)
   
   **Linux (apt):**
   ```bash
   sudo apt install yt-dlp
   ```
   
   **Linux/macOS (pip):**
   ```bash
   pip3 install yt-dlp
   ```
   
   **macOS (Homebrew):**
   ```bash
   brew install yt-dlp
   ```
   
   **Windows (pip):**
   ```bash
   pip install yt-dlp
   ```

## Initial Project Setup

### 1. Initialize Cargo Project (If not done)
```bash
cargo init --name vdownloader
```

### 2. Add Dependencies to Cargo.toml
```toml
[package]
name = "vdownloader"
version = "0.1.0"
edition = "2021"

[dependencies]
gtk4 = { version = "0.9", package = "gtk4" }
youtube_dl = "0.10.0"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
anyhow = "1"
```

### 3. Verify yt-dlp Installation
```bash
yt-dlp --version
```

### 4. Test youtube_dl Crate
Create a simple test to ensure everything works:

```rust
// src/main.rs (test)
use youtube_dl::YoutubeDl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test with a short video URL
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
    
    let output = YoutubeDl::new(url)
        .socket_timeout("15")
        .run()?;
    
    if let Some(video) = output.into_single_video() {
        println!("Title: {}", video.title);
        println!("Duration: {} seconds", video.duration.unwrap_or(0.0));
        println!("URL validation successful!");
    }
    
    Ok(())
}
```

Run test:
```bash
cargo run
```

## Basic Implementation Example

### Fetch Video Metadata
```rust
use youtube_dl::{YoutubeDl, SingleVideo};

async fn get_video_info(url: &str) -> Result<SingleVideo, Box<dyn std::error::Error>> {
    let output = YoutubeDl::new(url)
        .socket_timeout("15")
        .run()?;
    
    let video = output.into_single_video()
        .ok_or("Not a single video")?;
    
    Ok(video)
}
```

### Download Video
```rust
use youtube_dl::YoutubeDl;

async fn download_video(
    url: &str, 
    output_dir: &str
) -> Result<(), Box<dyn std::error::Error>> {
    YoutubeDl::new(url)
        .download(true)
        .output_directory(output_dir)
        .run()?;
    
    Ok(())
}
```

### With Format Selection
```rust
async fn download_best_quality(
    url: &str,
    output_dir: &str
) -> Result<(), Box<dyn std::error::Error>> {
    YoutubeDl::new(url)
        .download(true)
        .format("bestvideo+bestaudio/best")
        .output_directory(output_dir)
        .output_template("%(title)s.%(ext)s")
        .run()?;
    
    Ok(())
}
```

## Supported Platforms

The chosen solution supports all target platforms:

- ✅ **YouTube** (youtube.com, youtu.be)
- ✅ **TikTok** (tiktok.com, vm.tiktok.com)
- ✅ **Twitter/X** (twitter.com, x.com)
- ✅ **Instagram** (instagram.com)
- ✅ **Reddit** (reddit.com, v.redd.it)
- ✅ **1000+ other sites** (see yt-dlp documentation)

## Error Handling

```rust
use youtube_dl::YoutubeDl;
use anyhow::{Result, Context};

async fn safe_download(url: &str) -> Result<()> {
    YoutubeDl::new(url)
        .socket_timeout("15")
        .download(true)
        .run()
        .context("Failed to download video")?;
    
    Ok(())
}
```

## Next Development Steps

1. **Create Basic GTK4 Window**
   - URL input field
   - Download button
   - Status label

2. **Implement Download Service**
   - Wrap youtube_dl crate
   - Add error handling
   - Progress tracking

3. **Connect UI to Service**
   - Button click handler
   - Async download in background
   - Update UI with progress

4. **Add Features**
   - Format selection
   - Download queue
   - History tracking

## Troubleshooting

### yt-dlp Not Found
**Error**: `youtube-dl not found in PATH`

**Solution**: Install yt-dlp using one of the methods above

### GTK4 Build Errors
**Error**: `Package gtk4 was not found`

**Solution**: Install GTK4 development libraries for your platform

### Network Timeouts
**Error**: Download hangs or times out

**Solution**: 
```rust
YoutubeDl::new(url)
    .socket_timeout("30")  // Increase timeout
    .run()?;
```

### Platform Extraction Errors
**Error**: `Unable to extract video info`

**Solution**: Update yt-dlp:
```bash
pip3 install --upgrade yt-dlp
```

## Resources

- [Full Research Document](../RESEARCH_VIDEO_EXTRACTION.md)
- [Architecture Document](./ARCHITECTURE.md)
- [youtube_dl crate docs](https://docs.rs/youtube_dl/0.10.0)
- [yt-dlp documentation](https://github.com/yt-dlp/yt-dlp/wiki)
- [GTK4 Rust bindings](https://gtk-rs.org/gtk4-rs/)

## Development Workflow

```bash
# 1. Make changes to code
# 2. Build and test
cargo build

# 3. Run application
cargo run

# 4. Run tests (when implemented)
cargo test

# 5. Format code
cargo fmt

# 6. Check for issues
cargo clippy
```

## Summary

✅ **Ready to Start Development**
- Video extraction library selected and justified
- Dependencies documented
- Installation instructions provided
- Basic usage examples included
- All requirements met (multi-platform, license, maintenance, ease of integration)

Start building the GTK4 UI and integrate the video download functionality!
