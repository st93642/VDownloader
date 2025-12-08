# VDownloader Architecture

Comprehensive guide to VDownloader's system design, layered architecture,
and data flow.

## Architecture Overview

VDownloader follows a **layered architecture** separating presentation
concerns from business logic:

```text
┌─────────────────────────────────────────────────────────┐
│                    User Interface (GTK4)                │
│  ┌──────────────────────────────────────────────────┐  │
│  │  Main Window (window.rs)                         │  │
│  │  - Download Tab    - Search Tab                  │  │
│  │  - Progress View   - Result Cards                │  │
│  │  - Queue Manager   - Thumbnail Cache             │  │
│  └──────────────────────────────────────────────────┘  │
└──────────┬──────────────────────────────────────────────┘
           │ GTK Signals & Callbacks
           │ spawn_future_local() for async operations
           ▼
┌─────────────────────────────────────────────────────────┐
│         Core Business Logic (Platform-Agnostic)         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │  Downloader  │  │   Search     │  │    Queue     │  │
│  │              │  │   Service    │  │   Manager    │  │
│  │- validate()  │  │              │  │              │  │
│  │- download()  │  │- search()    │  │- add()       │  │
│  │- platform()  │  │- cache{}     │  │- update()    │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
│        │                  │                  │         │
│        └──────────────────┼──────────────────┘         │
│                           │                            │
│                    Error Handling                       │
│                 DownloadError | SearchError             │
└───────────┬──────────────────────────────────────┬─────┘
            │                                      │
            ▼                                      ▼
    ┌──────────────────────┐        ┌──────────────────────┐
    │ yt-dlp (CLI Tool)    │        │  yt-dlp (CLI Tool)   │
    │ - Download videos    │        │ - Search metadata    │
    │ - Extract metadata   │        │ - Parse JSON output  │
    └──────────────────────┘        └──────────────────────┘
            │                                      │
            ▼                                      ▼
    ┌──────────────────────┐        ┌──────────────────────┐
    │  Video Platforms     │        │  Platform APIs       │
    │ - YouTube            │        │ - YouTube Search     │
    │ - TikTok             │        │ - Dzen Search        │
    │ - Twitter/X          │        │ - Rutube API         │
    │ - Instagram          │        │ - And others...      │
    │ - And more...        │        └──────────────────────┘
    └──────────────────────┘
```

## Layered Architecture

### Layer 1: User Interface (GTK4)

**Location**: `src/ui/`

The UI layer handles all user interaction and display logic using GTK4.
It is platform-independent for video downloading logic but GTK4-specific for
presentation.

**Components**:

- **Main Window** (`window.rs`) - Tab-based interface with Download and Search tabs
- **Search View** (`search_view.rs`) - Reusable search component with async results
- **Download Queue** (`download_queue.rs`) - Visual queue manager with progress tracking
- **Preview Window** (`preview_window.rs`) - Video details and preview before download

**Responsibilities**:

- Render UI components
- Handle user input (button clicks, text entry)
- Display progress and status updates
- Show error messages
- Cache and display thumbnails

**Constraints**:

- Never performs I/O directly
- Never calls yt-dlp or network requests directly
- Delegates all business logic to core layer
- Uses signals and callbacks for GTK integration

### Layer 2: Core Business Logic

**Location**: `src/core/`

The core layer contains all business logic independent of the UI framework.
It can be tested, documented, and reused without GTK dependencies.

**Services**:

- **VideoDownloader** - Video download orchestration
- **SearchService** - Multi-platform video search
- **DownloadQueue** - Queue state management
- **Error Handling** - Comprehensive error types

**Responsibilities**:

- Validate URLs and inputs
- Detect video platforms
- Execute yt-dlp CLI commands
- Parse and structure responses
- Manage download state
- Handle errors and edge cases

**Constraints**:

- No UI dependencies (no GTK4 imports)
- Async-first design using Tokio
- Thread-safe state using `Arc<RwLock<T>>`
- All errors implement Clone trait

### Layer 3: External Tools & APIs

**Location**: System binaries and remote services

**Primary Tool**: `yt-dlp`

- Command-line tool for video extraction and metadata
- Installed separately from the application
- Called via `tokio::process::Command`

**Video Platforms**:

- YouTube, TikTok, Twitter/X, Instagram, Reddit, VK, Rutube, Dzen
- Accessed through yt-dlp (which handles authentication and protocol details)

## Data Flow

### Download Flow

```text
User Input (URL + Directory)
            │
            ▼
┌─────────────────────────────┐
│ URL Validation              │
│ VideoDownloader::validate() │
└─────────────────────────────┘
            │
     ✓ Valid
            │
            ▼
┌─────────────────────────────┐
│ Platform Detection          │
│ Platform::detect_platform() │
└─────────────────────────────┘
            │
            ▼
┌──────────────────────────────────┐
│ Add to Queue                     │
│ DownloadQueue::add(request)      │
└──────────────────────────────────┘
            │
            ▼
┌──────────────────────────────────┐
│ Spawn Download Task              │
│ tokio::spawn { download() }      │
│                                  │
│ ┌──────────────────────────────┐ │
│ │ Execute yt-dlp CLI           │ │
│ │ youtube_dl::YoutubeDl::run() │ │
│ └──────────────────────────────┘ │
│            │                     │
│            ▼                     │
│ ┌──────────────────────────────┐ │
│ │ Save Video File              │ │
│ │ To output_directory          │ │
│ └──────────────────────────────┘ │
└──────────────────────────────────┘
            │
            ▼
┌──────────────────────────────────┐
│ Update Queue Status              │
│ DownloadQueue::update_status()   │
│ - Completed { file_path }        │
│ - Failed { error }               │
└──────────────────────────────────┘
            │
            ▼
    UI Reflects Status
    (Callback invoked)
```

### Search Flow

```text
User Query Input
            │
            ▼
┌─────────────────────────┐
│ Query Validation        │
│ SearchService::search() │
└─────────────────────────┘
            │
     ✓ Valid
            │
            ▼
┌──────────────────────────────────┐
│ Determine Search Type            │
│ - URL: Exact video lookup        │
│ - Keyword: Multi-platform search │
└──────────────────────────────────┘
            │
     ┌──────┴────────────────┐
     │ (URL Case)            │ (Keyword Case)
     │                       │
     ▼                       ▼
┌─────────────┐    ┌─────────────────────────────┐
│ URL Search  │    │ Parallel Platform Search    │
│ - Single    │    │ - YouTube (ytsearch)        │
│   query     │    │ - Dzen (dzen.ru/search)     │
│ - Direct    │    │ - Rutube (API query)        │
│   result    │    │ - All simultaneous          │
└─────────────┘    │   (tokio::spawn)            │
      │            └─────────────────────────────┘
      │                         │
      └─────────────┬───────────┘
                    │
                    ▼
┌──────────────────────────────────┐
│ Parse yt-dlp Output (JSON)       │
│ - Extract video metadata         │
│ - Optional: thumbnail URL        │
│ - Platform detection             │
└──────────────────────────────────┘
                    │
                    ▼
┌──────────────────────────────────┐
│ Load Thumbnails Asynchronously   │
│ - Cache URLs to Pixbuf objects   │
│ - Scale to 120x90px              │
│ - Fallback placeholder on error  │
└──────────────────────────────────┘
                    │
                    ▼
┌──────────────────────────────────┐
│ Display Search Results           │
│ - Result cards with thumbnails   │
│ - Metadata (title, uploader)     │
│ - Download button per result     │
└──────────────────────────────────┘
```

## Module Dependencies

### Dependency Graph

```text
main.rs
  │
  ├─→ ui/window.rs
  │     ├─→ ui/components/search_view.rs
  │     ├─→ ui/components/download_queue.rs
  │     ├─→ ui/components/preview_window.rs
  │     ├─→ core/downloader.rs
  │     ├─→ core/search.rs
  │     └─→ core/queue.rs
  │
  ├─→ core/downloader.rs
  │     ├─→ core/error.rs
  │     └─→ youtube_dl (external)
  │
  ├─→ core/search.rs
  │     ├─→ core/downloader.rs (for platform detection)
  │     ├─→ core/error.rs
  │     ├─→ tokio (async runtime)
  │     └─→ youtube_dl (external)
  │
  └─→ core/queue.rs
        └─→ core/downloader.rs (DownloadRequest)
```

### Key Invariants

- **UI layer depends on Core layer** - UI imports from core
- **Core layer is independent** - No UI imports in core
- **Error types are in core** - UI imports error types
- **All I/O is async** - Using Tokio runtime

## Request-Response Pattern

### Download Request Processing

```text
HTTP/User Request
        │
        ▼
┌──────────────────────┐
│ DownloadRequest {    │
│   url: String,       │
│   platform: Platform,│
│   output_path: Opt,  │
│   overwrite: bool    │
│ }                    │
└──────────────────────┘
        │
        ▼
┌──────────────────────────────────┐
│ VideoDownloader::download()      │
│ - Returns Result<String, Error>  │
│ - String is file path on success │
└──────────────────────────────────┘
```

### Search Request Processing

```text
SearchRequest { query, limit }
        │
        ▼
┌──────────────────────────────────┐
│ SearchService::search()          │
│ - Returns Result<Vec<Result>, E> │
└──────────────────────────────────┘
        │
        ▼
┌──────────────────────────────────┐
│ SearchResult {                   │
│   id: String,                    │
│   title: String,                 │
│   url: String,                   │
│   thumbnail: Option<Url>,        │
│   duration: Option<u64>,         │
│   uploader: Option<String>,      │
│   view_count: Option<u64>,       │
│   platform: Platform             │
│ }                                │
└──────────────────────────────────┘
```

## Error Handling Strategy

### Error Categories

**DownloadError** (in `core/error.rs`):

- `InvalidUrl` - URL format validation failed
- `DownloadFailed` - Download operation failed
- `UnsupportedPlatform` - Platform not supported
- `IoError` - File system error
- `ExtractionError` - Metadata extraction failed
- `NetworkError` - Network request failed
- `VideoNotFound` - Video unavailable
- `InvalidOutputDirectory` - Directory invalid/inaccessible
- `Cancelled` - User cancelled

**SearchError** (in `core/search.rs`):

- `InvalidQuery` - Empty or invalid search query
- `CommandFailed` - yt-dlp execution failed
- `JsonParseError` - JSON parsing failed
- `MissingYtDlp` - yt-dlp not found in PATH
- `RateLimited` - HTTP 429 rate limit
- `IoError` - I/O operation failed

### Error Flow

```text
yt-dlp Command Execution
            │
    ┌───────┴────────┐
    │ (Exit Code 0)  │ (Exit Code != 0)
    │                │
    ▼                ▼
  Success    Inspect stderr
    │              │
    │         ┌────┴────────────────┐
    │         │                     │
    │         ▼                     ▼
    │    "HTTP Error 429"    Other errors
    │         │                    │
    │         ▼                    ▼
    │   RateLimited error   CommandFailed error
    │         │                    │
    └─────────┴────────────────────┘
              │
              ▼
        Return Result<T, E>
```

## Async Architecture

### Tokio Runtime Setup

Located in `main.rs`:

```rust
let runtime = tokio::runtime::Runtime::new()?;
let _guard = runtime.enter();
```

### Async Operation Patterns

**In Core Layer** (Services):

```rust
pub async fn download(&self, request: DownloadRequest) -> Result<String> {
    // Async operation
    let output = Command::new("yt-dlp").output().await?;
    Ok(output)
}
```

**In UI Layer** (Callbacks):

```rust
button.connect_clicked(|_| {
    gtk4::glib::spawn_future_local(async {
        let result = search_service.search("query", None).await;
        // Update UI with result
    });
});
```

### Concurrent Operations

**Download Queue**: Multiple downloads run concurrently in separate Tokio tasks
**Search**: Platform searches run concurrently (tokio::spawn for each platform)
**Thumbnail Loading**: Thumbnails load concurrently via reqwest

## State Management

### Core Layer State

**VideoDownloader**:

- Immutable: `output_directory: String`
- Stateless after initialization

**SearchService**:

- Immutable: `default_limit: u32`
- Stateless, implements Copy trait

**DownloadQueue**:

- Mutable shared state: `Arc<RwLock<HashMap<String, QueueItem>>>`
- Thread-safe for concurrent updates

### UI Layer State

**Main Window**:

- GTK widgets (UI state)
- References to core services
- Callbacks and event handlers

**SearchView Component**:

- `Rc<RefCell<HashMap<String, Pixbuf>>>` - Thumbnail cache
- `Rc<RefCell<Option<Box<dyn Fn(SearchResult)>>>>` - Download callback
- Widget references for updates

## Platform Detection Strategy

### URL-Based Detection (Primary)

Located in `VideoDownloader::detect_platform()`:

```rust
if url.contains("youtube.com") || url.contains("youtu.be") {
    Platform::YouTube
} else if url.contains("tiktok.com") {
    Platform::TikTok
}
// ... more platforms
```

### Extractor-Based Detection (Search Results)

Located in `search.rs` via yt-dlp's `extractor` field:

```rust
fn platform_from_hint(extractor: &str) -> Platform {
    match extractor {
        "youtube" => Platform::YouTube,
        "tiktok" => Platform::TikTok,
        // ... more platforms
    }
}
```

### Fallback Chain

1. yt-dlp `extractor` field (if from search)
1. yt-dlp `extractor_key` field (if from search)
1. URL pattern detection via `detect_platform()`
1. Default to `Platform::Other`

## Integration Points

### Extending with New Platforms

To add support for a new video platform:

1. **Update Platform enum** (downloader.rs):

   ```rust
   pub enum Platform {
       // ... existing
       NewPlatform,
   }
   ```

1. **Update URL detection** (downloader.rs):

   ```rust
   pub fn detect_platform(url: &str) -> Platform {
       // ... existing
       else if url.contains("newplatform.com") {
           Platform::NewPlatform
       }
   }
   ```

1. **Update search detection** (search.rs):

   ```rust
   fn platform_from_hint(extractor: &str) -> Platform {
       match extractor {
           // ... existing
           "newplatform" => Platform::NewPlatform,
       }
   }
   ```

1. **Test**: yt-dlp automatically handles the new platform

### Extending with New Search Features

To add search for a new platform in `SearchService::search()`:

```rust
let new_platform_results = tokio::spawn(async {
    Self::execute_search_command(
        &search_url,
        &["--dump-json", "--flat-playlist", "--skip-download"],
        Some(limit),
    ).await
});

tasks.push(new_platform_results);
```

## Performance Considerations

### Memory

- **Thumbnail cache** - Limited by SearchResult count (typically < 100)
- **Queue state** - HashMap with item count as upper bound
- **Release build** - Optimized for size with LTO

### CPU

- **yt-dlp spawning** - Minimal (single system call)
- **JSON parsing** - Streaming parser (doesn't load full output in memory)
- **UI updates** - Batched via GTK signal handlers

### Network

- **Concurrent requests** - Up to 3 platform searches simultaneously
- **Thumbnail loading** - Non-blocking via reqwest
- **Caching** - Per-URL to avoid duplicate requests

## Security Considerations

### URL Validation

- Whitelist protocol check (`http://` or `https://`)
- Domain validation before execution
- User-selected output directory

### yt-dlp Integration

- Spawned as separate process
- No shell execution (direct binary call)
- Output validation before display

### Error Handling

- Never expose system paths in user messages
- Sanitize error messages from external tools
- Log detailed errors internally

## Testing Strategy

### Unit Tests (in core layer)

- Platform detection
- URL validation
- Error handling
- Queue operations
- Search result parsing

### Integration Tests (would be added)

- End-to-end download flow
- Multi-platform search aggregation
- Error recovery scenarios

### Manual Testing

- UI responsiveness during downloads
- Thumbnail loading and caching
- Error message clarity
- Platform-specific edge cases

## Documentation Structure

```text
docs/
├── CODEBASE_INDEX.md     ← Module catalog and structure
├── ARCHITECTURE.md        ← This file - System design
├── SETUP.md              ← Development environment
└── DEVELOPMENT_GUIDE.md  ← How to extend the code
```
