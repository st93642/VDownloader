# Quick Build Reference

## 📦 Available Build Scripts

| Script | Platform | Description |
|--------|----------|-------------|
| `build.sh` | Linux (multi) | Native Linux + optional Windows cross-compile |
| `build-macos.sh` | macOS | Native macOS build + app bundle |
| `build.bat` | Windows | Native Windows build |

## 🚀 Quick Start

### Linux
```bash
chmod +x build.sh
./build.sh
# Output: dist/vdownloader-linux
```

### macOS
```bash
chmod +x build-macos.sh
./build-macos.sh
# Output: dist/vdownloader-macos + dist/VDownloader.app
```

### Windows
```cmd
build.bat
REM Output: dist\vdownloader-windows.exe
```

## 📊 Build Results

Current build status:
- ✅ Linux: 4.0 MB (built)
- ⏳ Windows: ~4-5 MB (requires Windows or MinGW)
- ⏳ macOS: ~4-5 MB (requires macOS)

## 🔄 Automated Builds (GitHub Actions)

Push a tag to automatically build all platforms:
```bash
git tag v0.1.0
git push origin v0.1.0
```

Workflow: `.github/workflows/build.yml`

## 📁 Output Structure

```
dist/
├── vdownloader-linux          # Linux executable
├── vdownloader-windows.exe    # Windows executable
├── vdownloader-macos          # macOS executable
└── VDownloader.app/           # macOS app bundle
    └── Contents/
        ├── MacOS/vdownloader
        └── Info.plist
```

## 🔧 Dependencies

### Build-time
- Rust 1.70+
- GTK4 development libraries
- pkg-config

### Runtime
- GTK4 runtime
- yt-dlp

## 📖 Documentation

- **BUILD.md** - Detailed build instructions
- **DISTRIBUTION.md** - User installation guide
- **BUILD_STATUS.md** - Current build status and next steps

## ✅ Testing Checklist

- [ ] Linux build runs
- [ ] Windows build runs
- [ ] macOS build runs
- [ ] All platforms can download videos
- [ ] GTK UI renders correctly
- [ ] yt-dlp integration works

## 🐛 Common Issues

**"GTK not found"**: Install GTK4 dev packages  
**"yt-dlp not found"**: Install yt-dlp separately  
**Permission denied**: `chmod +x` the build script  

## 💡 Tips

- Use `--release` for production builds (automatically done by scripts)
- Binary size ~4MB (optimized with LTO and strip)
- Cross-compiling GTK apps is complex - native builds recommended

---

For detailed information, see BUILD.md
