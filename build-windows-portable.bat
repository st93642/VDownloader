@echo off
REM *****************************************************************************
REM                                                                             
REM  build-windows-portable.bat - Portable Windows build  TTTTTTTT SSSSSSS II  
REM                                                           TT    SS      II  
REM  By: st93642@students.tsi.lv                              TT    SSSSSSS II  
REM                                                           TT         SS II  
REM  Created: Dec 08 2025 st93642                             TT    SSSSSSS II  
REM                                                                             
REM   Transport and Telecommunication Institute - Riga, Latvia                  
REM                       https://tsi.lv                                        
REM *****************************************************************************

echo =========================================
echo VDownloader Portable Windows Builder
echo =========================================
echo.

REM Build the executable
echo Building release executable...
cargo build --release

if %ERRORLEVEL% NEQ 0 (
    echo Build failed!
    exit /b 1
)

REM Create portable directory structure
echo.
echo Creating portable package structure...
mkdir dist\windows-portable 2>nul
mkdir dist\windows-portable\bin 2>nul
mkdir dist\windows-portable\lib 2>nul
mkdir dist\windows-portable\share 2>nul

REM Copy executable
echo Copying executable...
copy target\release\vdownloader.exe dist\windows-portable\vdownloader.exe

REM Find MSYS2 GTK4 path (common locations)
set "GTK_PATH="
if exist "C:\msys64\ucrt64" set "GTK_PATH=C:\msys64\ucrt64"
if exist "C:\msys64\mingw64" set "GTK_PATH=C:\msys64\mingw64"

if not defined GTK_PATH (
    echo ERROR: MSYS2 GTK4 not found!
    echo Please install MSYS2 and GTK4: pacman -S mingw-w64-ucrt-x86_64-gtk4
    exit /b 1
)

echo Using GTK4 from: %GTK_PATH%
echo.

REM Copy required DLLs
echo Copying GTK4 and dependencies...
for %%F in (
    libgtk-4-1.dll
    libgdk_pixbuf-2.0-0.dll
    libgio-2.0-0.dll
    libglib-2.0-0.dll
    libgobject-2.0-0.dll
    libgmodule-2.0-0.dll
    libcairo-2.dll
    libcairo-gobject-2.dll
    libpango-1.0-0.dll
    libpangocairo-1.0-0.dll
    libpangoft2-1.0-0.dll
    libpangowin32-1.0-0.dll
    libharfbuzz-0.dll
    libfontconfig-1.dll
    libfreetype-6.dll
    libpng16-16.dll
    libepoxy-0.dll
    libfribidi-0.dll
    libgraphene-1.0-0.dll
    libjpeg-8.dll
    libtiff-6.dll
    libintl-8.dll
    libpcre2-8-0.dll
    libiconv-2.dll
    libffi-8.dll
    libwinpthread-1.dll
    zlib1.dll
    libbz2-1.dll
    libexpat-1.dll
    libbrotlicommon.dll
    libbrotlidec.dll
    libpixman-1-0.dll
) do (
    if exist "%GTK_PATH%\bin\%%F" (
        copy "%GTK_PATH%\bin\%%F" dist\windows-portable\ >nul 2>&1
    )
)

REM Copy GDK pixbuf loaders
echo Copying GDK pixbuf loaders...
if exist "%GTK_PATH%\lib\gdk-pixbuf-2.0" (
    xcopy /E /I /Y "%GTK_PATH%\lib\gdk-pixbuf-2.0" dist\windows-portable\lib\gdk-pixbuf-2.0 >nul
)

REM Copy GTK modules and data
echo Copying GTK4 modules...
if exist "%GTK_PATH%\lib\gtk-4.0" (
    xcopy /E /I /Y "%GTK_PATH%\lib\gtk-4.0" dist\windows-portable\lib\gtk-4.0 >nul
)

REM Copy icon themes and schemas
echo Copying GTK4 data files...
if exist "%GTK_PATH%\share\icons" (
    xcopy /E /I /Y "%GTK_PATH%\share\icons\hicolor" dist\windows-portable\share\icons\hicolor >nul
    xcopy /E /I /Y "%GTK_PATH%\share\icons\Adwaita" dist\windows-portable\share\icons\Adwaita >nul 2>nul
)

if exist "%GTK_PATH%\share\glib-2.0" (
    xcopy /E /I /Y "%GTK_PATH%\share\glib-2.0\schemas" dist\windows-portable\share\glib-2.0\schemas >nul
)

REM Compile GSettings schemas
echo Compiling schemas...
if exist "%GTK_PATH%\bin\glib-compile-schemas.exe" (
    "%GTK_PATH%\bin\glib-compile-schemas.exe" dist\windows-portable\share\glib-2.0\schemas
)

REM Create launcher script
echo Creating launcher...
echo @echo off > dist\windows-portable\VDownloader.bat
echo set "DIR=%%~dp0" >> dist\windows-portable\VDownloader.bat
echo set "PATH=%%DIR%%;%%PATH%%" >> dist\windows-portable\VDownloader.bat
echo set "GDK_PIXBUF_MODULE_FILE=%%DIR%%\lib\gdk-pixbuf-2.0\2.10.0\loaders.cache" >> dist\windows-portable\VDownloader.bat
echo set "GTK_DATA_PREFIX=%%DIR%%" >> dist\windows-portable\VDownloader.bat
echo set "XDG_DATA_DIRS=%%DIR%%\share;%%XDG_DATA_DIRS%%" >> dist\windows-portable\VDownloader.bat
echo start "" "%%DIR%%\vdownloader.exe" %%* >> dist\windows-portable\VDownloader.bat

REM Create README
echo Creating README...
(
echo VDownloader - Portable Windows Version
echo ========================================
echo.
echo This is a portable version that includes all dependencies.
echo.
echo To run:
echo   1. Double-click VDownloader.bat
echo   OR
echo   2. Double-click vdownloader.exe directly
echo.
echo Requirements:
echo   - yt-dlp ^(download from https://github.com/yt-dlp/yt-dlp^)
echo   - Place yt-dlp.exe in the same folder or add to PATH
echo.
echo The package includes:
echo   - VDownloader executable
echo   - All GTK4 runtime libraries
echo   - Icon themes and schemas
echo.
echo Total size: ~100-150 MB
echo.
) > dist\windows-portable\README.txt

REM Create zip package
echo.
echo Creating ZIP package...
if exist "C:\Program Files\7-Zip\7z.exe" (
    cd dist
    "C:\Program Files\7-Zip\7z.exe" a -tzip vdownloader-windows-portable.zip windows-portable
    cd ..
    echo.
    echo =========================================
    echo Portable package created successfully!
    echo =========================================
    echo.
    echo Package: dist\vdownloader-windows-portable.zip
    dir dist\vdownloader-windows-portable.zip
) else (
    echo.
    echo =========================================
    echo Portable folder created successfully!
    echo =========================================
    echo.
    echo Location: dist\windows-portable\
    echo.
    echo Note: 7-Zip not found. Package not zipped.
    echo You can manually zip the windows-portable folder.
)

echo.
echo To test: Run dist\windows-portable\VDownloader.bat
echo.
