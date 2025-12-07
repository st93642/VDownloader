#!/bin/bash
#*****************************************************************************#
#                                                                             #
#  build.sh - Cross-platform build script             TTTTTTTT SSSSSSS II     #
#                                                        TT    SS      II     #
#  By: st93642@students.tsi.lv                           TT    SSSSSSS II     #
#                                                        TT         SS II     #
#  Created: Dec 07 2025 st93642                          TT    SSSSSSS II     #
#                                                                             #
#   Transport and Telecommunication Institute - Riga, Latvia                  #
#                       https://tsi.lv                                        #
#*****************************************************************************#

set -e

echo "========================================="
echo "VDownloader Multi-Platform Build Script"
echo "========================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Create dist directory
mkdir -p dist

# Function to build for a target
build_target() {
    local target=$1
    local platform=$2
    local ext=$3
    
    echo -e "${BLUE}Building for ${platform}...${NC}"
    
    if cargo build --release --target "$target" 2>&1; then
        echo -e "${GREEN}✓ Build successful for ${platform}${NC}"
        
        # Copy binary to dist folder with appropriate name
        local binary_name="vdownloader${ext}"
        local dist_name="vdownloader-${platform}${ext}"
        
        if [ -f "target/${target}/release/${binary_name}" ]; then
            cp "target/${target}/release/${binary_name}" "dist/${dist_name}"
            echo -e "${GREEN}✓ Binary copied to dist/${dist_name}${NC}"
            
            # Get file size
            local size=$(du -h "dist/${dist_name}" | cut -f1)
            echo -e "  Size: ${size}"
        else
            echo -e "${RED}✗ Binary not found at expected location${NC}"
        fi
    else
        echo -e "${RED}✗ Build failed for ${platform}${NC}"
        return 1
    fi
    
    echo ""
}

# Build for Linux (native)
echo -e "${BLUE}=== Building for Linux (native) ===${NC}"
if cargo build --release; then
    echo -e "${GREEN}✓ Linux build successful${NC}"
    cp target/release/vdownloader dist/vdownloader-linux
    chmod +x dist/vdownloader-linux
    size=$(du -h dist/vdownloader-linux | cut -f1)
    echo -e "  Size: ${size}"
    echo ""
else
    echo -e "${RED}✗ Linux build failed${NC}"
    exit 1
fi

# Check for cross-compilation tools
echo -e "${BLUE}Checking for cross-compilation tools...${NC}"

# For Windows cross-compilation
if command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo -e "${GREEN}✓ MinGW found - Windows cross-compilation available${NC}"
    HAS_MINGW=1
else
    echo -e "${RED}✗ MinGW not found - Windows cross-compilation unavailable${NC}"
    echo "  Install with: sudo apt-get install mingw-w64"
    HAS_MINGW=0
fi

# For macOS cross-compilation (osxcross)
if command -v x86_64-apple-darwin20.2-clang &> /dev/null || command -v o64-clang &> /dev/null; then
    echo -e "${GREEN}✓ OSXCross found - macOS cross-compilation available${NC}"
    HAS_OSXCROSS=1
else
    echo -e "${RED}✗ OSXCross not found - macOS cross-compilation unavailable${NC}"
    echo "  OSXCross setup required for macOS builds"
    HAS_OSXCROSS=0
fi

echo ""

# Add Windows target if MinGW is available
if [ $HAS_MINGW -eq 1 ]; then
    echo -e "${BLUE}Adding Windows target...${NC}"
    rustup target add x86_64-pc-windows-gnu 2>/dev/null || echo "Target already added"
    echo ""
    
    echo -e "${BLUE}=== Building for Windows ===${NC}"
    build_target "x86_64-pc-windows-gnu" "windows" ".exe" || echo "Windows build skipped"
else
    echo -e "${RED}Skipping Windows build - MinGW not available${NC}"
    echo ""
fi

# macOS builds are complex due to GTK dependencies
echo -e "${BLUE}=== macOS Builds ===${NC}"
echo -e "${RED}Note: macOS builds with GTK require native macOS environment${NC}"
echo "Cross-compiling GTK applications to macOS from Linux is not straightforward."
echo "For macOS builds, please build natively on a Mac using: cargo build --release"
echo ""

# Summary
echo "========================================="
echo -e "${GREEN}Build Summary${NC}"
echo "========================================="
echo ""
ls -lh dist/ 2>/dev/null || echo "No builds in dist/"
echo ""
echo -e "${BLUE}Native builds location:${NC}"
echo "  Linux:   dist/vdownloader-linux"
if [ $HAS_MINGW -eq 1 ] && [ -f dist/vdownloader-windows.exe ]; then
    echo "  Windows: dist/vdownloader-windows.exe"
fi
echo ""
echo -e "${BLUE}Additional requirements:${NC}"
echo "  • yt-dlp must be installed on target system"
echo "  • GTK4 runtime must be available on target system"
echo ""
echo -e "${GREEN}Build process complete!${NC}"
