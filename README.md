# Keyboard Lock

A lightweight, high-performance Rust application that globally disables keyboard input with a single click. Designed to be minimal, secure, and extremely small.

## Features

- **Global Keyboard Intercept:** Uses a low-level Win32 keyboard hook (`WH_KEYBOARD_LL`) to swallow all keyboard events.
- **Always on Top:** The UI remains visible above other windows so you can always unlock your keyboard using the mouse.
- **Stealth Mode:** Launched as a Windows GUI application (no command prompt window).
- **Extremely Optimized:** Built with size-optimized compilation flags and compressed backend for a minimal footprint.

## Requirements

- **Windows OS** (64-bit recommended)
- **Rust Toolchain** (to build from source)

## Building and Optimizing

The project is configured for maximum binary size reduction. To build the standard release version:

```powershell
cargo build --release
```

### Deep Optimization & Compression

A helper script `compress.ps1` is provided to generate the smallest possible executable (typically under 600KB). It automates the following:
1. Performs a full release build.
2. Downloads a portable version of **UPX** (Ultimate Packer for eXecutables).
3. Compresses the binary using the `--lzma` algorithm.
4. Cleans up temporary tools.

**To run the compression script:**
```powershell
.\compress.ps1
```
*Note: You may need to bypass PowerShell execution policy if it's your first time running a local script: `Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass`*

## Technical Details

- **GUI Framework:** [egui](https://github.com/emilk/egui) (via `eframe`) using the `glow` (OpenGL) backend for size efficiency.
- **System API:** [windows-rs](https://github.com/microsoft/windows-rs) for direct Win32 integration.
- **Safety:** The `Ctrl+Alt+Del` sequence is handled by the Windows kernel and remains functional as a safety fallback.

## License

MIT
