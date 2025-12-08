#!/bin/bash
# *****************************************************************************
#                                                                             
#  check-dll-deps.sh - Check DLL dependencies                TTTTTTTT SSSSSSS II  
#                                                                TT    SS      II  
#  By: st93642@students.tsi.lv                                   TT    SSSSSSS II  
#                                                                TT         SS II  
#  Created: Dec 08 2025 st93642                                  TT    SSSSSSS II  
#                                                                             
#   Transport and Telecommunication Institute - Riga, Latvia                  
#                       https://tsi.lv                                        
# *****************************************************************************

echo "========================================="
echo "VDownloader DLL Dependency Checker"
echo "========================================="
echo

if [ ! -f "target/x86_64-pc-windows-gnu/release/vdownloader.exe" ]; then
    echo "Error: vdownloader.exe not found!"
    echo "Please build the project first with: cargo build --release --target x86_64-pc-windows-gnu"
    exit 1
fi

echo "Analyzing dependencies of vdownloader.exe..."
echo

# Get all dependencies
DEPS=$(ldd target/x86_64-pc-windows-gnu/release/vdownloader.exe 2>/dev/null | grep -i "ucrt64\|mingw64" | awk '{print $3}' | sort -u)

if [ -z "$DEPS" ]; then
    echo "Error: Could not analyze dependencies. Make sure you're running this in MSYS2."
    exit 1
fi

echo "Found dependencies:"
echo "-------------------"
count=0
for dll in $DEPS; do
    basename "$dll"
    count=$((count + 1))
done

echo
echo "Total: $count DLLs"
echo
echo "Checking which DLLs are missing from build script..."
echo "----------------------------------------------------"

# DLLs currently in the build script
KNOWN_DLLS="libgtk-4-1.dll libgdk_pixbuf-2.0-0.dll libgio-2.0-0.dll libglib-2.0-0.dll \
libgobject-2.0-0.dll libgmodule-2.0-0.dll libcairo-2.dll libcairo-gobject-2.dll \
libpango-1.0-0.dll libpangocairo-1.0-0.dll libpangoft2-1.0-0.dll libpangowin32-1.0-0.dll \
libharfbuzz-0.dll libfontconfig-1.dll libfreetype-6.dll libpng16-16.dll libepoxy-0.dll \
libfribidi-0.dll libgraphene-1.0-0.dll libjpeg-8.dll libtiff-6.dll libintl-8.dll \
libpcre2-8-0.dll libiconv-2.dll libffi-8.dll libwinpthread-1.dll zlib1.dll libbz2-1.dll \
libexpat-1.dll libbrotlicommon.dll libbrotlidec.dll libpixman-1-0.dll libgcc_s_seh-1.dll \
libstdc++-6.dll libdatrie-1.dll libthai-0.dll libdeflate.dll libsharpyuv-0.dll libwebp-7.dll \
liblzma-5.dll libzstd.dll libLerc.dll libjbig-0.dll libhwy.dll"

missing_count=0
for dll in $DEPS; do
    dll_name=$(basename "$dll")
    if ! echo "$KNOWN_DLLS" | grep -q "$dll_name"; then
        echo "  MISSING: $dll_name"
        missing_count=$((missing_count + 1))
    fi
done

if [ $missing_count -eq 0 ]; then
    echo "  All dependencies are included in the build script!"
else
    echo
    echo "Found $missing_count missing DLLs. Please add them to the build script."
fi

echo
echo "To copy all dependencies automatically:"
echo "  cd dist/windows-portable"
echo "  ldd vdownloader.exe | grep -i 'ucrt64\\|mingw64' | awk '{print \$3}' | xargs -I {} cp {} ."
echo
