# VDownloader Distribution Package

## What's Included

This package contains standalone executables for VDownloader - a cross-platform video downloader with a modern GTK4 interface.

## System Requirements

### All Platforms
- **yt-dlp**: Video download engine (must be installed separately)

### Linux
- GTK4 runtime (usually pre-installed on modern distributions)
- Tested on: Ubuntu 22.04+, Fedora 38+, Arch Linux

### macOS
- macOS 10.15 (Catalina) or later
- GTK4 runtime (install via Homebrew)

### Windows
- Windows 10 or later
- GTK4 runtime (included with MSYS2 installation)

## Installation Instructions

### Linux

1. Download `vdownloader-linux`
2. Make it executable:
   ```bash
   chmod +x vdownloader-linux
   ```
3. Install yt-dlp:
   ```bash
   # Ubuntu/Debian
   sudo apt install yt-dlp
   
   # Or via pip
   pip install yt-dlp
   ```
4. Run:
   ```bash
   ./vdownloader-linux
   ```

**Optional**: Move to system path:
```bash
sudo mv vdownloader-linux /usr/local/bin/vdownloader
```

### macOS

1. Download `vdownloader-macos` or `VDownloader.app`
2. Install dependencies via Homebrew:
   ```bash
   brew install gtk4 yt-dlp
   ```
3. For the app bundle:
   - Copy `VDownloader.app` to `/Applications/`
   - First run: Right-click → Open (to bypass Gatekeeper)
   
4. For the executable:
   ```bash
   chmod +x vdownloader-macos
   ./vdownloader-macos
   ```

### Windows

1. Download `vdownloader-windows.exe`
2. Install dependencies:
   - **MSYS2**: Download from https://www.msys2.org/
     - Open MSYS2 UCRT64 terminal
     - Run: `pacman -S mingw-w64-ucrt-x86_64-gtk4`
   - **yt-dlp**: Download from https://github.com/yt-dlp/yt-dlp/releases
     - Place `yt-dlp.exe` in `C:\Windows\System32\` or same folder as vdownloader
3. Double-click `vdownloader-windows.exe` to run

**Alternative**: Add MSYS2 bin directory to PATH:
```
C:\msys64\ucrt64\bin
```

## Features

- 🎬 Download videos from multiple platforms:
  - YouTube
  - Dzen (VK Video)
  - Rutube
  - And many more supported by yt-dlp

- 🔍 Search functionality
- 📥 Queue management
- 📊 Real-time download progress
- 🎨 Modern GTK4 interface
- 🌐 Cross-platform support

## Usage

1. Launch VDownloader
2. Enter a video URL or search for videos
3. Select output directory (defaults to `~/Videos` or `%USERPROFILE%\Videos`)
4. Click Download
5. Monitor progress in the download queue

## Supported Platforms

VDownloader supports downloading from any platform supported by yt-dlp, including:
- YouTube
- Dzen (VK Video)
- Rutube
- Vimeo
- Dailymotion
- And 1000+ more sites

## Troubleshooting

### "yt-dlp not found"
- Make sure yt-dlp is installed and available in your system PATH
- Test by running `yt-dlp --version` in terminal

### "GTK initialization failed"
- **Linux**: Install GTK4 runtime: `sudo apt install libgtk-4-1`
- **macOS**: Install via Homebrew: `brew install gtk4`
- **Windows**: Install MSYS2 and GTK4 as described above

### Linux: "Permission denied"
- Make the file executable: `chmod +x vdownloader-linux`

### macOS: "Cannot open because developer cannot be verified"
- Right-click the app → Open → Open
- Or run: `xattr -cr VDownloader.app`

### Windows: Missing DLL errors
- Make sure MSYS2 GTK4 is installed
- Add `C:\msys64\ucrt64\bin` to your PATH environment variable

## Uninstallation

### Linux
```bash
rm /usr/local/bin/vdownloader  # If installed to system
rm ~/.config/vdownloader        # Remove config (optional)
```

### macOS
```bash
rm -rf /Applications/VDownloader.app
rm -rf ~/Library/Application\ Support/vdownloader
```

### Windows
- Delete `vdownloader-windows.exe`
- Delete `%APPDATA%\vdownloader` (optional)

## Building from Source

See `BUILD.md` for detailed instructions on building VDownloader from source code.

## License

MIT License - See LICENSE file for details

## Support & Issues

- GitHub: https://github.com/st93642/VDownloader
- Report bugs: Create an issue on GitHub

## Version

Current version: 0.1.0

Built with ❤️ using Rust and GTK4
