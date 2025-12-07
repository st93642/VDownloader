# Windows Executable

This directory contains the Windows executable built by GitHub Actions.

- `vdownloader-windows.exe` - Automatically built and updated on every push to master

The executable is built with:

- Rust stable toolchain
- MSYS2 UCRT64 environment
- GTK4 and all required dependencies
- Target: x86_64-pc-windows-gnu

**Note**: The executable requires GTK4 runtime on Windows to run. Users should install MSYS2 and GTK4 as described in the documentation.
