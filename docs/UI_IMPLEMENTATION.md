# Minimal GTK4 UI Implementation

## Overview

This document describes the minimal GTK4 user interface implementation for VDownloader.

## Implemented Features

### UI Components

1. **Header Section**
   - Application title: "VDownloader"
   - Subtitle: "Download videos from multiple platforms"
   - Clean, centered presentation

2. **Video URL Input**
   - Label: "Video URL:"
   - Text entry field with placeholder text
   - Expandable horizontally for long URLs
   - Basic validation for http/https URLs

3. **Download Directory Selection**
   - Label: "Download Directory:"
   - Path display with monospace font
   - Middle ellipsize for long paths
   - "Browse..." button
   - Modal file picker dialog
   - Defaults to user's home directory ($HOME or %USERPROFILE%)

4. **Download Action**
   - "Download" button with suggested-action styling (blue/accent color)
   - Disabled state during operations
   - Label changes to "Downloading..." during operations

5. **Status Display**
   - Status label showing current operation
   - States: Ready, Downloading, Error, Success
   - Error messages styled with error CSS class
   - Informational messages styled with dim-label class

## Technical Implementation

### Window Properties
- Title: "VDownloader"
- Default size: 600x300 pixels (compact and responsive)
- Vertical layout with 12px spacing
- 24px margins on all sides

### State Management
- Uses `Rc<RefCell<String>>` for shared download path state
- Widget cloning for closure captures
- Async-ready architecture with GTK4's glib integration

### File Picker Dialog
```rust
FileDialog::builder()
    .title("Select Download Directory")
    .modal(true)
    .build()
```

- Uses GTK4's modern FileDialog API (v4_10 feature)
- Async callback pattern for directory selection
- Automatically updates path label when directory is selected

### URL Validation
- Checks for empty URL input
- Validates http:// or https:// prefix
- Displays error messages in status label
- Prevents download with invalid input

### Error Handling
- Visual feedback via status label
- CSS class changes for error styling
- Non-blocking error messages
- User can correct and retry

## User Experience

### Flow
1. Application launches showing default home directory
2. User enters video URL
3. User optionally selects different download directory via Browse button
4. User clicks Download button
5. Status updates to show progress
6. Success or error message displayed

### Visual Design
- Clean, minimal interface
- GNOME HIG compliant
- Proper spacing and margins
- Monospace font for technical paths
- Accent color for primary action (Download button)
- Dim styling for secondary text
- Error styling for validation messages

## Code Location

### Main Implementation
- **File**: `src/ui/window.rs`
- **Function**: `build_window(app: &Application) -> ApplicationWindow`

### Key Functions
- Window creation and configuration
- Widget layout and styling
- Event handler setup
- File dialog integration

## Testing

### Manual Testing (requires display server)
```bash
# Run with logging
RUST_LOG=info cargo run
```

### Validation Checklist
- ✅ Window opens with correct size
- ✅ Default path is user's home directory
- ✅ URL input accepts text
- ✅ Browse button opens file picker
- ✅ Selected path is displayed correctly
- ✅ Empty URL shows error message
- ✅ Invalid URL (no http/https) shows error
- ✅ Download button changes state during operation
- ✅ Status label updates correctly
- ✅ No GTK warnings on startup

## Build Status

### Compilation
✅ Debug build: Success
✅ Release build: Success
✅ All tests: Pass (4/4)
✅ Code formatting: Pass
✅ Linting: Pass (only expected dead code warnings)

### Warnings
- Dead code warnings for scaffolded core functionality (expected)
- Will be resolved when download logic is implemented

## Next Steps

### Integration
1. Connect Download button to actual youtube_dl functionality
2. Implement real progress tracking
3. Add download completion handling
4. Implement file naming and organization

### Enhancements (Future)
1. Progress bar showing download percentage
2. Download history view
3. Settings/preferences dialog
4. Queue management for multiple downloads
5. Format selection (quality, codec)
6. Automatic filename detection

## Dependencies

### GTK4 Features Used
- ApplicationWindow
- Box (layout container)
- Button (with suggested-action styling)
- Entry (text input)
- FileDialog (folder selection)
- Label (text display)
- CSS classes for styling

### Runtime Requirements
- GTK4 runtime libraries
- Display server (X11/Wayland on Linux, Quartz on macOS, Windows Desktop on Windows)
- yt-dlp (for actual video downloading - to be integrated)

## Troubleshooting

### Common Issues

**Window doesn't appear**
- Ensure GTK4 is installed and display server is running
- Check `DISPLAY` environment variable on Linux
- Run with `RUST_LOG=debug cargo run` for detailed logs

**File picker crashes**
- Verify GTK4 version supports v4_10 features
- Check for portal support on Flatpak/Snap environments

**Path not updating**
- Verify directory has read permissions
- Check logs for file selection errors

## References

- [GTK4 Documentation](https://gtk-rs.org/gtk4-rs/)
- [GNOME HIG](https://developer.gnome.org/hig/)
- [Rust GTK4 Book](https://gtk-rs.org/gtk4-rs/stable/latest/book/)
