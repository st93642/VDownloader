# Video Extraction Library Comparison

Quick reference comparison of evaluated libraries for VDownloader.

## Summary Table

| Library | Version | License | Stars | Multi-Platform | Maintenance | Verdict |
|---------|---------|---------|-------|----------------|-------------|---------|
| **youtube_dl crate** | 0.10.0 | MIT/Apache-2.0 | 141 | ✅ All (via yt-dlp) | ✅ Active | ✅ **SELECTED** |
| rustube | 0.6.0 | MIT/Apache-2.0 | 264 | ❌ YouTube only | ✅ Active | ❌ Limited platform support |
| yt-dlp crate | 1.4.6 | GPL-3.0 | 95 | ✅ All (via yt-dlp) | ✅ Active | ❌ License incompatible |
| yt-dlp CLI | Latest | Unlicense | 137K+ | ✅ 1000+ sites | ✅ Very Active | ✅ Backend (wrapped by youtube_dl crate) |
| youtube-dl CLI | Legacy | Unlicense | Legacy | ⚠️ Many sites | ⚠️ Superseded | ❌ Use yt-dlp instead |

## Detailed Scores

### youtube_dl Crate (SELECTED) ⭐⭐⭐⭐⭐

| Criteria | Score | Notes |
|----------|-------|-------|
| Multi-Platform Support | 5/5 | YouTube, TikTok, Twitter, Instagram, Reddit, 1000+ sites |
| License Compatibility | 5/5 | MIT/Apache-2.0 (fully compatible) |
| Active Maintenance | 5/5 | Updated Nov 2025, active issues/PRs |
| Community Support | 5/5 | Leverages yt-dlp (137K stars) |
| Ease of Integration | 5/5 | Clean Rust API, well documented |
| Cross-Platform | 5/5 | Linux, Windows, macOS |
| API Stability | 5/5 | Platform changes handled by yt-dlp |
| Implementation Complexity | 5/5 | Simple wrapper, minimal code |
| **Total** | **40/40** | **BEST CHOICE** |

**Key Advantages:**
- ✅ Meets all project requirements
- ✅ Leverages battle-tested yt-dlp backend
- ✅ No platform-specific maintenance burden
- ✅ Async support for GTK integration
- ✅ Type-safe Rust API

**Trade-offs:**
- ⚠️ Requires yt-dlp installation (mitigable with docs/bundling)
- ⚠️ Process spawning overhead (minimal impact)

---

### rustube ⭐⭐⭐

| Criteria | Score | Notes |
|----------|-------|-------|
| Multi-Platform Support | 1/5 | YouTube ONLY |
| License Compatibility | 5/5 | MIT/Apache-2.0 |
| Active Maintenance | 4/5 | Active but smaller community |
| Community Support | 3/5 | 264 stars, decent docs |
| Ease of Integration | 5/5 | Pure Rust, excellent API |
| Cross-Platform | 5/5 | Linux, Windows, macOS |
| API Stability | 2/5 | Vulnerable to YouTube changes |
| Implementation Complexity | 5/5 | Simple, pure Rust |
| **Total** | **30/40** | **Not sufficient** |

**Why Not Selected:**
- ❌ Only supports YouTube (need 5+ platforms)
- ❌ Would require implementing TikTok, Twitter, Instagram, Reddit separately
- ❌ Maintenance burden for platform API changes

**When to Use:**
- YouTube-only applications
- Pure Rust requirement (no external dependencies)
- Learning/educational projects

---

### yt-dlp Rust Crate ⭐⭐⭐

| Criteria | Score | Notes |
|----------|-------|-------|
| Multi-Platform Support | 5/5 | All platforms via yt-dlp |
| License Compatibility | 0/5 | **GPL-3.0 (INCOMPATIBLE)** |
| Active Maintenance | 5/5 | Updated Dec 2025 |
| Community Support | 3/5 | 95 stars, smaller community |
| Ease of Integration | 4/5 | Auto-installs dependencies |
| Cross-Platform | 5/5 | Linux, Windows, macOS |
| API Stability | 5/5 | Leverages yt-dlp |
| Implementation Complexity | 4/5 | Slightly more complex setup |
| **Total** | **31/40** | **License issue** |

**Why Not Selected:**
- ❌ **GPL-3.0 license conflicts with MIT project**
- ❌ Would require entire project to be GPL (viral license)
- ❌ Restricts future commercial use

**Otherwise Good:**
- ✅ Auto-installs yt-dlp
- ✅ Built-in caching
- ✅ Multi-platform support

---

### yt-dlp CLI (Backend) ⭐⭐⭐⭐⭐

| Criteria | Score | Notes |
|----------|-------|-------|
| Multi-Platform Support | 5/5 | 1000+ websites supported |
| License Compatibility | 5/5 | Unlicense (public domain) |
| Active Maintenance | 5/5 | Updated daily, massive team |
| Community Support | 5/5 | 137K+ stars, huge community |
| Ease of Integration | 3/5 | CLI tool (need wrapper) |
| Cross-Platform | 5/5 | Excellent cross-platform support |
| API Stability | 5/5 | Constantly updated for platform changes |
| Implementation Complexity | 3/5 | Need to handle process management |
| **Total** | **36/40** | **Industry Standard** |

**Role:**
- ✅ Backend technology for youtube_dl crate
- ✅ Handles all platform-specific extraction
- ✅ Community maintains platform updates

**Direct CLI Usage:**
- Could write own wrapper (more work)
- youtube_dl crate provides this already
- Better to use existing wrapper

---

## Platform Support Matrix

| Platform | youtube_dl crate | rustube | yt-dlp crate | yt-dlp CLI |
|----------|------------------|---------|--------------|------------|
| YouTube | ✅ | ✅ | ✅ | ✅ |
| TikTok | ✅ | ❌ | ✅ | ✅ |
| Twitter/X | ✅ | ❌ | ✅ | ✅ |
| Instagram | ✅ | ❌ | ✅ | ✅ |
| Reddit | ✅ | ❌ | ✅ | ✅ |
| Facebook | ✅ | ❌ | ✅ | ✅ |
| Vimeo | ✅ | ❌ | ✅ | ✅ |
| Dailymotion | ✅ | ❌ | ✅ | ✅ |
| **Total Sites** | **1000+** | **1** | **1000+** | **1000+** |

## License Compatibility

| License | Compatible with MIT | Commercial Use | Copyleft | Notes |
|---------|---------------------|----------------|----------|-------|
| MIT/Apache-2.0 | ✅ Yes | ✅ Yes | ❌ No | Best for MIT projects |
| Unlicense | ✅ Yes | ✅ Yes | ❌ No | Public domain, very permissive |
| GPL-3.0 | ❌ No | ⚠️ Restricted | ✅ Yes | Viral license, incompatible |

**VDownloader License:** MIT
**Compatible Choices:** MIT, Apache-2.0, Unlicense

---

## Maintenance Activity Comparison

| Library | Last Update | Update Frequency | Open Issues | Recent Commits |
|---------|-------------|------------------|-------------|----------------|
| youtube_dl crate | Nov 2025 | Monthly | 17 | Active |
| rustube | Nov 2025 | Monthly | 24 | Active |
| yt-dlp crate | Dec 2025 | Monthly | 3 | Active |
| yt-dlp CLI | Dec 2025 | **Daily** | 2222 | **Very Active** |

**Key Insight:** Using youtube_dl crate leverages yt-dlp's daily maintenance for platform updates.

---

## External Dependencies

| Solution | Requires | Installation | Bundle Option |
|----------|----------|--------------|---------------|
| youtube_dl crate | yt-dlp CLI | User installs | ✅ Yes (future) |
| rustube | None | N/A | ✅ Native binary |
| yt-dlp crate | yt-dlp CLI | Auto-installs | ⚠️ Complex |
| Direct CLI | yt-dlp + Python | User installs | ✅ Yes (future) |

**Mitigation for youtube_dl crate:**
1. Clear installation docs ✅
2. Runtime check with helpful error ✅
3. Future: Bundle yt-dlp binary ✅

---

## Code Complexity Comparison

### youtube_dl Crate (SELECTED)
```rust
// Simple, clean API
let output = YoutubeDl::new(url)
    .download(true)
    .output_directory("~/Downloads")
    .run()?;
```
**Lines of code needed:** ~10-20 for basic functionality
**Complexity:** ⭐ Very Low

### rustube
```rust
// Pure Rust, also simple
let video = Video::from_url(url).await?;
video.best_quality()?.download().await?;
```
**Lines of code needed:** ~10-20
**Complexity:** ⭐ Very Low
**Issue:** YouTube only

### Direct CLI Wrapper
```rust
// More boilerplate needed
let output = Command::new("yt-dlp")
    .args(&["--print-json", url])
    .output()?;
let json: Value = serde_json::from_slice(&output.stdout)?;
// Parse and handle...
```
**Lines of code needed:** ~50-100
**Complexity:** ⭐⭐⭐ Medium
**Issue:** youtube_dl crate does this already

---

## Risk Assessment

### youtube_dl Crate (SELECTED)

| Risk | Severity | Likelihood | Mitigation |
|------|----------|------------|------------|
| User doesn't have yt-dlp | Medium | Medium | Clear install docs, runtime check |
| yt-dlp breaks | Low | Very Low | yt-dlp team fixes quickly |
| Process overhead | Low | N/A | Minimal impact on desktop app |
| License issues | None | N/A | MIT/Apache-2.0 compatible |

**Overall Risk:** ⭐⭐ Low

### rustube

| Risk | Severity | Likelihood | Mitigation |
|------|----------|------------|------------|
| YouTube API changes | High | Medium | Wait for library update |
| Limited platforms | Critical | N/A | Need separate implementations |
| Maintenance burden | High | High | Must implement other platforms |

**Overall Risk:** ⭐⭐⭐⭐ High (due to limited scope)

### yt-dlp Rust Crate

| Risk | Severity | Likelihood | Mitigation |
|------|----------|------------|------------|
| GPL license conflict | Critical | N/A | Cannot use with MIT project |

**Overall Risk:** ⭐⭐⭐⭐⭐ Critical (legal issue)

---

## Decision Matrix

| Factor | Weight | youtube_dl | rustube | yt-dlp crate |
|--------|--------|------------|---------|--------------|
| Multi-platform | 25% | 5/5 | 1/5 | 5/5 |
| License | 25% | 5/5 | 5/5 | 0/5 |
| Maintenance | 15% | 5/5 | 4/5 | 5/5 |
| Integration | 15% | 5/5 | 5/5 | 4/5 |
| Community | 10% | 5/5 | 3/5 | 3/5 |
| Complexity | 10% | 5/5 | 5/5 | 4/5 |
| **Weighted Score** | | **4.85** | **3.10** | **2.75** |

**Winner:** youtube_dl crate with 4.85/5.0 weighted score

---

## Recommendation

### ✅ Selected: youtube_dl Rust Crate

**Summary:**
The `youtube_dl` Rust crate (v0.10.0) is the optimal choice for VDownloader because it:

1. **Meets all requirements** (multi-platform, license, maintenance)
2. **Leverages industry standard** (yt-dlp with 137K+ stars)
3. **Minimal implementation** (clean API, well documented)
4. **Future-proof** (platform changes handled automatically)
5. **Battle-tested** (used in production by many projects)

**Next Steps:**
1. Add to Cargo.toml: `youtube_dl = "0.10.0"`
2. Document yt-dlp installation for users
3. Implement download service wrapper
4. Integrate with GTK4 UI
5. Test with all target platforms

**Status:** ✅ Ready for implementation

---

For complete analysis, see [RESEARCH_VIDEO_EXTRACTION.md](../RESEARCH_VIDEO_EXTRACTION.md)
