#!/bin/bash
#*****************************************************************************#
#                                                                             #
#  build-appimage.sh - AppImage builder                TTTTTTTT SSSSSSS II    #
#                                                         TT    SS      II    #
#  By: st93642@students.tsi.lv                            TT    SSSSSSS II    #
#                                                         TT         SS II    #
#  Created: Dec 07 2025 st93642                           TT    SSSSSSS II    #
#                                                                             #
#   Transport and Telecommunication Institute - Riga, Latvia                  #
#                       https://tsi.lv                                        #
#*****************************************************************************#

set -e

echo "========================================="
echo "VDownloader AppImage Builder"
echo "========================================="
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Check if binary exists
if [ ! -f "dist/vdownloader-linux" ]; then
    echo -e "${RED}Error: dist/vdownloader-linux not found${NC}"
    echo "Please run ./build.sh first"
    exit 1
fi

# Create directories
echo -e "${BLUE}Setting up AppDir structure...${NC}"
mkdir -p AppDir/usr/bin
mkdir -p AppDir/usr/share/applications
mkdir -p AppDir/usr/share/icons/hicolor/256x256/apps
mkdir -p AppDir/usr/share/icons/hicolor/scalable/apps

# Copy binary
echo -e "${BLUE}Copying binary...${NC}"
cp dist/vdownloader-linux AppDir/usr/bin/vdownloader
chmod +x AppDir/usr/bin/vdownloader

# Create desktop file
echo -e "${BLUE}Creating desktop file...${NC}"
cat > AppDir/usr/share/applications/vdownloader.desktop << 'EOF'
[Desktop Entry]
Type=Application
Name=VDownloader
Comment=Cross-platform video downloader
Exec=vdownloader
Icon=vdownloader
Categories=Network;AudioVideo;Video;
Terminal=false
StartupNotify=true
EOF

# Create icon (simple SVG)
echo -e "${BLUE}Creating application icon...${NC}"
cat > AppDir/usr/share/icons/hicolor/scalable/apps/vdownloader.svg << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<svg width="256" height="256" version="1.1" viewBox="0 0 256 256" xmlns="http://www.w3.org/2000/svg">
  <rect width="256" height="256" rx="32" fill="#4a90e2"/>
  <path d="m128 64v96l-32-32m32 32 32-32" fill="none" stroke="#fff" stroke-linecap="round" stroke-linejoin="round" stroke-width="16"/>
  <rect x="64" y="176" width="128" height="16" rx="4" fill="#fff"/>
</svg>
EOF

# Also create PNG version
if command -v convert &> /dev/null || command -v rsvg-convert &> /dev/null; then
    if command -v rsvg-convert &> /dev/null; then
        rsvg-convert -w 256 -h 256 AppDir/usr/share/icons/hicolor/scalable/apps/vdownloader.svg \
            -o AppDir/usr/share/icons/hicolor/256x256/apps/vdownloader.png
        echo -e "${GREEN}✓ PNG icon created${NC}"
    elif command -v convert &> /dev/null; then
        convert -background none -size 256x256 AppDir/usr/share/icons/hicolor/scalable/apps/vdownloader.svg \
            AppDir/usr/share/icons/hicolor/256x256/apps/vdownloader.png
        echo -e "${GREEN}✓ PNG icon created${NC}"
    fi
else
    echo -e "${YELLOW}⚠ ImageMagick or rsvg-convert not found, skipping PNG icon${NC}"
    # Create a simple placeholder
    cp AppDir/usr/share/icons/hicolor/scalable/apps/vdownloader.svg \
       AppDir/usr/share/icons/hicolor/256x256/apps/vdownloader.png 2>/dev/null || true
fi

# Copy icon to root for AppImage
cp AppDir/usr/share/icons/hicolor/scalable/apps/vdownloader.svg AppDir/vdownloader.svg
[ -f AppDir/usr/share/icons/hicolor/256x256/apps/vdownloader.png ] && \
    cp AppDir/usr/share/icons/hicolor/256x256/apps/vdownloader.png AppDir/vdownloader.png || \
    cp AppDir/vdownloader.svg AppDir/vdownloader.png

# Copy desktop file to root
cp AppDir/usr/share/applications/vdownloader.desktop AppDir/vdownloader.desktop

# Create AppRun script
echo -e "${BLUE}Creating AppRun script...${NC}"
cat > AppDir/AppRun << 'EOF'
#!/bin/bash
# AppRun script for VDownloader

SELF=$(readlink -f "$0")
HERE=${SELF%/*}

# Export environment for GTK
export PATH="${HERE}/usr/bin:${PATH}"
export LD_LIBRARY_PATH="${HERE}/usr/lib:${LD_LIBRARY_PATH}"
export XDG_DATA_DIRS="${HERE}/usr/share:${XDG_DATA_DIRS:-/usr/local/share:/usr/share}"
export GSETTINGS_SCHEMA_DIR="${HERE}/usr/share/glib-2.0/schemas:${GSETTINGS_SCHEMA_DIR}"

# Run the application
exec "${HERE}/usr/bin/vdownloader" "$@"
EOF

chmod +x AppDir/AppRun

echo ""
echo -e "${BLUE}Downloading linuxdeploy...${NC}"

# Download linuxdeploy if not present
if [ ! -f "linuxdeploy-x86_64.AppImage" ]; then
    wget -q --show-progress https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
    chmod +x linuxdeploy-x86_64.AppImage
    echo -e "${GREEN}✓ linuxdeploy downloaded${NC}"
else
    echo -e "${GREEN}✓ linuxdeploy already present${NC}"
fi

# Download GTK plugin if not present
if [ ! -f "linuxdeploy-plugin-gtk.sh" ]; then
    echo -e "${BLUE}Downloading GTK plugin...${NC}"
    wget -q --show-progress https://raw.githubusercontent.com/linuxdeploy/linuxdeploy-plugin-gtk/master/linuxdeploy-plugin-gtk.sh
    chmod +x linuxdeploy-plugin-gtk.sh
    echo -e "${GREEN}✓ GTK plugin downloaded${NC}"
else
    echo -e "${GREEN}✓ GTK plugin already present${NC}"
fi

echo ""
echo -e "${BLUE}Building AppImage...${NC}"
echo -e "${YELLOW}This may take several minutes as it bundles GTK4 and dependencies...${NC}"
echo ""

# Set environment for GTK plugin
export DEPLOY_GTK_VERSION=4

# Build AppImage
./linuxdeploy-x86_64.AppImage \
    --appdir AppDir \
    --plugin gtk \
    --output appimage \
    --desktop-file AppDir/usr/share/applications/vdownloader.desktop \
    --icon-file AppDir/usr/share/icons/hicolor/scalable/apps/vdownloader.svg

# Find the generated AppImage
APPIMAGE=$(ls VDownloader-*.AppImage 2>/dev/null | head -1)

if [ -f "$APPIMAGE" ]; then
    # Move to dist
    mkdir -p dist
    mv "$APPIMAGE" dist/VDownloader-x86_64.AppImage
    chmod +x dist/VDownloader-x86_64.AppImage
    
    echo ""
    echo -e "${GREEN}=========================================${NC}"
    echo -e "${GREEN}✓ AppImage built successfully!${NC}"
    echo -e "${GREEN}=========================================${NC}"
    echo ""
    echo -e "${BLUE}Output:${NC} dist/VDownloader-x86_64.AppImage"
    
    # Show size
    SIZE=$(du -h dist/VDownloader-x86_64.AppImage | cut -f1)
    echo -e "${BLUE}Size:${NC} $SIZE"
    echo ""
    echo -e "${BLUE}To run:${NC}"
    echo "  ./dist/VDownloader-x86_64.AppImage"
    echo ""
    echo -e "${BLUE}To install:${NC}"
    echo "  chmod +x dist/VDownloader-x86_64.AppImage"
    echo "  mv dist/VDownloader-x86_64.AppImage ~/.local/bin/VDownloader"
    echo ""
    echo -e "${GREEN}The AppImage is fully portable and includes all dependencies!${NC}"
else
    echo -e "${RED}Error: AppImage build failed${NC}"
    exit 1
fi
