# Windows Portable Build

## Overview

VDownloader now has **two Windows build options**:

1. **Standard Build** (`vdownloader-windows.exe`) - 4 MB
   - Requires GTK4 runtime installation
   - Best for developers or users with MSYS2

2. **Portable Build** (`vdownloader-windows-portable.zip`) - ~100-150 MB
   - **No installation required**
   - All dependencies bundled
   - Just extract and run

## What's Included in Portable Build?

The portable package includes everything needed to run VDownloader:

- ✅ VDownloader executable
- ✅ All GTK4 runtime DLLs (~30 libraries)
- ✅ GDK PixBuf loaders (for image formats)
- ✅ GTK4 modules and plugins
- ✅ Icon themes (Hicolor, Adwaita)
- ✅ GSettings schemas (compiled)
- ✅ Launcher batch file (`VDownloader.bat`)
- ✅ README with instructions

## How to Use Portable Build

### For End Users:

1. **Download** `vdownloader-windows-portable.zip` from:
   - GitHub repository: `dist/vdownloader-windows-portable.zip`
   - GitHub Actions artifacts
   - Release page (for tagged versions)

2. **Extract** the ZIP file to any folder

3. **Download yt-dlp.exe**:
   - Get it from: https://github.com/yt-dlp/yt-dlp/releases
   - Place `yt-dlp.exe` in the extracted folder

4. **Run** the application:
   - Double-click `VDownloader.bat` (recommended)
   - OR double-click `vdownloader.exe` directly

That's it! No installation, no admin rights required.

### For Developers:

Build locally using the provided script:

```bash
# On Windows with MSYS2 installed
build-windows-portable.bat
```

This will:
- Build the release executable
- Copy all GTK4 dependencies
- Create the portable folder structure
- Generate a ZIP package (requires 7-Zip)

## Technical Details

### Bundled Libraries

The portable build includes these essential DLLs:
- GTK4 core libraries
- Cairo graphics engine
- Pango text rendering
- GDK PixBuf image loading
- GLib utilities
- Supporting libraries (zlib, libpng, etc.)

### Environment Setup

The launcher script (`VDownloader.bat`) automatically:
- Sets PATH to include bundled DLLs
- Configures GTK data directories
- Sets GDK PixBuf module paths
- Starts the application

### Directory Structure

```
windows-portable/
├── vdownloader.exe          # Main executable
├── VDownloader.bat           # Launcher script
├── README.txt                # User instructions
├── *.dll                     # GTK4 and dependency DLLs (~30 files)
├── lib/
│   ├── gdk-pixbuf-2.0/      # Image loaders
│   └── gtk-4.0/              # GTK modules
└── share/
    ├── glib-2.0/schemas/     # Compiled schemas
    └── icons/                # Icon themes
```

## Automated Builds

GitHub Actions automatically:
1. Builds both standard and portable versions
2. Creates the portable ZIP package
3. Commits both to the repository (`dist/` folder)
4. Makes them available as artifacts
5. Includes in releases (for version tags)

The workflow runs on every push to master.

## Size Comparison

| Build Type | Size | Dependencies | Installation |
|------------|------|--------------|--------------|
| Standard   | ~4 MB | Requires GTK4 | User must install MSYS2 |
| Portable   | ~100-150 MB | All bundled | Just extract and run |

## Advantages of Portable Build

✅ **Zero Installation** - No admin rights needed  
✅ **Self-Contained** - All dependencies included  
✅ **Portable** - Run from USB drive or any folder  
✅ **User-Friendly** - Simple extract-and-run experience  
✅ **Offline** - No internet needed after download  

## Trade-offs

⚠️ **Larger Download** - ~25-35x bigger than standard build  
⚠️ **Not Truly Standalone** - Still requires yt-dlp.exe  

## Why Not Bundle yt-dlp?

`yt-dlp` updates frequently (weekly) with site compatibility fixes. Bundling it would:
- Require frequent rebuilds
- Make the package even larger
- Prevent users from updating yt-dlp independently

We keep it separate so users can update it easily without waiting for a new VDownloader release.

## Comparison with Other Formats

| Format | Linux | Windows | macOS |
|--------|-------|---------|-------|
| Standard Binary | 4 MB | 4 MB | 4 MB |
| AppImage | 36 MB | N/A | N/A |
| Portable ZIP | N/A | 100-150 MB | N/A |
| App Bundle | N/A | N/A | ~10 MB |

## Building from Source

If you want to build locally:

### Requirements:
- Windows 10/11
- MSYS2 with UCRT64 environment
- Rust toolchain
- GTK4 development libraries

### Steps:
```bash
# Install dependencies
pacman -S mingw-w64-ucrt-x86_64-gtk4 mingw-w64-ucrt-x86_64-toolchain

# Run build script
build-windows-portable.bat
```

The script will create `dist/windows-portable/` with all files and optionally create a ZIP if 7-Zip is installed.

## For Distributors

If you want to distribute VDownloader:

1. **Choose the right build**:
   - Standard: For tech-savvy users
   - Portable: For general audience

2. **Include yt-dlp**:
   - Download latest yt-dlp.exe
   - Place in the portable folder
   - Mention it in your distribution notes

3. **Repackaging**:
   - You can repackage the portable build
   - Include yt-dlp if desired
   - Add your own launcher/installer
   - Credit original project

## Troubleshooting

### "Application failed to start"
- Ensure you're using `VDownloader.bat` launcher
- Check that all DLLs are present in the folder

### Missing icons or themes
- Schemas might not be compiled
- Run: `glib-compile-schemas.exe share/glib-2.0/schemas`

### "yt-dlp not found"
- Download yt-dlp.exe
- Place in same folder as vdownloader.exe
- Or add to system PATH

## Future Improvements

Potential enhancements for portable build:
- [ ] Auto-download yt-dlp if missing
- [ ] Include FFmpeg for audio extraction
- [ ] Create installer wrapper (NSIS/Inno Setup)
- [ ] Code signing for Windows SmartScreen
- [ ] Reduce size with MinGW dependencies instead of UCRT64

## Questions?

Check the main documentation:
- `BUILD.md` - Build instructions
- `DISTRIBUTION.md` - Distribution guide
- `README.md` - Project overview
