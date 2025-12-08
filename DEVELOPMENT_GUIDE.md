# VDownloader Development Guide

Guide for developers who want to understand, modify, and extend VDownloader.

## Getting Started

### 1. Understand the Project Structure

VDownloader is organized into two main layers:

**Core Layer** (`src/core/`):
- Business logic independent of UI framework
- Video downloading, searching, queue management
- Error handling
- Platform detection

**UI Layer** (`src/ui/`):
- GTK4-based user interface
- Components and widgets
- Signal handling and callbacks
- Display of information

Read [CODEBASE_INDEX.md](CODEBASE_INDEX.md) for detailed module overview.

### 2. Understand the Architecture

VDownloader follows a layered architecture:
- UI → calls → Core services
- Core → calls → yt-dlp and external APIs
- No circular dependencies

Read [ARCHITECTURE.md](ARCHITECTURE.md) for detailed design patterns.

### 3. Set Up Development Environment

Follow [SETUP.md](SETUP.md) for:
- Prerequisites (Rust, GTK4, yt-dlp)
- Installation steps
- Building the project
- Running tests

## Common Development Tasks

### Adding Support for a New Video Platform

Example: Add support for "NewVideo" platform at newvideo.com

#### Step 1: Update Platform Enum

Edit `src/core/downloader.rs`:

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
    NewVideo,  // Add here
    Other,
}
```

#### Step 2: Update Platform Detection

Edit `src/core/downloader.rs`, update `detect_platform()`:

```rust
pub fn detect_platform(url: &str) -> Platform {
    debug!("Detecting platform for URL: {}", url);

    if url.contains("youtube.com") || url.contains("youtu.be") {
        Platform::YouTube
    } else if url.contains("tiktok.com") {
        Platform::TikTok
    }
    // ... other platforms ...
    else if url.contains("newvideo.com") {
        Platform::NewVideo
    }
    else {
        Platform::Other
    }
}
```

#### Step 3: Update Search Platform Detection

Edit `src/core/search.rs`, update `platform_from_hint()`:

```rust
fn platform_from_hint(extractor: &str) -> Platform {
    match extractor {
        "youtube" => Platform::YouTube,
        "tiktok" => Platform::TikTok,
        // ... other platforms ...
        "newvideo" => Platform::NewVideo,
        _ => Platform::Other,
    }
}
```

#### Step 4: Add Tests

Edit `src/core/downloader.rs`, add test in `tests` module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_newvideo_platform() {
        assert_eq!(
            VideoDownloader::detect_platform("https://newvideo.com/watch?v=123"),
            Platform::NewVideo
        );
    }

    #[test]
    fn test_detect_newvideo_alternative_url() {
        assert_eq!(
            VideoDownloader::detect_platform("newvideo.com/video/456"),
            Platform::NewVideo
        );
    }
}
```

#### Step 5: Test Your Changes

```bash
# Run your new tests
cargo test test_detect_newvideo

# Run all tests to ensure you didn't break anything
cargo test

# Format and lint
cargo fmt
cargo clippy
```

#### Step 6: Verify With Real Videos

```bash
# Try downloading from the new platform
RUST_LOG=info cargo run

# In the UI:
# 1. Enter a URL from newvideo.com
# 2. Select output directory
# 3. Click Download
```

That's it! yt-dlp will automatically handle the platform. No downloader changes needed.

### Adding a New Search Platform

Example: Add searching on "NewSearchEngine" at newsearch.example.com

#### Step 1: Update Search Service

Edit `src/core/search.rs`, update `SearchService::search()`:

```rust
pub async fn search(
    &self,
    query: &str,
    limit: Option<u32>,
) -> Result<Vec<SearchResult>, SearchError> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Err(SearchError::InvalidQuery(
            "Query cannot be empty".to_string(),
        ));
    }

    let limit = limit
        .filter(|value| *value > 0)
        .unwrap_or(self.default_limit);

    // ... existing URL check ...

    debug!("Executing multi-platform search for: {}", trimmed);
    let mut tasks = Vec::new();

    // ... existing YouTube and Dzen searches ...

    // 4. New Search Engine Search
    let new_query = trimmed.to_string();
    let new_limit = limit;
    tasks.push(tokio::spawn(async move {
        Self::search_new_engine(&new_query, new_limit).await
    }));

    // ... rest of aggregation ...
}
```

#### Step 2: Implement Search Function

Still in `src/core/search.rs`:

```rust
async fn search_new_engine(
    query: &str,
    limit: u32,
) -> Result<Vec<SearchResult>, SearchError> {
    let search_url = format!(
        "https://newsearch.example.com/search?q={}",
        urlencoding::encode(query)
    );
    
    Self::execute_search_command(
        &search_url,
        &["--dump-json", "--flat-playlist", "--skip-download"],
        Some(limit),
    )
    .await
}
```

#### Step 3: Test Search

```bash
# Run the application
RUST_LOG=debug cargo run

# In the UI:
# 1. Go to Search tab
# 2. Enter a search query
# 3. Check if results include items from the new engine
```

### Modifying the UI

Example: Add a "Open in Browser" button to search results

#### Step 1: Understand Current UI

Review `src/ui/components/search_view.rs`:
- How SearchResult cards are built
- How buttons are added
- How callbacks work

#### Step 2: Add Button to Result Card

Edit `src/ui/components/search_view.rs`, in the result card building code:

```rust
// Existing code builds the card box...
let download_button = Button::builder()
    .label("Download")
    .css_classes(vec!["suggested-action".to_string()])
    .build();

// Add new button
let browser_button = Button::builder()
    .label("Open in Browser")
    .build();

// Handle click
let result_url = result.url.clone();
browser_button.connect_clicked(move |_| {
    // Open URL in browser using xdg-open (Linux), open (macOS), start (Windows)
    let url = result_url.clone();
    gtk4::glib::spawn_future_local(async move {
        let _ = tokio::process::Command::new("xdg-open")
            .arg(&url)
            .spawn();
    });
});

// Add to card
button_box.append(&download_button);
button_box.append(&browser_button);
```

#### Step 3: Test UI Change

```bash
# Build and run
cargo run

# Verify the new button appears in search results
# Click it to test opening URLs in browser
```

### Adding a New UI Component

Example: Add a "Download History" panel

#### Step 1: Create Component File

Create `src/ui/components/history.rs`:

```rust
use gtk4::{Box, Label, Orientation};

pub struct HistoryView {
    container: Box,
}

impl HistoryView {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 12);
        container.set_margin_top(12);
        container.set_margin_bottom(12);
        container.set_margin_start(12);
        container.set_margin_end(12);

        let title = Label::new(Some("Download History"));
        title.add_css_class("title-2");
        container.append(&title);

        let empty_label = Label::new(Some("No downloads yet"));
        empty_label.add_css_class("dim-label");
        container.append(&empty_label);

        Self { container }
    }

    pub fn widget(&self) -> &Box {
        &self.container
    }
}
```

#### Step 2: Register Component Module

Edit `src/ui/components/mod.rs`:

```rust
pub mod download_queue;
pub mod history;  // Add here
pub mod preview_window;
pub mod search_view;
```

#### Step 3: Add to Main Window

Edit `src/ui/window.rs`:

```rust
use crate::ui::components::history::HistoryView;

// In build_window function:
let history_view = HistoryView::new();

// Add to content stack
content_stack.add_titled(
    history_view.widget(),
    Some("history"),
    "History"
);
```

#### Step 4: Test Component

```bash
cargo run

# Verify the new History tab appears
# Check that it renders correctly
```

### Fixing a Bug

Example: Search results showing duplicate titles

#### Step 1: Reproduce the Bug

```bash
cargo run
# Go to Search tab
# Enter a query
# Look for duplicates
```

#### Step 2: Locate the Issue

Based on symptoms, check:
- `src/core/search.rs` - Search result aggregation
- `src/ui/components/search_view.rs` - Result display
- yt-dlp output parsing

#### Step 3: Add a Test

In the relevant module, add `#[cfg(test)]` test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_no_duplicates() {
        // Test that aggregation doesn't duplicate results
    }
}
```

#### Step 4: Fix the Bug

Make minimal code changes to fix the issue.

#### Step 5: Verify Fix

```bash
cargo test test_search_no_duplicates

# Also test manually
cargo run
```

### Optimizing Performance

#### Profiling Downloads

```bash
# Run with logging to see timing
RUST_LOG=debug cargo run

# Watch for slow operations in logs
```

#### Profiling Searches

Search performance depends on:
1. yt-dlp execution time (can't optimize much)
2. JSON parsing (already streaming)
3. Thumbnail loading (already asynchronous)

#### Memory Usage

Check memory with system tools:
```bash
# Linux
watch -n 1 'ps aux | grep vdownloader'

# Check heap size
valgrind --tool=massif ./target/release/vdownloader
```

### Writing Tests

VDownloader uses standard Rust testing patterns.

#### Unit Test Example

In `src/core/downloader.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_url() {
        assert!(VideoDownloader::validate_url("https://youtube.com/watch?v=123").is_ok());
    }

    #[test]
    fn test_validate_invalid_url() {
        let result = VideoDownloader::validate_url("not a url");
        assert!(result.is_err());
        match result {
            Err(DownloadError::InvalidUrl(_)) => (),
            _ => panic!("Expected InvalidUrl error"),
        }
    }
}
```

#### Async Test Example

In `src/core/search.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_invalid_query() {
        let service = SearchService::new(10);
        let result = service.search("", None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_search_min_limit_enforcement() {
        let service = SearchService::new(1);
        let result = service.search("test", Some(0)).await;
        // Should use default_limit of 1, not 0
        if let Ok(results) = result {
            assert!(results.len() <= 1);
        }
    }
}
```

#### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_validate_valid_url

# Run tests in debug mode (faster compilation)
cargo test

# Run tests in release mode (slower build, faster execution)
cargo test --release
```

## Code Style and Conventions

### Formatting

Always format code with `cargo fmt`:

```bash
cargo fmt
cargo fmt --check  # Check without modifying
```

### Naming Conventions

Follow Rust standards:
- **Modules**: `snake_case` (e.g., `download_queue.rs`)
- **Functions**: `snake_case` (e.g., `pub fn download_video()`)
- **Types/Structs**: `PascalCase` (e.g., `struct VideoDownloader`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `const APP_ID: &str = "..."`)
- **Variables**: `snake_case` (e.g., `let output_dir = "..."`)

### Comments

Comment complex logic, not obvious code:

```rust
// Good: Explains why
let limit = limit.max(1);  // Ensure minimum 1 result

// Bad: Explains what
let limit = limit.max(1);  // Set limit to max of limit and 1
```

### Error Handling

Use custom error types and Result:

```rust
// Good
pub fn download(&self, url: &str) -> Result<String, DownloadError> {
    if url.is_empty() {
        return Err(DownloadError::InvalidUrl("Empty URL".to_string()));
    }
    // ...
}

// Avoid
pub fn download(&self, url: &str) -> String {
    if url.is_empty() {
        panic!("Empty URL");  // Don't panic
    }
    // ...
}
```

### Async Code

Use async/await with Tokio:

```rust
// Good: Non-blocking
pub async fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
    let output = Command::new("yt-dlp").output().await?;
    Ok(parse_output(&output))
}

// Avoid: Blocking
pub fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
    let output = std::process::Command::new("yt-dlp").output()?;  // Blocks!
    Ok(parse_output(&output))
}
```

### GTK4 Specifics

Use `gtk4::Box` for layout to avoid conflict with `std::boxed::Box`:

```rust
// Good: Explicit gtk4::Box
let container = gtk4::Box::new(Orientation::Vertical, 12);

// Avoid: Can be ambiguous
use gtk4::Box;
let container = Box::new(Orientation::Vertical, 12);  // Which Box?
```

## Debugging Tips

### Logging

Add logging to understand execution flow:

```rust
use log::{debug, info, warn, error};

pub fn download(&self, url: &str) -> Result<String> {
    info!("Starting download from: {}", url);
    
    self.validate_url(url)?;
    debug!("URL validation passed");
    
    let platform = Self::detect_platform(url);
    info!("Detected platform: {:?}", platform);
    
    // ... actual download ...
    
    info!("Download completed");
    Ok(output)
}
```

Run with logging:
```bash
RUST_LOG=debug cargo run
```

### Inspecting State

Print state when debugging:

```rust
#[derive(Debug)]
struct MyStruct {
    field: String,
}

let obj = MyStruct { field: "value".to_string() };
dbg!(obj);  // Prints: [src/file.rs:123] obj = MyStruct { field: "value" }
```

### Stack Traces

Get full stack trace on panic:

```bash
RUST_BACKTRACE=full cargo run
```

## Common Patterns

### Shared Mutable State in GTK

GTK requires `Rc<RefCell<T>>` for shared mutable state in closures:

```rust
let state = Rc::new(RefCell::new(HashMap::new()));
let state_clone = state.clone();

button.connect_clicked(move |_| {
    let mut map = state_clone.borrow_mut();
    map.insert("key", "value");
});
```

### Thread-Safe State in Core Layer

Core layer uses `Arc<RwLock<T>>` for thread-safe state:

```rust
let items = Arc::new(RwLock::new(HashMap::new()));

tokio::spawn({
    let items = items.clone();
    async move {
        let mut map = items.write().await;
        map.insert("key", "value");
    }
});
```

### Error Propagation

Use `?` operator for clean error handling:

```rust
pub async fn process(&self) -> Result<Output, MyError> {
    let response = make_request().await?;  // Propagate error if Err
    let data = parse_response(&response)?;
    Ok(data)
}
```

### Async Closures in GTK

Use `gtk4::glib::spawn_future_local()`:

```rust
button.connect_clicked(|_| {
    gtk4::glib::spawn_future_local(async {
        let result = some_async_operation().await;
        // Update UI with result
    });
});
```

## Performance Tips

### Build Time
- Use debug build for development: `cargo build`
- Use release only when needed: `cargo build --release`
- Use `cargo check` for fast validation without compilation
- Use faster linker on Linux: `mold`

### Runtime
- Release build is optimized: `cargo build --release`
- Async operations are non-blocking: uses Tokio
- Thumbnail cache reduces network requests
- JSON parsing is streaming: doesn't load full output

### Binary Size
- Release build is stripped of debug symbols
- Binary compression with UPX if needed
- Optimize-for-size profile in Cargo.toml

## Getting Help

### Read Documentation
- [CODEBASE_INDEX.md](CODEBASE_INDEX.md) - What files exist
- [ARCHITECTURE.md](ARCHITECTURE.md) - How things work
- [SETUP.md](SETUP.md) - How to build
- [README.md](README.md) - Overview
- `cargo doc --open` - API documentation

### Run Tests
- `cargo test` - Verify your changes don't break existing functionality
- `cargo test -- --nocapture` - See test output

### Check Code Quality
- `cargo fmt` - Format your code
- `cargo clippy` - Get suggestions
- `cargo test` - Run tests

### Look at Examples
- Search for similar code in the codebase
- Check `#[cfg(test)]` modules for usage examples
- Review git history: `git log -p --follow filename`

## Before Submitting Changes

Checklist before committing:

- [ ] Code compiles: `cargo build`
- [ ] Tests pass: `cargo test`
- [ ] Code formatted: `cargo fmt`
- [ ] No clippy warnings: `cargo clippy`
- [ ] Commit message is clear and descriptive
- [ ] No debug print statements left
- [ ] Documentation is updated if needed

Quick verification:
```bash
cargo fmt && cargo clippy && cargo test && cargo build --release
```

## Summary

| Task | Files | Key Concept |
|------|-------|------------|
| Add platform | `downloader.rs`, `search.rs` | Update enum + detection functions |
| Add feature | `core/*.rs` | Implement service, add tests |
| Add UI component | `ui/components/*.rs` | Create widget struct, integrate |
| Fix bug | Locate in relevant module | Add test, fix, verify |
| Improve performance | Profile first, then optimize | Measure impact |
| Add test | Same file as code | `#[cfg(test)]` modules |

Questions? Check [CODEBASE_INDEX.md](CODEBASE_INDEX.md), [ARCHITECTURE.md](ARCHITECTURE.md), or open an issue!
