#!/bin/bash
#*****************************************************************************#
#                                                                             #
#  build-macos.sh - macOS native build script      TTTTTTTT SSSSSSS II        #
#                                                     TT    SS      II        #
#  By: st93642@students.tsi.lv                        TT    SSSSSSS II        #
#                                                     TT         SS II        #
#  Created: Dec 07 2025 st93642                       TT    SSSSSSS II        #
#                                                                             #
#   Transport and Telecommunication Institute - Riga, Latvia                  #
#                       https://tsi.lv                                        #
#*****************************************************************************#

set -e

echo "========================================="
echo "VDownloader macOS Build Script"
echo "========================================="
echo ""

# Create dist directory
mkdir -p dist

echo "Building for macOS (native)..."
echo ""

# Build release binary
cargo build --release

if [ $? -eq 0 ]; then
    echo "✓ Build successful!"
    echo ""
    
    # Copy binary
    cp target/release/vdownloader dist/vdownloader-macos
    chmod +x dist/vdownloader-macos
    
    echo "✓ Binary copied to dist/vdownloader-macos"
    echo ""
    
    # Show size
    ls -lh dist/vdownloader-macos
    echo ""
    
    # Create app bundle
    echo "Creating macOS app bundle..."
    
    APP_DIR="dist/VDownloader.app"
    mkdir -p "$APP_DIR/Contents/MacOS"
    mkdir -p "$APP_DIR/Contents/Resources"
    
    # Copy binary
    cp target/release/vdownloader "$APP_DIR/Contents/MacOS/vdownloader"
    chmod +x "$APP_DIR/Contents/MacOS/vdownloader"
    
    # Create Info.plist
    cat > "$APP_DIR/Contents/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>vdownloader</string>
    <key>CFBundleIdentifier</key>
    <string>lv.tsi.vdownloader</string>
    <key>CFBundleName</key>
    <string>VDownloader</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF
    
    echo "✓ App bundle created at $APP_DIR"
    echo ""
else
    echo "✗ Build failed!"
    exit 1
fi

echo "========================================="
echo "Build complete!"
echo "========================================="
echo ""
echo "Outputs:"
echo "  Executable: dist/vdownloader-macos"
echo "  App Bundle: dist/VDownloader.app"
echo ""
echo "Requirements for running:"
echo "  • GTK4 runtime (install via Homebrew: brew install gtk4)"
echo "  • yt-dlp (install via Homebrew: brew install yt-dlp)"
echo ""
echo "To install dependencies:"
echo "  brew install gtk4 yt-dlp"
echo ""
