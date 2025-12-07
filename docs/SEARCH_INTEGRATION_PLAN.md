# Search Service Integration Plan

## 1. Core Layer (`src/core/search.rs`)

We will introduce a new module `src/core/search.rs` to handle video searching functionality.

### Data Structures

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub url: String,
    pub thumbnail: Option<String>,
    pub duration: Option<f64>, // in seconds
    pub uploader: Option<String>,
    pub view_count: Option<u64>,
    pub platform: Platform,
}
```

### Service Interface

```rust
pub struct SearchService;

impl SearchService {
    /// Searches for videos using yt-dlp's search functionality.
    /// 
    /// # Arguments
    /// * `query` - The search query string
    /// * `limit` - Maximum number of results to return (default: 10)
    /// * `platform` - Optional platform filter (e.g., "ytsearch", "tiktoksearch")
    pub async fn search(query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        // Implementation details:
        // 1. Construct yt-dlp command: `yt-dlp "ytsearch<limit>:<query>" --dump-json --flat-playlist --skip-download`
        // 2. Execute command asynchronously
        // 3. Parse line-delimited JSON output
        // 4. Map JSON to SearchResult structs
    }
}
```

### Dependencies

- `serde` & `serde_json` for parsing `yt-dlp` output.
- `tokio::process::Command` for async execution.

## 2. UI Layer (`src/ui/components/search_view.rs`)

We will create a new UI component to present the search interface.

### Components

- **Search Bar**: `gtk::SearchEntry` for input.
- **Results View**: `gtk::ListView` or `gtk::FlowBox` to display results.
- **Result Item**: A custom widget (composite of `Image`, `Label`s, and `Button`) to display video metadata.

### Interaction Flow

1. User types query and presses Enter.
2. UI spawns an async task calling `SearchService::search`.
3. Loading spinner (`gtk::Spinner`) is shown.
4. On success, results are populated in the list.
5. On failure, an error message is displayed.
6. **Selection**: Clicking a result or a "Download" button on a result will:
   - Switch the main view to the "Download" tab (if we implement tabs).
   - OR populate the URL entry in the existing download form.
   - OR immediately trigger a download confirmation dialog.

## 3. Integration Steps

1. **Core Implementation**:
    - Create `src/core/search.rs`.
    - Implement `SearchService::search` using `yt-dlp`.
    - Add unit tests for parsing `yt-dlp` JSON output.

2. **UI Implementation**:
    - Create `src/ui/components/search_view.rs`.
    - Design the row item layout (thumbnail left, title/metadata right).
    - Implement the search action and state management.

3. **Main Window Integration**:
    - Modify `src/ui/window.rs` to include a `gtk::Stack` or `gtk::Notebook`.
    - Add "Download" and "Search" pages to the stack.
    - Add navigation controls (e.g., a `gtk::StackSwitcher` in the header bar).
    - Wire up the "Download" action from the search view to the main downloader logic.

## 4. Technical Considerations

- **Thumbnails**: `yt-dlp` returns thumbnail URLs. We will need an async image loader (e.g., using `gdk_pixbuf` and `reqwest` or `soup`) to display them in the UI. For the MVP, we might skip thumbnails or use a placeholder.
- **Performance**: Search operations are network-bound. Ensure the UI remains responsive using `tokio::spawn` or `glib::spawn_future_local`.
- **Rate Limiting**: Frequent searches might trigger rate limits from providers (YouTube). We should handle 429 errors gracefully.
