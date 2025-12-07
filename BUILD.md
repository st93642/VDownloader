# Building VDownloader for Multiple Platforms

This guide explains how to build VDownloader for Linux, macOS, and Windows.

## Prerequisites

All platforms require:

- **Rust** (1.70 or later): Install from <https://rustup.rs/>
- **GTK4 development libraries**
- **yt-dlp**: Video download tool

## Linux

### Install Dependencies

**Ubuntu/Debian:**

```bash
sudo apt-get update
sudo apt-get install -y libgtk-4-dev build-essential
sudo apt-get install yt-dlp  # or use pip: pip install yt-dlp
```

**Fedora:**

```bash
sudo dnf install gtk4-devel gcc
sudo dnf install yt-dlp
```

**Arch Linux:**

```bash
sudo pacman -S gtk4 base-devel
sudo pacman -S yt-dlp
```

### Build

```bash
chmod +x build.sh
./build.sh
```

Output: `dist/vdownloader-linux`

### Cross-compile for Windows from Linux (Optional)

Install MinGW:

```bash
sudo apt-get install mingw-w64
rustup target add x86_64-pc-windows-gnu
```

Then run `./build.sh` - it will automatically build for Windows too.

## macOS

### Install Dependencies

Install Homebrew if not already installed:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

Install required packages:

```bash
brew install gtk4 pkg-config
brew install yt-dlp
```

### Build

```bash
chmod +x build-macos.sh
./build-macos.sh
```

Outputs:

- Executable: `dist/vdownloader-macos`
- App Bundle: `dist/VDownloader.app`

You can copy `VDownloader.app` to `/Applications/` to install it system-wide.

## Windows

### Install Dependencies

1. **Install Rust**: Download from <https://rustup.rs/> and run the installer

2. **Install MSYS2**: Download from <https://www.msys2.org/>

   Open MSYS2 UCRT64 terminal and run:

   ```bash
   pacman -Syu
   pacman -S mingw-w64-ucrt-x86_64-gtk4
   pacman -S mingw-w64-ucrt-x86_64-toolchain
   pacman -S mingw-w64-ucrt-x86_64-pkgconf
   ```

3. **Install yt-dlp**: Download from <https://github.com/yt-dlp/yt-dlp/releases>
   - Place `yt-dlp.exe` in `C:\Windows\System32\` or add to PATH

### Build

Open Command Prompt or PowerShell in the project directory:

```cmd
build.bat
```

Output: `dist\vdownloader-windows.exe`

### Running on Windows

Users need to have GTK4 runtime installed. They can either:

- Install MSYS2 and GTK4 as above
- Or bundle GTK4 DLLs with your application (see Bundle section below)

## Distribution Bundles

### Linux - AppImage (Recommended)

Create a portable AppImage that includes all dependencies:

```bash
# Install linuxdeploy
wget https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
chmod +x linuxdeploy-x86_64.AppImage

# Create AppImage
./linuxdeploy-x86_64.AppImage \
    --executable=dist/vdownloader-linux \
    --desktop-file=vdownloader.desktop \
    --icon-file=icon.png \
    --appdir=AppDir \
    --output appimage
```

### Windows - Installer with Dependencies

Use Inno Setup or WiX to create an installer that includes:

- `vdownloader-windows.exe`
- GTK4 runtime DLLs
- yt-dlp.exe

Or create a portable package with all DLLs in the same folder.

### macOS - DMG Package

Create a DMG disk image:

```bash
hdiutil create -volname "VDownloader" \
    -srcfolder dist/VDownloader.app \
    -ov -format UDZO \
    dist/VDownloader.dmg
```

## Quick Build Summary

| Platform | Command | Output |
|----------|---------|--------|
| Linux | `./build.sh` | `dist/vdownloader-linux` |
| macOS | `./build-macos.sh` | `dist/VDownloader.app` |
| Windows | `build.bat` | `dist\vdownloader-windows.exe` |

## File Sizes (Approximate)

- Linux: ~15-20 MB (stripped)
- macOS: ~15-20 MB
- Windows: ~15-20 MB

GTK4 dependencies add ~50-100 MB when bundled.

## Notes

- **GTK4 Dependency**: All platforms require GTK4 runtime. Consider bundling for easier distribution.
- **yt-dlp**: Must be available in system PATH on the target machine.
- **Cross-compilation**: Building GTK applications for other platforms is complex. Native builds are recommended.

## Troubleshooting

### "GTK not found" errors during build

Make sure `pkg-config` can find GTK4:

```bash
pkg-config --modversion gtk4
```

### Windows: Missing DLLs at runtime

Copy required DLLs from MSYS2 to the same directory as the .exe:

```bash
ldd dist/vdownloader-windows.exe  # Shows required DLLs
```

### macOS: "Cannot open because developer cannot be verified"

```bash
xattr -cr dist/VDownloader.app
```

## CI/CD

For automated builds, see `.github/workflows/build.yml` (if available) or set up GitHub Actions to build for all platforms automatically.
