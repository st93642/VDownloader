# VDownloader API Reference

Complete reference for all public APIs and modules in VDownloader.

Note: VDownloader is a desktop application, not a web service. This document describes the public Rust APIs that can be used by other code or extensions.

## Table of Contents

1. [Core Module (core/)](#core-module)
2. [UI Module (ui/)](#ui-module)
3. [Error Types](#error-types)
4. [Types and Enums](#types-and-enums)
5. [Common Usage Examples](#common-usage-examples)

## Core Module

The core module contains all business logic independent of the UI framework.

### Module: `core::downloader`

Video download engine with platform detection.

#### Enum: `Platform`

Represents supported video platforms for downloading.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Platform {
    YouTube,
    TikTok,
    Twitter,
    Instagram,
    Reddit,
    Vk,
    Rutube,
    Dzen,
    Other,
}
```

**Variants**:
- `YouTube` - YouTube and YouTube Shorts
- `TikTok` - TikTok videos and compilations
- `Twitter` - Twitter/X posts and threads
- `Instagram` - Instagram photos, videos, and reels
- `Reddit` - Reddit videos and GIFs
- `Vk` - VK and VKVideo platform videos
- `Rutube` - Russian video platform
- `Dzen` - Yandex Dzen videos
- `Other` - Any other platform (yt-dlp dependent)

**Implementations**:
- `PartialEq` - Compare platforms
- `Eq` - Equality
- `Clone` - Clone enum
- `Debug` - Debug formatting

#### Struct: `DownloadRequest`

Request parameters for a video download.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRequest {
    pub url: String,                    // Video URL
    pub platform: Platform,             // Detected platform
    pub output_path: Option<String>,    // Output directory
    pub overwrite: bool,                // Overwrite existing files
}
```

**Fields**:
- `url: String` - Video URL (must start with http:// or https://)
- `platform: Platform` - Platform (auto-detected via `Platform::detect_platform()`)
- `output_path: Option<String>` - Output directory path (defaults to current directory)
- `overwrite: bool` - Whether to overwrite existing files (default: false)

**Example**:
```rust
let request = DownloadRequest {
    url: "https://youtube.com/watch?v=dQw4w9WgXcQ".to_string(),
    platform: Platform::YouTube,
    output_path: Some("/home/user/Videos".to_string()),
    overwrite: false,
};
```

#### Enum: `DownloadStatus`

Current status of a download in the queue.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DownloadStatus {
    Pending,
    Downloading { progress: f32 },           // 0.0 to 1.0
    Completed { file_path: String },
    Failed { error: String },
}
```

**Variants**:
- `Pending` - Queued but not yet downloading
- `Downloading { progress: f32 }` - In progress with 0.0-1.0 progress
- `Completed { file_path: String }` - Successfully completed with output path
- `Failed { error: String }` - Failed with error message

#### Struct: `VideoDownloader`

Main download service.

```rust
pub struct VideoDownloader {
    output_directory: String,
}

impl VideoDownloader {
    pub fn new(output_directory: String) -> Self;
    
    pub fn detect_platform(url: &str) -> Platform;
    
    pub fn validate_url(url: &str) -> Result<()>;
    
    pub async fn download(
        &self,
        request: DownloadRequest,
    ) -> Result<String>;
}
```

**Methods**:

##### `new(output_directory: String) -> Self`

Create a new downloader instance.

**Parameters**:
- `output_directory: String` - Default output directory for downloads

**Returns**: `VideoDownloader` instance

**Example**:
```rust
let downloader = VideoDownloader::new("/home/user/Videos".to_string());
```

##### `detect_platform(url: &str) -> Platform`

Detect video platform from URL.

**Parameters**:
- `url: &str` - Video URL

**Returns**: `Platform` enum

**Example**:
```rust
let platform = VideoDownloader::detect_platform("https://youtube.com/watch?v=123");
assert_eq!(platform, Platform::YouTube);
```

##### `validate_url(url: &str) -> Result<()>`

Validate URL format.

**Parameters**:
- `url: &str` - URL to validate

**Returns**: `Result<(), DownloadError>`
- `Ok(())` if valid
- `Err(DownloadError::InvalidUrl(...))` if invalid

**Validation rules**:
- URL cannot be empty
- URL must start with `http://` or `https://`
- URL must be properly formatted

**Example**:
```rust
if let Err(e) = VideoDownloader::validate_url("not a url") {
    println!("Invalid: {}", e);  // Invalid URL: URL must start with...
}
```

##### `async fn download(&self, request: DownloadRequest) -> Result<String>`

Execute video download.

**Parameters**:
- `request: DownloadRequest` - Download request with URL and options

**Returns**: `Result<String, DownloadError>`
- `Ok(String)` - File path of downloaded video
- `Err(DownloadError)` - Error during download

**Error cases**:
- `InvalidUrl` - URL validation failed
- `InvalidOutputDirectory` - Directory doesn't exist or not writable
- `DownloadFailed` - Download operation failed
- `VideoNotFound` - Video unavailable
- `ExtractionError` - Metadata extraction failed
- `NetworkError` - Network request failed
- `UnsupportedPlatform` - Platform not supported by yt-dlp
- `IoError` - File I/O error
- `Cancelled` - User cancelled download

**Example**:
```rust
let downloader = VideoDownloader::new("/home/user/Videos".to_string());
let request = DownloadRequest {
    url: "https://youtube.com/watch?v=dQw4w9WgXcQ".to_string(),
    platform: Platform::YouTube,
    output_path: Some("/home/user/Downloads".to_string()),
    overwrite: true,
};

match downloader.download(request).await {
    Ok(file_path) => println!("Downloaded to: {}", file_path),
    Err(e) => eprintln!("Download failed: {}", e),
}
```

### Module: `core::search`

Video search service with multi-platform support.

#### Enum: `SearchError`

Errors specific to search operations.

```rust
#[derive(Error, Debug, Clone)]
pub enum SearchError {
    #[error("Invalid query: {0}")]
    InvalidQuery(String),

    #[error("Command failed: {0}")]
    CommandFailed(String),

    #[error("JSON parse error: {0}")]
    JsonParseError(String),

    #[error("yt-dlp not found or not installed")]
    MissingYtDlp,

    #[error("Rate limit exceeded (HTTP 429): {0}")]
    RateLimited(String),

    #[error("IO error: {0}")]
    IoError(String),
}
```

**Variants**:
- `InvalidQuery(String)` - Empty or invalid search query
- `CommandFailed(String)` - yt-dlp execution failed
- `JsonParseError(String)` - JSON response parsing failed
- `MissingYtDlp` - yt-dlp binary not found in PATH
- `RateLimited(String)` - HTTP 429 rate limit exceeded
- `IoError(String)` - File I/O error

#### Struct: `SearchResult`

Structured search result with metadata.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchResult {
    pub id: String,                    // Unique video ID
    pub title: String,                 // Video title
    pub url: String,                   // Video URL
    pub thumbnail: Option<String>,     // Thumbnail URL
    pub duration: Option<u64>,         // Duration in seconds
    pub uploader: Option<String>,      // Uploader/channel name
    pub view_count: Option<u64>,       // View count
    pub platform: Platform,            // Detected platform
}
```

**Fields**:
- `id: String` - Unique identifier from platform
- `title: String` - Video title/name
- `url: String` - Direct link to video
- `thumbnail: Option<String>` - Thumbnail image URL (may be None)
- `duration: Option<u64>` - Video duration in seconds (may be None)
- `uploader: Option<String>` - Creator/channel name (may be None)
- `view_count: Option<u64>` - View count (may be None)
- `platform: Platform` - Detected platform enum

**Example**:
```rust
SearchResult {
    id: "dQw4w9WgXcQ".to_string(),
    title: "Rick Astley - Never Gonna Give You Up".to_string(),
    url: "https://youtube.com/watch?v=dQw4w9WgXcQ".to_string(),
    thumbnail: Some("https://i.ytimg.com/...".to_string()),
    duration: Some(212),
    uploader: Some("Rick Astley".to_string()),
    view_count: Some(1234567890),
    platform: Platform::YouTube,
}
```

#### Struct: `SearchService`

Video search orchestrator.

```rust
#[derive(Debug, Clone, Copy)]
pub struct SearchService {
    default_limit: u32,
}

impl SearchService {
    pub fn new(default_limit: u32) -> Self;
    
    pub fn default_limit(&self) -> u32;
    
    pub async fn search(
        &self,
        query: &str,
        limit: Option<u32>,
    ) -> Result<Vec<SearchResult>, SearchError>;
}
```

**Methods**:

##### `new(default_limit: u32) -> Self`

Create a new search service.

**Parameters**:
- `default_limit: u32` - Default result limit per platform (minimum 1)

**Returns**: `SearchService` instance

**Example**:
```rust
let search = SearchService::new(10);  // 10 results per platform
```

##### `default_limit(&self) -> u32`

Get the default result limit.

**Returns**: `u32` - Default limit value

**Example**:
```rust
let search = SearchService::new(10);
assert_eq!(search.default_limit(), 10);
```

##### `async fn search(&self, query: &str, limit: Option<u32>) -> Result<Vec<SearchResult>, SearchError>`

Search for videos.

**Parameters**:
- `query: &str` - Search query (keyword or URL)
- `limit: Option<u32>` - Result limit (None uses default)

**Returns**: `Result<Vec<SearchResult>, SearchError>`
- `Ok(Vec<SearchResult>)` - Search results from all platforms
- `Err(SearchError)` - Error during search

**Search modes**:
- **Keyword search**: Returns aggregated results from YouTube, Dzen, Rutube
- **URL search**: Returns single video with metadata

**Error cases**:
- `InvalidQuery` - Query is empty or whitespace
- `CommandFailed` - yt-dlp execution failed
- `JsonParseError` - JSON parsing failed
- `MissingYtDlp` - yt-dlp not installed
- `RateLimited` - HTTP 429 rate limit
- `IoError` - File I/O error

**Example - Keyword Search**:
```rust
let search = SearchService::new(10);
match search.search("rust programming", Some(5)).await {
    Ok(results) => {
        for result in results {
            println!("{}: {}", result.title, result.url);
        }
    }
    Err(e) => eprintln!("Search failed: {}", e),
}
```

**Example - URL Search**:
```rust
let search = SearchService::new(10);
match search.search("https://youtube.com/watch?v=dQw4w9WgXcQ", None).await {
    Ok(results) => {
        if let Some(video) = results.first() {
            println!("Found: {}", video.title);
        }
    }
    Err(e) => eprintln!("Search failed: {}", e),
}
```

### Module: `core::queue`

Download queue management.

#### Struct: `QueueItem`

Individual item in download queue.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueItem {
    pub id: String,
    pub request: DownloadRequest,
    pub status: DownloadStatus,
}
```

**Fields**:
- `id: String` - Unique queue item ID
- `request: DownloadRequest` - Original download request
- `status: DownloadStatus` - Current status

#### Struct: `DownloadQueue`

Thread-safe queue for managing downloads.

```rust
pub struct DownloadQueue {
    // Fields are private
}

impl DownloadQueue {
    pub fn new() -> Self;
    
    pub async fn add(&self, request: DownloadRequest) -> String;
    
    pub async fn get(&self, id: &str) -> Option<QueueItem>;
    
    pub async fn update_status(&self, id: &str, status: DownloadStatus);
    
    pub async fn remove(&self, id: &str);
    
    pub async fn list_all(&self) -> Vec<QueueItem>;
    
    pub async fn clear(&self);
}

impl Default for DownloadQueue {
    fn default() -> Self;
}
```

**Methods**:

##### `new() -> Self`

Create a new download queue.

**Returns**: `DownloadQueue` instance

**Example**:
```rust
let queue = DownloadQueue::new();
```

##### `async fn add(&self, request: DownloadRequest) -> String`

Add item to queue.

**Parameters**:
- `request: DownloadRequest` - Download request

**Returns**: `String` - Unique item ID

**Example**:
```rust
let queue = DownloadQueue::new();
let request = DownloadRequest {
    url: "https://youtube.com/watch?v=123".to_string(),
    platform: Platform::YouTube,
    output_path: None,
    overwrite: false,
};
let item_id = queue.add(request).await;
println!("Added to queue: {}", item_id);  // download_0
```

##### `async fn get(&self, id: &str) -> Option<QueueItem>`

Get queue item by ID.

**Parameters**:
- `id: &str` - Item ID

**Returns**: `Option<QueueItem>`
- `Some(QueueItem)` if found
- `None` if not found

**Example**:
```rust
if let Some(item) = queue.get("download_0").await {
    println!("Status: {:?}", item.status);
}
```

##### `async fn update_status(&self, id: &str, status: DownloadStatus)`

Update item status.

**Parameters**:
- `id: &str` - Item ID
- `status: DownloadStatus` - New status

**Example**:
```rust
queue.update_status("download_0", DownloadStatus::Downloading {
    progress: 0.5,
}).await;
```

##### `async fn remove(&self, id: &str)`

Remove item from queue.

**Parameters**:
- `id: &str` - Item ID

**Example**:
```rust
queue.remove("download_0").await;
```

##### `async fn list_all(&self) -> Vec<QueueItem>`

Get all items in queue.

**Returns**: `Vec<QueueItem>` - All items

**Example**:
```rust
let items = queue.list_all().await;
for item in items {
    println!("{}: {:?}", item.id, item.status);
}
```

##### `async fn clear(&self)`

Clear all items from queue.

**Example**:
```rust
queue.clear().await;
```

### Module: `core::error`

Error types and result aliases.

```rust
#[derive(Error, Debug, Clone)]
pub enum DownloadError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Download failed: {0}")]
    DownloadFailed(String),

    #[error("Unsupported platform: {0}")]
    UnsupportedPlatform(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Video extraction error: {0}")]
    ExtractionError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Video not found or unavailable")]
    VideoNotFound,

    #[error("Output directory does not exist or is not writable")]
    InvalidOutputDirectory,

    #[error("Cancelled by user")]
    Cancelled,
}

pub type Result<T> = std::result::Result<T, DownloadError>;
```

**Type Alias**:
- `Result<T>` = `std::result::Result<T, DownloadError>`

## UI Module

The UI module contains GTK4-specific components.

### Module: `ui::window`

Main application window.

```rust
pub fn build_window(app: &Application) -> ApplicationWindow;
```

**Function**:

##### `build_window(app: &Application) -> ApplicationWindow`

Build and return the main application window.

**Parameters**:
- `app: &Application` - GTK Application instance

**Returns**: `ApplicationWindow` - Main window with all UI components

### Module: `ui::components::search_view`

Search UI component.

```rust
pub struct SearchView {
    // Fields are private
}

impl SearchView {
    pub fn new() -> Self;
    
    pub fn container(&self) -> &gtk4::Box;
    
    pub fn set_download_callback<F>(&mut self, callback: F)
    where
        F: Fn(SearchResult) + 'static,
}
```

**Methods**:

##### `new() -> Self`

Create a new search view component.

**Returns**: `SearchView` instance

##### `container(&self) -> &gtk4::Box`

Get the container widget for adding to UI.

**Returns**: Reference to GTK Box widget

##### `set_download_callback<F>(&mut self, callback: F)`

Register callback for download button clicks.

**Parameters**:
- `callback: F` - Closure called when user clicks Download on a result
  - Takes `SearchResult` as parameter
  - Called from GTK signal context

**Example**:
```rust
let mut search_view = SearchView::new();
search_view.set_download_callback(|result: SearchResult| {
    println!("Download: {}", result.url);
});
```

### Module: `ui::components::download_queue`

Download queue UI widget.

```rust
pub struct DownloadQueueWidget {
    // Fields are private
}

impl DownloadQueueWidget {
    pub fn new() -> Self;
    
    pub fn widget(&self) -> &gtk4::Box;
    
    pub fn update_queue(&self, items: Vec<QueueItem>);
}
```

## Error Types

### DownloadError

Used by downloader module.

```rust
pub enum DownloadError {
    InvalidUrl(String),
    DownloadFailed(String),
    UnsupportedPlatform(String),
    IoError(String),
    ExtractionError(String),
    NetworkError(String),
    VideoNotFound,
    InvalidOutputDirectory,
    Cancelled,
}
```

### SearchError

Used by search module.

```rust
pub enum SearchError {
    InvalidQuery(String),
    CommandFailed(String),
    JsonParseError(String),
    MissingYtDlp,
    RateLimited(String),
    IoError(String),
}
```

## Types and Enums

### Platform

Supported video platforms.

**Variants**:
- `YouTube`
- `TikTok`
- `Twitter`
- `Instagram`
- `Reddit`
- `Vk`
- `Rutube`
- `Dzen`
- `Other`

### DownloadStatus

Download progress status.

**Variants**:
- `Pending` - Queued but not started
- `Downloading { progress: f32 }` - In progress (0.0 to 1.0)
- `Completed { file_path: String }` - Finished successfully
- `Failed { error: String }` - Failed with error message

## Common Usage Examples

### Example 1: Download a Video

```rust
use vdownloader::core::downloader::{VideoDownloader, DownloadRequest, Platform};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let downloader = VideoDownloader::new("/home/user/Videos".to_string());
    
    // Validate URL first
    let url = "https://youtube.com/watch?v=dQw4w9WgXcQ";
    VideoDownloader::validate_url(url)?;
    
    // Create request
    let request = DownloadRequest {
        url: url.to_string(),
        platform: VideoDownloader::detect_platform(url),
        output_path: Some("/home/user/Downloads".to_string()),
        overwrite: false,
    };
    
    // Download
    match downloader.download(request).await {
        Ok(file_path) => println!("Downloaded to: {}", file_path),
        Err(e) => eprintln!("Download failed: {}", e),
    }
    
    Ok(())
}
```

### Example 2: Search for Videos

```rust
use vdownloader::core::search::SearchService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let search = SearchService::new(10);
    
    match search.search("rust programming", Some(5)).await {
        Ok(results) => {
            for result in results {
                println!("Title: {}", result.title);
                println!("URL: {}", result.url);
                println!("Platform: {:?}", result.platform);
                if let Some(duration) = result.duration {
                    println!("Duration: {}s", duration);
                }
                println!();
            }
        }
        Err(e) => eprintln!("Search failed: {}", e),
    }
    
    Ok(())
}
```

### Example 3: Manage Download Queue

```rust
use vdownloader::core::queue::DownloadQueue;
use vdownloader::core::downloader::{DownloadRequest, Platform, DownloadStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let queue = DownloadQueue::new();
    
    // Add items
    let request1 = DownloadRequest {
        url: "https://youtube.com/watch?v=123".to_string(),
        platform: Platform::YouTube,
        output_path: None,
        overwrite: false,
    };
    let id1 = queue.add(request1).await;
    
    let request2 = DownloadRequest {
        url: "https://tiktok.com/video/456".to_string(),
        platform: Platform::TikTok,
        output_path: None,
        overwrite: false,
    };
    let id2 = queue.add(request2).await;
    
    // List items
    let items = queue.list_all().await;
    println!("Queue has {} items", items.len());
    
    // Update status
    queue.update_status(&id1, DownloadStatus::Downloading {
        progress: 0.5,
    }).await;
    
    // Get item
    if let Some(item) = queue.get(&id1).await {
        println!("Item {}: {:?}", item.id, item.status);
    }
    
    // Remove item
    queue.remove(&id2).await;
    
    Ok(())
}
```

## See Also

- [CODEBASE_INDEX.md](CODEBASE_INDEX.md) - Module locations and organization
- [ARCHITECTURE.md](ARCHITECTURE.md) - System design and patterns
- [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md) - How to extend the code
