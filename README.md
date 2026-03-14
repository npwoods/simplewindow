# simplewindow

A small cross-platform Rust project that opens and displays a single native window.  This is intended to be a simple debugging tool for applications that can attach themselves to windows (such as MAME with the `-attach_window` argument)

Features
- Minimal example for creating a native window using winit and raw-window-handle
- Cross-platform focus (Windows, Linux/X11)
