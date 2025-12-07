# VDownloader Codebase Index

_Last updated: December 2025_

This document maps every major artifact in the repository, describes how modules interact, and captures the current feature set, dependencies, and outstanding work. Use it as a quick reference when planning enhancements or onboarding new contributors.

---

## 1. Repository Layout

```
VDownloader/
├── Cargo.toml
├── README.md
├── docs/
│   ├── ARCHITECTURE.md
│   └── CODEBASE_INDEX.md (this file)
├── src/
│   ├── main.rs
│   ├── core/
│   │   ├── mod.rs
│   │   ├── downloader.rs
│   │   ├── queue.rs
│   │   └── error.rs
│   └── ui/
│       ├── mod.rs
│       ├── window.rs
│       └── components/
│           ├── mod.rs
│           └── download_queue.rs
└── .github/, .gitignore, LICENSE, etc.
```

No build scripts or additional binaries exist; `cargo` drives the entire toolchain.

---

## 2. Module-by-Module Breakdown

### `src/main.rs`
- Configures logging via `env_logger`.
- Initializes a Tokio runtime so async code can run alongside GTK's event loop.
- Builds and runs the GTK application, delegating window construction to `ui::window`.

### `src/core`
- **`mod.rs`** simply re-exports the submodules.
- **`downloader.rs`**
  - Defines `Platform`, `DownloadRequest`, `DownloadStatus`, and `VideoDownloader`.
  - `VideoDownloader::download` validates input, sanitizes VK URLs, detects the platform, and invokes `perform_download` inside `tokio::task::spawn_blocking`.
  - `perform_download` calls the `yt-dlp` CLI, parses stdout for progress updates, and handles playlist/metadata flow using the `youtube_dl` crate.
  - Unit tests cover detection, validation, and sanitization helpers.
- **`queue.rs`**
  - Provides a `DownloadQueue` (in-memory HashMap guarded by `tokio::sync::RwLock`).
  - Supports add/get/update/remove/list/clear operations, returning `QueueItem` structs that hold the `DownloadRequest` plus `DownloadStatus`.
  - Not yet connected to the UI; current tests verify CRUD behavior but require maintenance (they must set the `overwrite` field).
- **`error.rs`**
  - Implements a cloneable `DownloadError` enum with variants for validation, IO, unsupported platform, extraction, networking, and cancellation issues.
  - `Result<T>` type alias standardizes error propagation.

### `src/ui`
- **`window.rs`**
  - Builds the GTK widget tree (title labels, URL entry, download path picker, overwrite checkbox, progress bar, status label, download button, queue placeholder).
  - Uses `Rc<RefCell<String>>` to share the currently selected output path between callbacks.
  - Validates URL input locally before kicking off a download via `gtk4::glib::spawn_future_local`.
  - Instantiates `VideoDownloader`, builds a `DownloadRequest`, and wires up a `std::sync::mpsc` channel to stream progress into a `ProgressBar`.
  - Centralizes user-facing error messaging in `format_error`, mapping `DownloadError` variants to human-readable strings.
- **`components/download_queue.rs`**
  - Currently renders only a placeholder frame stating "No downloads in queue". Intended to host queue visualization once `DownloadQueue` is integrated.

### `docs`
- **`ARCHITECTURE.md`** (see companion document) explains motivations, workflows, and extension points.
- **`CODEBASE_INDEX.md`** (this file) catalogs the actual code artifacts and their state.

---

## 3. Platform Support Inventory

`Platform` (defined in `core::downloader`) currently includes:

| Variant | Detection Logic |
|---------|-----------------|
| `YouTube` | URL contains `youtube.com` or `youtu.be` |
| `TikTok` | URL contains `tiktok.com` |
| `Twitter` | URL contains `twitter.com` or `x.com` |
| `Instagram` | URL contains `instagram.com` |
| `Reddit` | URL contains `reddit.com` |
| `Vk` | URL contains `vk.com` or `vkvideo.ru` |
| `Rutube` | URL contains `rutube.ru` |
| `Dzen` | URL contains `dzen.ru` |
| `Other` | Fallback |

The `Platform` enum and detection logic cover all platforms listed in the README. Adding new platforms requires extending the enum and detector.

---

## 4. Feature Inventory & Current State

| Area | Status | Notes |
|------|--------|-------|
| Core single-download flow | ✅ Working | Validates URLs, supports custom file path selection, and surfaces progress/errors in the UI. |
| Platform detection | ✅ Working | Simple substring-based detection only. |
| Progress feedback | ✅ Working | `yt-dlp` stdout parsing updates a `ProgressBar` through an mpsc + GTK timeout pipe. |
| Download queue infrastructure | ⚠️ Partial | Queue structs/tests exist but no UI integration or scheduling logic. |
| Download queue UI | ⚠️ Placeholder | Static frame with "No downloads in queue" text. |
| Search functionality | 🚧 Not implemented | No modules, types, or configs exist for search/API providers. |
| Configuration management | 🚧 Not implemented | No config files; only `HOME`/`USERPROFILE` are read. |
| Settings/history | 🚧 Not implemented | Mentioned in README but absent in code. |

---

## 5. Build & Toolchain

- Standard Cargo workflows:
  - `cargo build` / `cargo build --release`
  - `cargo run`
  - `RUST_LOG=info cargo run`
  - `cargo test`, `cargo fmt`, `cargo clippy`
- GTK4 development headers must be installed on the host system.
- The `yt-dlp` executable must be present in `$PATH` at runtime.
- No custom build.rs or additional binaries exist.

---

## 6. External Dependencies & Purpose

(Directly derived from `Cargo.toml`)

| Crate | Version | Usage in Code |
|-------|---------|---------------|
| `gtk4` | 0.9 (`v4_10`) | UI widgets, dialogs, application shell |
| `glib` | 0.20 | GTK settings, idle/timeout sources, shared types |
| `gio` | 0.20 | File dialog support |
| `tokio` | 1.35 (full) | Async runtime backing downloads/queue |
| `youtube_dl` | 0.9 | Metadata/playlist probing before running CLI |
| `regex` | 1.12.2 | VK URL sanitization |
| `serde` | 1.0 (`derive`) | Serialization of request/status types |
| `serde_json` | 1.0 | Placeholder for future persistence/serialization |
| `log` | 0.4 | Structured logging API |
| `env_logger` | 0.11 | Logging backend |
| `anyhow` | 1.0 | Reserved for general-purpose errors (minimal use today) |
| `thiserror` | 1.0 | Custom `DownloadError` derive |

External binaries:
- `yt-dlp` (CLI) – required for actual downloads
- GTK4 runtime – required to run the UI application

---

## 7. Configuration, Environment, and Secrets

- No `.env` loader or config file is shipped.
- `window.rs` inspects `HOME` or `USERPROFILE` to set a default `~/Videos/video.mp4` path.
- Users configure download destinations via the GTK file dialog.
- Any future search/API work must add an explicit configuration mechanism (TOML, JSON, or env var parsing) since nothing exists today.

---

## 8. API & Data Structures

### Core Types
- `VideoDownloader` – public facade exposing:
  - `new(output_directory: String)` constructor
  - `download(request, on_progress)` async method (returns `Result<String, DownloadError>`)
  - Helpers `validate_url`, `validate_output_directory`, `sanitize_url`, `detect_platform`
- `DownloadRequest` – DTO with fields `url`, `platform`, `output_path: Option<String>`, `overwrite: bool`.
- `DownloadStatus` – enum representing queue states (currently unused outside the queue module).
- `DownloadQueue` – async-safe queue with `add/get/update_status/remove/list_all/clear` methods.

### UI Interaction
- `format_error(&DownloadError) -> String` centralizes text shown in the status label.
- GTK widgets (entries, labels, buttons) are constructed once and then cloned into closures (`connect_clicked`, `spawn_future_local`).

### Common Pattern Example

Creating a new download request from the UI:
```rust
let platform = VideoDownloader::detect_platform(url);
let request = DownloadRequest {
    url: url.to_string(),
    platform,
    output_path: Some(selected_path.clone()),
    overwrite: overwrite_check.is_active(),
};
```

This pattern should be reused anywhere the core downloader is invoked.

---

## 9. UI Structure & State Flow

1. **Header & subtitle** describe the app.
2. **URL entry + clear button** capture the video link.
3. **Download location section** shows the currently selected file path and launches a `FileDialog` when "Save As" is clicked.
4. **Overwrite checkbox** toggles whether `yt-dlp` receives `--force-overwrite`.
5. **Download button** triggers validation, UI state changes (disable button, show progress), and spawns the async task.
6. **Progress bar & status label** reflect updates from the download task via the mpsc channel and centralized error formatting.
7. **Queue placeholder frame** is rendered but not dynamic.

State sharing strategy:
- Long-lived values (selected path) use `Rc<RefCell<_>>`.
- GTK widgets are cloned and moved into closures to satisfy the `'static` lifetime required by GTK signals and `spawn_future_local`.
- Progress updates are marshaled through a channel to avoid crossing thread boundaries directly.

---

## 10. Testing Setup & Coverage

- `core::downloader::tests` (plain `#[test]`) cover:
  - Platform detection for YouTube/TikTok/Twitter/X.
  - URL validation (good/bad cases, length checks).
  - Output directory validation (using `std::env::temp_dir`).
  - VK-specific URL sanitization.
- `core::queue::tests` (`#[tokio::test]`) cover add/get/list/remove behavior.
- No tests exist for UI code or the actual interaction with `yt-dlp`.
- Test debt: queue tests must be updated to initialize the `overwrite` field; otherwise they fail to compile under Rust 2021.

---

## 11. Known Gaps, Technical Debt & Branch Notes

- **Queue/UI disconnect:** queue data never surfaces in the interface.
- **Platform coverage mismatch:** README lists VK Video, Rutube, and Dzen as supported, but code does not.
- **Search/historical features absent:** despite being in the roadmap, there are zero modules referencing search providers or history storage.
- **Config/documentation drift:** README references `docs/` even though documentation was missing prior to this ticket (now addressed).
- **Tests lagging behind code:** the `DownloadRequest` constructor signature change (`overwrite` flag) was not reflected in queue tests.
- **No feature branches tracked in-code:** repository history (not inspected here) does not expose partial branches; unfinished work is instead visible via unused modules and TODO narratives in README.

---

## 12. Next Steps & Extension Hooks

- **Wire up `DownloadQueue`:** instantiate it in the UI, update status as downloads progress, and render entries in `download_queue.rs`.
- **Refactor `Platform`:** move into its own module to enable reuse and easier expansion.
- **Introduce configuration struct:** centralize defaults (download directory, concurrency, API keys) and expose them to both UI and core layers.
- **Add search module:** once requirements are clear, create `src/core/search/` with provider traits and UI affordances for querying platforms.
- **Broaden testing:** add integration tests (potentially feature-gated) that mock `yt-dlp` output to validate the full pipeline.

Refer to `docs/ARCHITECTURE.md` for a conceptual deep dive, workflow diagrams, and extension recommendations.
