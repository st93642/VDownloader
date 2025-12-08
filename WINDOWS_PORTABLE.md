# Windows Portable Build

## Overview

VDownloader now has **two Windows build options**:

1. **Standard Build** (`vdownloader-windows.exe`) - 4 MB
   - Requires GTK4 runtime installation
   - Best for developers or users with MSYS2

2. **Portable Build** (`vdownloader-windows-portable.zip`) - ~150-200 MB
   - **No installation required**
   - All dependencies bundled (including ffmpeg)
   - Just extract and run

## What's Included in Portable Build?

The portable package includes everything needed to run VDownloader:

- ✅ VDownloader executable
- ✅ **yt-dlp.exe** (automatically downloaded, latest version)
- ✅ **ffmpeg.exe** and **ffprobe.exe** (for best video format support)
- ✅ All GTK4 runtime DLLs (~30 libraries)
- ✅ GDK PixBuf loaders (for image formats)
- ✅ GTK4 modules and plugins
- ✅ Icon themes (Hicolor, Adwaita)
- ✅ GSettings schemas (compiled)
- ✅ Launcher batch file (`VDownloader.bat`)
- ✅ README with instructions

## How to Use Portable Build

### For End Users

1. **Download** `vdownloader-windows-portable.zip` from:
   - GitHub repository: `dist/vdownloader-windows-portable.zip`
   - GitHub Actions artifacts
   - Release page (for tagged versions)

2. **Extract** the ZIP file to any folder

3. **Run** the application:
   - Double-click `VDownloader.bat` (recommended)
   - OR double-click `vdownloader.exe` directly

That's it! No installation, no admin rights, no additional downloads required.

**Note**: yt-dlp is automatically included in the package (latest version at build time).

### For Developers

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

```text
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
| Portable   | ~150-200 MB | All bundled (with ffmpeg) | Just extract and run |

## Advantages of Portable Build

✅ **Zero Installation** - No admin rights needed  
✅ **Fully Self-Contained** - All dependencies AND yt-dlp included  
✅ **Portable** - Run from USB drive or any folder  
✅ **User-Friendly** - Simple extract-and-run experience  
✅ **Offline** - No internet needed after download  
✅ **Always Latest** - yt-dlp version from build time

## Trade-offs

⚠️ **Larger Download** - ~25-35x bigger than standard build  
⚠️ **yt-dlp Updates** - Bundled version may become outdated (update manually if needed)

## yt-dlp Updates

`yt-dlp` is now bundled with the portable build! The version included is the latest at build time.

Since yt-dlp updates frequently (weekly) with site compatibility fixes:

- **Automatic**: Each new VDownloader build includes the latest yt-dlp
- **Manual Update**: You can replace `yt-dlp.exe` with a newer version anytime
- **Download**: Get latest from <https://github.com/yt-dlp/yt-dlp/releases>

Simply replace the `yt-dlp.exe` file in your portable folder to update it independently.

## Comparison with Other Formats

| Format | Linux | Windows | macOS |
|--------|-------|---------|-------|
| Standard Binary | 4 MB | 4 MB | 4 MB |
| AppImage | 36 MB | N/A | N/A |
| Portable ZIP | N/A | 150-200 MB | N/A |
| App Bundle | N/A | N/A | ~10 MB |

## Building from Source

If you want to build locally:

### Requirements

- Windows 10/11
- MSYS2 with UCRT64 environment
- Rust toolchain
- GTK4 development libraries

### Steps

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
   - Portable: For general audience (recommended - includes yt-dlp)

2. **Repackaging**:
   - You can repackage the portable build
   - yt-dlp is already included
   - Add your own launcher/installer if desired
   - Credit original project

## Troubleshooting

### "Application failed to start"

- Ensure you're using `VDownloader.bat` launcher
- Check that all DLLs are present in the folder

### Missing icons or themes

- Schemas might not be compiled
- Run: `glib-compile-schemas.exe share/glib-2.0/schemas`

### "yt-dlp not found"

- Should not occur (yt-dlp is bundled)
- If missing, download from <https://github.com/yt-dlp/yt-dlp/releases>
- Place in same folder as vdownloader.exe

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
