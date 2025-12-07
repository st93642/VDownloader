@echo off
REM *****************************************************************************
REM                                                                             
REM  build.bat - Windows build script                TTTTTTTT SSSSSSS II        
REM                                                     TT    SS      II        
REM  By: st93642@students.tsi.lv                        TT    SSSSSSS II        
REM                                                     TT         SS II        
REM  Created: Dec 07 2025 st93642                       TT    SSSSSSS II        
REM                                                                             
REM   Transport and Telecommunication Institute - Riga, Latvia                  
REM                       https://tsi.lv                                        
REM *****************************************************************************

echo =========================================
echo VDownloader Windows Build Script
echo =========================================
echo.

REM Create dist directory
if not exist dist mkdir dist

echo Building for Windows (native)...
echo.

cargo build --release

if %ERRORLEVEL% EQU 0 (
    echo Build successful!
    echo.
    
    copy target\release\vdownloader.exe dist\vdownloader-windows.exe
    
    echo Binary copied to dist\vdownloader-windows.exe
    echo.
    
    dir dist\vdownloader-windows.exe
) else (
    echo Build failed!
    exit /b 1
)

echo.
echo =========================================
echo Build complete!
echo =========================================
echo.
echo Output: dist\vdownloader-windows.exe
echo.
echo Requirements for running:
echo   - GTK4 runtime (install from https://gtk.org)
echo   - yt-dlp (install from https://github.com/yt-dlp/yt-dlp)
echo.
