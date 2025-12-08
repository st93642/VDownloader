# Windows Portable Build - Troubleshooting

## Common "Missing DLL" Errors

If you get errors about missing DLLs when running the portable build, this guide will help you fix them.

### Error Types

#### 1. "The code execution cannot continue because [DLL] was not found"

This means a required library is missing from the portable package.

**Solution:**
- Download the missing DLL from the MSYS2 UCRT64 environment
- Place it in the same folder as `vdownloader.exe`
- Common missing DLLs and where to find them:

```
libgcc_s_seh-1.dll      - C:\msys64\ucrt64\bin\
libstdc++-6.dll         - C:\msys64\ucrt64\bin\
libwinpthread-1.dll     - C:\msys64\ucrt64\bin\
```

#### 2. "Application failed to initialize properly"

This usually means multiple DLL dependencies are missing.

**Solution:**
1. Use the automatic dependency checker (if building locally)
2. Or download a fresh build from GitHub Actions

### Rebuilding Locally with All Dependencies

If you're building the portable version yourself:

```bash
# In MSYS2 UCRT64 terminal
cd /path/to/VDownloader

# Run the dependency checker
./scripts/check-dll-deps.sh

# It will show you which DLLs are missing
# Then run the build script
./build-windows-portable.bat
```

### Manual Dependency Copy

If you want to manually ensure all dependencies are included:

```bash
# In MSYS2 UCRT64 terminal
cd dist/windows-portable

# Copy ALL dependencies automatically
ldd vdownloader.exe | grep -i 'ucrt64\|mingw64' | awk '{print $3}' | xargs -I {} cp {} .

# Verify
ls -lh *.dll | wc -l
# Should show 50+ DLLs
```

### Complete DLL List

The portable build should include these DLLs at minimum:

**GTK4 Core:**
- libgtk-4-1.dll
- libgdk-3-0.dll (if GTK3 fallback is used)
- libgdk_pixbuf-2.0-0.dll
- libepoxy-0.dll
- libgraphene-1.0-0.dll

**GLib Ecosystem:**
- libglib-2.0-0.dll
- libgobject-2.0-0.dll
- libgio-2.0-0.dll
- libgmodule-2.0-0.dll

**Text Rendering:**
- libpango-1.0-0.dll
- libpangocairo-1.0-0.dll
- libpangoft2-1.0-0.dll
- libpangowin32-1.0-0.dll
- libharfbuzz-0.dll
- libfontconfig-1.dll
- libfreetype-6.dll
- libfribidi-0.dll
- libdatrie-1.dll
- libthai-0.dll

**Graphics:**
- libcairo-2.dll
- libcairo-gobject-2.dll
- libpixman-1-0.dll
- librsvg-2-2.dll (for SVG icons)

**Image Formats:**
- libpng16-16.dll
- libjpeg-8.dll
- libtiff-6.dll
- libwebp-7.dll
- libsharpyuv-0.dll

**Compression:**
- zlib1.dll
- libbz2-1.dll
- liblzma-5.dll
- libzstd.dll
- libdeflate.dll
- libbrotlicommon.dll
- libbrotlidec.dll

**Utilities:**
- libintl-8.dll (internationalization)
- libpcre2-8-0.dll (regex)
- libiconv-2.dll (character encoding)
- libffi-8.dll (foreign function interface)
- libexpat-1.dll (XML parsing)
- libxml2-2.dll (XML parsing)

**C++ Runtime:**
- libgcc_s_seh-1.dll
- libstdc++-6.dll
- libwinpthread-1.dll

**Image Processing:**
- libLerc.dll
- libjbig-0.dll
- libhwy.dll

### Testing the Portable Build

To verify your portable build has all dependencies:

1. **Copy to a clean system** (or VM without MSYS2)
2. **Run VDownloader.bat**
3. If it starts without errors, you're good!

### Using Dependency Walker

For advanced debugging, use **Dependency Walker**:

1. Download from: https://www.dependencywalker.com/
2. Open `vdownloader.exe` in Dependency Walker
3. It will show all missing DLLs in red
4. Copy those DLLs from `C:\msys64\ucrt64\bin\`

### GitHub Actions Build

The GitHub Actions build should automatically include all dependencies because it:
1. Analyzes `vdownloader.exe` with `ldd`
2. Copies all UCRT64 dependencies found
3. Adds additional known GTK4 libraries

If the GitHub Actions build is missing DLLs:
1. Check the workflow logs for errors
2. Run the dependency checker locally
3. Update the workflow with missing DLLs

### Environment Variables

The `VDownloader.bat` launcher sets these environment variables:

```bat
PATH=%DIR%;%PATH%
GDK_PIXBUF_MODULE_FILE=%DIR%\lib\gdk-pixbuf-2.0\2.10.0\loaders.cache
GTK_DATA_PREFIX=%DIR%
XDG_DATA_DIRS=%DIR%\share;%XDG_DATA_DIRS%
```

Make sure these are set when running directly from `vdownloader.exe`.

### Still Having Issues?

If you still get errors after following this guide:

1. **Check the error message** - Note the exact DLL name
2. **Search in MSYS2**: `find /ucrt64 -name "missing-dll-name"`
3. **Copy it**: `cp /ucrt64/bin/missing-dll.dll dist/windows-portable/`
4. **Report it** - Open an issue so we can add it to the build script

### Getting the Latest Build

Always download the latest portable build from:
- GitHub repository: `dist/vdownloader-windows-portable.zip`
- GitHub Actions artifacts (most recent)
- Release page (stable versions)

The automated builds are more reliable than manual builds because they use a clean MSYS2 environment.

### Quick Fix Script

Save this as `fix-deps.bat` in the portable folder:

```bat
@echo off
echo Copying missing dependencies from MSYS2...
set "MSYS_BIN=C:\msys64\ucrt64\bin"

if not exist "%MSYS_BIN%" (
    echo Error: MSYS2 not found at %MSYS_BIN%
    exit /b 1
)

for %%F in (
    libgcc_s_seh-1.dll libstdc++-6.dll libwinpthread-1.dll
    libdatrie-1.dll libthai-0.dll librsvg-2-2.dll libxml2-2.dll
) do (
    if not exist "%%F" (
        if exist "%MSYS_BIN%\%%F" (
            copy "%MSYS_BIN%\%%F" .
            echo Copied %%F
        )
    )
)

echo Done!
pause
```

Run this script in the portable folder if you have MSYS2 installed.
