# SearchView Component

## Overview

The `SearchView` is a reusable GTK4 component that provides a complete video search interface with async thumbnail loading. It integrates with the `SearchService` to query video platforms via yt-dlp and displays results in an elegant, user-friendly interface.

## Features

- ✅ **Non-blocking Search**: Uses `gtk4::glib::spawn_future_local()` for async operations
- ✅ **Loading States**: Visual feedback with spinner and status messages
- ✅ **Async Thumbnail Loading**: Fetches and caches thumbnails asynchronously
- ✅ **Error Handling**: Graceful handling of network errors, rate limits, and parse failures
- ✅ **Result Cards**: Rich display with thumbnail, title, metadata, and action buttons
- ✅ **Callback API**: Exposes download actions via callback setter

## Usage

### Basic Integration

```rust
use crate::ui::components::search_view::SearchView;

// Create the search view
let search_view = SearchView::new();

// Set up callback for download actions
search_view.set_download_callback(|result| {
    println!("User wants to download: {}", result.title);
    println!("URL: {}", result.url);
    // Trigger download or populate URL field
});

// Add to your UI
main_layout.append(&search_view.container);
```

### Integration Example (Main Window)

```rust
pub fn build_window(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("VDownloader")
        .default_width(800)
        .default_height(600)
        .build();

    let main_box = gtk4::Box::new(Orientation::Vertical, 12);
    
    // Create search view
    let search_view = SearchView::new();
    
    // Set up download callback
    let url_entry = Entry::new(); // Your existing URL entry field
    let url_entry_clone = url_entry.clone();
    search_view.set_download_callback(move |result| {
        // Populate the URL field with selected result
        url_entry_clone.set_text(&result.url);
    });
    
    // Add to layout
    main_box.append(&search_view.container);
    main_box.append(&url_entry);
    // ... rest of your UI
    
    window.set_child(Some(&main_box));
    window
}
```

## Component Structure

### Widget Hierarchy

```
gtk4::Box (container - vertical)
  ├─ gtk4::Box (search_box - horizontal)
  │   ├─ SearchEntry - User input for queries
  │   └─ Button - "Search" trigger
  ├─ Spinner - Loading indicator
  ├─ Label - Status/error messages
  └─ ScrolledWindow
      └─ ListBox - Search results
          └─ [Result cards...]
```

### Result Card Layout

```
gtk4::Box (horizontal)
  ├─ Image - Thumbnail (120x90px, placeholder fallback)
  └─ gtk4::Box (vertical)
      ├─ Label - Title (heading)
      ├─ Label - Uploader (dim-label)
      ├─ Label - Duration & Views (dim-label, caption)
      ├─ Label - Platform (dim-label, caption)
      └─ Button - "Download" (suggested-action)
```

## API Reference

### Constructor

```rust
pub fn new() -> Self
```

Creates a new `SearchView` instance with default configuration (10 results per search).

### Methods

```rust
pub fn set_download_callback<F>(&self, callback: F)
where
    F: Fn(SearchResult) + 'static
```

Registers a callback that will be invoked when the user clicks "Download" on a search result.

**Parameters:**
- `callback`: Closure that receives a `SearchResult` when triggered

**Example:**
```rust
search_view.set_download_callback(|result| {
    println!("Download: {} from {}", result.title, result.url);
});
```

### Public Fields

```rust
pub container: gtk4::Box
```

The main container widget. Append this to your layout to display the search view.

## Behavior

### Search Flow

1. User enters query in `SearchEntry`
2. User presses Enter or clicks "Search" button
3. Component validates query (non-empty)
4. Controls are disabled, spinner starts
5. `SearchService::search()` is called asynchronously
6. Results are parsed and displayed
7. Controls are re-enabled, spinner stops

### Thumbnail Loading

1. Result cards are created with placeholder images
2. Async requests fetch thumbnails from URLs
3. Downloaded images are decoded with `PixbufLoader`
4. Thumbnails are scaled to 120x90 pixels
5. Images are cached per URL to avoid redundant requests
6. Errors are logged but don't block result display

### Error Handling

The component handles several error scenarios:

- **Invalid Query**: Empty or whitespace-only input
- **Missing yt-dlp**: Binary not found in PATH
- **Rate Limiting**: HTTP 429 from video platforms
- **Network Errors**: Connection failures, timeouts
- **Parse Errors**: Malformed JSON from yt-dlp

All errors are displayed in the status label with appropriate CSS classes.

## CSS Classes

The component uses the following CSS classes:

- `suggested-action` - Search button
- `boxed-list` - Results list container
- `heading` - Result title
- `dim-label` - Secondary text (uploader, metadata)
- `caption` - Small text (metadata)
- `error` - Error messages
- `warning` - Warning messages (no results)

## Metadata Formatting

### Duration

- Less than 1 hour: `M:SS` (e.g., "3:42")
- 1 hour or more: `H:MM:SS` (e.g., "1:23:45")

### View Count

- Less than 1K: Raw number (e.g., "42 views")
- 1K to 1M: Thousands with 1 decimal (e.g., "15.3K views")
- 1M or more: Millions with 1 decimal (e.g., "2.5M views")

## Performance Considerations

### Caching

Thumbnails are cached in memory per URL. The cache persists for the lifetime of the `SearchView` instance. This prevents redundant network requests when:

- The same video appears in multiple searches
- User triggers the same search multiple times
- Different videos share the same thumbnail URL

### Async Operations

All network operations are asynchronous:

- Search queries via `SearchService`
- Thumbnail downloads via `reqwest`
- Image decoding via `PixbufLoader`

This ensures the UI remains responsive during:
- Slow network conditions
- Large result sets
- Multiple concurrent thumbnail downloads

## Testing

The component includes unit tests for:

- Duration formatting
- View count formatting
- Error message formatting

Run tests with:
```bash
cargo test ui::components::search_view::tests
```

## Dependencies

### Required Crates

- `gtk4` (0.9 with v4_10 features) - UI framework
- `glib` (0.20) - GTK foundation
- `gdk-pixbuf` (0.20) - Image loading/scaling
- `reqwest` (0.11 with rustls-tls) - HTTP client
- `tokio` (1.35 with full features) - Async runtime

### Runtime Dependencies

- `yt-dlp` - Must be installed and available in PATH
  - Ubuntu/Debian: `sudo apt install yt-dlp`
  - macOS: `brew install yt-dlp`
  - Windows: `pip install yt-dlp`

## Limitations

1. **No Result Limit Control**: Currently fixed at 10 results per search
2. **No Platform Filter**: Searches all platforms (YouTube by default)
3. **No Pagination**: All results loaded at once
4. **No Preview**: No video preview/playback before download
5. **Memory Cache Only**: Thumbnails not persisted to disk

## Future Enhancements

### Planned Features

- [ ] Configurable result limit
- [ ] Platform-specific searches (YouTube, TikTok, etc.)
- [ ] Search history with autocomplete
- [ ] Result pagination/infinite scroll
- [ ] Video preview on hover
- [ ] Thumbnail disk caching
- [ ] Sort/filter results by date, views, duration
- [ ] Export search results to file

### Integration Points

The component is designed to be extended:

1. **Custom Result Actions**: Add more buttons to result cards
2. **Preview Integration**: Connect to video player component
3. **Queue Management**: Integrate with download queue
4. **Playlist Support**: Batch download from search results

## Troubleshooting

### Common Issues

**Problem**: "yt-dlp not found" error
- **Solution**: Install yt-dlp: `pip3 install yt-dlp` or `sudo apt install yt-dlp`

**Problem**: "Rate limited" error
- **Solution**: Wait a few minutes before searching again. Consider using a VPN or changing IP.

**Problem**: Thumbnails not loading
- **Solution**: Check network connection. Some platforms block direct thumbnail access.

**Problem**: Empty results
- **Solution**: Verify yt-dlp is up to date: `pip3 install --upgrade yt-dlp`

**Problem**: UI freezes during search
- **Solution**: Ensure you're using `gtk4::glib::spawn_future_local()` for async operations

## Examples

### Populate URL Field from Search

```rust
let url_entry = Entry::new();
let url_entry_clone = url_entry.clone();

search_view.set_download_callback(move |result| {
    url_entry_clone.set_text(&result.url);
});
```

### Trigger Download Directly

```rust
let downloader = VideoDownloader::new("/path/to/downloads".to_string());

search_view.set_download_callback(move |result| {
    let downloader_clone = downloader.clone();
    gtk4::glib::spawn_future_local(async move {
        let request = DownloadRequest {
            url: result.url.clone(),
            platform: result.platform,
            output_path: None,
            overwrite: false,
        };
        
        match downloader_clone.download(request, |_| {}).await {
            Ok(path) => println!("Downloaded to: {}", path),
            Err(e) => eprintln!("Download failed: {}", e),
        }
    });
});
```

### Add to Download Queue

```rust
let queue = DownloadQueue::new();
let queue_clone = queue.clone();

search_view.set_download_callback(move |result| {
    queue_clone.add_download(result.url, result.platform);
});
```

## Architecture Notes

### Type Disambiguation

The component carefully disambiguates between:
- `gtk4::Box` - GTK widget for layout
- `std::boxed::Box` - Heap allocation for trait objects

This is critical to avoid compilation errors. Always use fully-qualified paths:

```rust
// GTK widget
let container = gtk4::Box::new(Orientation::Vertical, 12);

// Heap-allocated callback
type DownloadCallback = std::boxed::Box<dyn Fn(SearchResult)>;
```

### State Management

Shared mutable state uses `Rc<RefCell<T>>`:

```rust
// Thumbnail cache
Rc<RefCell<HashMap<String, Pixbuf>>>

// Download callback
Rc<RefCell<Option<Box<dyn Fn(SearchResult)>>>>
```

This allows multiple closures to share state without borrow checker issues.

## License

Same as parent project (MIT).
