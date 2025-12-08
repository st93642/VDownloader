# Windows Builds

This directory contains Windows builds automatically created by GitHub Actions.

## Files

### 1. `vdownloader-windows.exe` (Standard Build)
- **Size**: ~4 MB
- **Requirements**: Requires GTK4 runtime to be installed separately
- **Best for**: Users who already have MSYS2/GTK4 installed

**To run:**
1. Install GTK4 via MSYS2:
   ```bash
   pacman -S mingw-w64-ucrt-x86_64-gtk4
   ```
2. Install yt-dlp from [releases](https://github.com/yt-dlp/yt-dlp/releases)
3. Run `vdownloader-windows.exe`

### 2. `vdownloader-windows-portable.zip` (Portable Build)
- **Size**: ~100-150 MB
- **Requirements**: NONE - everything included!
- **Best for**: Users who want zero installation
- **Includes**: All GTK4 DLLs, icon themes, schemas, AND yt-dlp

**To run:**
1. Extract the ZIP file
2. Run `VDownloader.bat` or `vdownloader.exe`
3. That's it!

## Platform

- **Architecture**: Windows x86_64
- **Target**: `x86_64-pc-windows-gnu`
- **Compiler**: Rust stable with MSYS2 UCRT64 toolchain

## Automatic Updates

Both builds are automatically rebuilt and updated whenever code is pushed to the master branch. The build process:

1. Compiles optimized release binary
2. For portable: Bundles all GTK4 dependencies and creates ZIP
3. Commits updated builds back to this repository

## Build Optimizations

- **opt-level**: "z" (optimize for size)
- **LTO**: Enabled (link-time optimization)
- **Strip**: Enabled (removes debug symbols)
- **codegen-units**: 1 (better optimization)

## Last Updated

Check git commit history for these files to see when they were last built.
