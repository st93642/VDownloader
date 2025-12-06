# Video Extraction Library Research

## Overview

This document presents research findings for selecting a video extraction library for the VDownloader Rust GTK desktop application. The goal is to find the best solution for a minimal implementation that supports multiple platforms (YouTube, TikTok, Twitter, Instagram, Reddit) with cross-platform compatibility (Linux, Windows, macOS).

## Research Date
December 2025

## Options Evaluated

### 1. Rust Native Libraries

#### 1.1 rustube (v0.6.0)
- **Repository**: https://github.com/DzenanJupic/rustube
- **Stars**: 264 | **Forks**: 45 | **Open Issues**: 24
- **Last Updated**: November 2025 (Active)
- **License**: MIT/Apache-2.0 ✅
- **Description**: Pure Rust YouTube video downloader inspired by pytube

**Features:**
- Native Rust implementation (no external dependencies)
- Async/await support
- Direct YouTube API interaction
- Download and stream capabilities
- Format selection support

**Pros:**
- ✅ Pure Rust (no external dependencies)
- ✅ Type-safe API
- ✅ Async support (perfect for GTK async runtime)
- ✅ MIT/Apache-2.0 license compatible with project
- ✅ Good documentation

**Cons:**
- ❌ **YouTube ONLY** - Does not support TikTok, Twitter, Instagram, Reddit
- ❌ Vulnerable to YouTube API changes (requires library updates)
- ❌ Limited community (264 stars vs alternatives)
- ❌ No built-in format conversion

**Verdict**: ❌ Does not meet multi-platform requirements

---

#### 1.2 youtube_dl Rust Crate (v0.10.0)
- **Repository**: https://github.com/GyrosOfWar/youtube-dl-rs
- **Crate**: `youtube_dl` on crates.io
- **Stars**: 141 | **Forks**: 48 | **Open Issues**: 17
- **Last Updated**: November 2025 (Active)
- **License**: MIT/Apache-2.0 ✅
- **Description**: Rust wrapper that runs yt-dlp and parses its JSON output

**Features:**
- Wraps yt-dlp CLI tool
- Parses structured JSON output
- Supports all platforms that yt-dlp supports (1000+ websites)
- Progress tracking
- Format selection
- Playlist support

**Pros:**
- ✅ **Multi-platform support** (YouTube, TikTok, Twitter, Instagram, Reddit, etc.)
- ✅ Leverages yt-dlp's massive community and maintenance
- ✅ MIT/Apache-2.0 license compatible
- ✅ Simple, clean Rust API
- ✅ Cross-platform (Linux, Windows, macOS)
- ✅ Actively maintained
- ✅ Platform API changes handled by yt-dlp team
- ✅ Optional features for async downloads

**Cons:**
- ⚠️ Requires yt-dlp to be installed on the system
- ⚠️ External dependency (Python + yt-dlp)
- ⚠️ Slightly higher resource usage (process spawning)

**Verdict**: ✅ **STRONG CANDIDATE** - Meets all requirements

---

#### 1.3 yt-dlp Rust Crate (v1.4.6)
- **Repository**: https://github.com/boul2gom/yt-dlp
- **Crate**: `yt-dlp` on crates.io
- **Stars**: 95 | **Forks**: 32 | **Open Issues**: 3
- **Last Updated**: December 2025 (Active)
- **License**: GPL-3.0 ❌
- **Description**: Rust library with automatic yt-dlp dependency installation

**Features:**
- Auto-installs yt-dlp dependencies
- Caching support (SQLite)
- Tracing/logging support
- Multi-platform support

**Pros:**
- ✅ Auto-installs dependencies
- ✅ Multi-platform support
- ✅ Recently maintained
- ✅ Built-in caching

**Cons:**
- ❌ **GPL-3.0 license** - Incompatible with project's MIT license
- ⚠️ Smaller community
- ⚠️ Still depends on external yt-dlp installation

**Verdict**: ❌ License incompatibility (GPL vs MIT)

---

### 2. CLI Tools (Direct Wrappers)

#### 2.1 yt-dlp (Python CLI)
- **Repository**: https://github.com/yt-dlp/yt-dlp
- **Stars**: 137,336 | **Forks**: 11,068 | **Open Issues**: 2,222
- **Last Updated**: December 2025 (Very Active - daily updates)
- **License**: Unlicense (Public Domain) ✅
- **Description**: Feature-rich command-line audio/video downloader

**Features:**
- Supports **1000+ websites** including all target platforms
- Format selection and merging
- Thumbnail embedding
- Subtitle download
- Geo-bypass
- Playlist support
- Authentication support
- Regular updates (almost daily)

**Pros:**
- ✅ **Industry standard** (137K+ stars)
- ✅ **Extremely active maintenance** (updated daily)
- ✅ Supports ALL required platforms and hundreds more
- ✅ Handles platform API changes immediately
- ✅ Very permissive license (Unlicense/Public Domain)
- ✅ Cross-platform (Linux, Windows, macOS)
- ✅ Extensive documentation
- ✅ Large community support

**Cons:**
- ⚠️ Requires Python runtime
- ⚠️ External dependency management
- ⚠️ Need to handle process management in Rust

**Verdict**: ✅ **BASE TECHNOLOGY** - Best backend choice

---

#### 2.2 youtube-dl (Python CLI)
- **Repository**: https://github.com/ytdl-org/youtube-dl
- **Description**: Original YouTube downloader (predecessor to yt-dlp)

**Status:**
- Less actively maintained than yt-dlp
- yt-dlp is a fork with more features and better maintenance
- Most projects have migrated to yt-dlp

**Verdict**: ❌ Superseded by yt-dlp

---

## Comparison Matrix

| Criteria | rustube | youtube_dl crate | yt-dlp crate | yt-dlp CLI |
|----------|---------|------------------|--------------|------------|
| **Multi-platform** | ❌ YouTube only | ✅ All platforms | ✅ All platforms | ✅ All platforms |
| **License Compatible** | ✅ MIT/Apache | ✅ MIT/Apache | ❌ GPL-3.0 | ✅ Unlicense |
| **Active Maintenance** | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Very Active |
| **External Dependencies** | ❌ None | ⚠️ yt-dlp required | ⚠️ yt-dlp required | ⚠️ Python + yt-dlp |
| **Platform API Updates** | ❌ Library updates | ✅ yt-dlp handles | ✅ yt-dlp handles | ✅ Daily updates |
| **Community Size** | 264 stars | 141 stars | 95 stars | 137K+ stars |
| **Implementation Complexity** | Low | Low | Low | Medium |
| **Type Safety** | ✅ Native Rust | ✅ Rust wrapper | ✅ Rust wrapper | ⚠️ Manual parsing |
| **Cross-platform** | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes |

---

## Recommendation

### ✅ SELECTED: `youtube_dl` Rust Crate (wrapper around yt-dlp CLI)

**Rationale:**

1. **Multi-Platform Support** ⭐⭐⭐⭐⭐
   - Supports ALL required platforms (YouTube, TikTok, Twitter, Instagram, Reddit)
   - Supports 1000+ additional websites through yt-dlp
   - Future-proof for adding new platforms

2. **Maintenance & Reliability** ⭐⭐⭐⭐⭐
   - Leverages yt-dlp's massive community (137K+ stars)
   - Platform API changes handled by yt-dlp team (updated almost daily)
   - Don't need to maintain platform-specific extraction logic
   - Rust wrapper is actively maintained

3. **License Compatibility** ⭐⭐⭐⭐⭐
   - MIT/Apache-2.0 license fully compatible with project's MIT license
   - yt-dlp itself is Public Domain (Unlicense)
   - No legal restrictions for distribution

4. **Development Simplicity** ⭐⭐⭐⭐⭐
   - Clean, idiomatic Rust API
   - Minimal implementation required
   - Type-safe wrapper around JSON output
   - Good documentation

5. **Integration with GTK/Rust** ⭐⭐⭐⭐
   - Async support available through `tokio` feature
   - Compatible with GTK's async runtime
   - Easy to show download progress in UI
   - Can handle multiple simultaneous downloads

6. **Cross-Platform** ⭐⭐⭐⭐⭐
   - Works on Linux, Windows, macOS
   - yt-dlp available on all platforms

**Trade-offs:**
- **External Dependency**: Requires yt-dlp to be installed on the user's system
  - **Mitigation**: Provide clear installation instructions in README
  - **Future Option**: Bundle yt-dlp binary with the application
  - **Alternative**: Auto-download yt-dlp on first run

---

## Implementation Plan

### Phase 1: Basic Integration (Minimal MVP)

```rust
// Add to Cargo.toml
[dependencies]
youtube_dl = "0.10.0"
tokio = { version = "1", features = ["full"] }

// Basic usage example
use youtube_dl::YoutubeDl;

async fn download_video(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = YoutubeDl::new(url)
        .socket_timeout("15")
        .output()?;
    
    println!("Title: {}", output.into_single_video()?.title);
    Ok(())
}
```

### Phase 2: GTK Integration

```rust
// Download with progress tracking for GTK UI
use youtube_dl::YoutubeDl;
use gtk::prelude::*;

async fn download_with_progress(
    url: &str,
    progress_bar: gtk::ProgressBar
) -> Result<(), Box<dyn std::error::Error>> {
    let output = YoutubeDl::new(url)
        .download(true)
        .output_directory("~/Downloads")
        .run_async()
        .await?;
    
    // Update GTK progress bar
    progress_bar.set_fraction(1.0);
    Ok(())
}
```

### Phase 3: Advanced Features (Future)

- Format selection (video quality, audio only, etc.)
- Playlist support
- Download queue management
- Thumbnail extraction
- Metadata display

---

## Dependency Management

### User Installation (Recommended for MVP)

Users need to install yt-dlp on their system:

**Linux (apt):**
```bash
sudo apt install yt-dlp
```

**Linux (pip):**
```bash
pip3 install yt-dlp
```

**macOS (Homebrew):**
```bash
brew install yt-dlp
```

**Windows (Chocolatey):**
```bash
choco install yt-dlp
```

**Windows (pip):**
```bash
pip install yt-dlp
```

### Future Enhancement: Bundled Binary

For easier distribution, consider bundling yt-dlp binary:
- Include platform-specific yt-dlp binaries
- Auto-detect and use bundled version if system version not found
- Update bundled version periodically

---

## Alternative Approaches Considered

### Why Not Pure Rust (rustube)?
- ❌ YouTube-only support doesn't meet requirements
- ❌ Need to implement TikTok, Twitter, Instagram, Reddit separately
- ❌ Maintenance burden for platform API changes
- ❌ Each platform has different authentication and extraction logic

### Why Not GPL yt-dlp Crate?
- ❌ GPL-3.0 license incompatible with MIT project
- ❌ Would require entire project to be GPL (viral license)
- ❌ Restricts future commercial use or closed-source forks

### Why Not Direct CLI Wrapper?
- ⚠️ youtube_dl crate provides this but with better API
- ⚠️ Would need to manually parse JSON output
- ⚠️ More boilerplate code
- ✅ Could be a fallback if crate has issues

---

## Risk Assessment

### Technical Risks

1. **External Dependency (yt-dlp)**
   - **Risk**: Users might not have yt-dlp installed
   - **Mitigation**: Clear installation instructions, runtime check with helpful error
   - **Alternative**: Auto-download or bundle yt-dlp

2. **Platform API Changes**
   - **Risk**: Platforms change their APIs breaking downloads
   - **Mitigation**: yt-dlp team handles this (updated almost daily)
   - **Action**: Recommend users keep yt-dlp updated

3. **Process Spawning Overhead**
   - **Risk**: Spawning processes for each download
   - **Mitigation**: Use async operations, connection pooling for multiple downloads
   - **Impact**: Minimal for desktop application

### Legal Risks

1. **Terms of Service**
   - **Risk**: Some platforms prohibit downloading
   - **Mitigation**: Add disclaimer about following platform ToS
   - **Note**: This applies to any video downloader, not specific to our choice

2. **License Compliance**
   - **Risk**: License conflicts
   - **Status**: ✅ CLEAR - MIT + Unlicense are fully compatible

---

## Success Criteria

✅ **All criteria met with youtube_dl crate:**

- [x] Multi-platform support (YouTube, TikTok, Twitter, Instagram, Reddit)
- [x] Cross-platform compatibility (Linux, Windows, macOS)
- [x] Active maintenance and community support
- [x] MIT license compatibility
- [x] Easy integration with Rust/GTK
- [x] Minimal implementation complexity
- [x] Handles platform API changes automatically
- [x] Type-safe Rust API
- [x] Async support for GTK integration

---

## Next Steps

1. **Add Dependency**
   - Add `youtube_dl = "0.10.0"` to Cargo.toml
   - Add `tokio` with appropriate features

2. **Create Abstraction Layer**
   - Build a `VideoDownloader` service in Rust
   - Wrap youtube_dl crate with app-specific API
   - Handle errors gracefully

3. **Implement Basic Download**
   - URL validation
   - Simple download to default directory
   - Display video metadata (title, duration, etc.)

4. **GTK Integration**
   - Connect download service to GTK UI
   - Show progress updates
   - Handle download completion/errors

5. **Documentation**
   - Update README with yt-dlp installation instructions
   - Add troubleshooting section
   - Document supported platforms

6. **Testing**
   - Test with each platform (YouTube, TikTok, Twitter, Instagram, Reddit)
   - Test error handling (invalid URLs, network issues)
   - Test on all target platforms (Linux, Windows, macOS)

---

## References

- [youtube_dl crate documentation](https://docs.rs/youtube_dl/0.10.0)
- [youtube_dl crate repository](https://github.com/GyrosOfWar/youtube-dl-rs)
- [yt-dlp official repository](https://github.com/yt-dlp/yt-dlp)
- [yt-dlp documentation](https://github.com/yt-dlp/yt-dlp/wiki)
- [rustube repository](https://github.com/DzenanJupic/rustube)

---

## Conclusion

The **youtube_dl Rust crate** (wrapping yt-dlp CLI) is the optimal choice for VDownloader because it:

1. ✅ Meets all multi-platform requirements
2. ✅ Leverages battle-tested, actively maintained yt-dlp
3. ✅ Provides clean Rust API with minimal implementation
4. ✅ Compatible with MIT license
5. ✅ Handles platform changes automatically
6. ✅ Cross-platform support
7. ✅ Easy GTK integration with async support

The trade-off of requiring yt-dlp as an external dependency is acceptable and mitigable through clear documentation and potential future bundling.

**Status**: Ready for implementation ✅
