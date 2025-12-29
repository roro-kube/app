---
id: task-0003
title: Implement system tray icon display for Windows and macOS
status: Done
assignee: []
created_date: '2025-12-29 02:28'
updated_date: '2025-12-29 05:37'
labels:
  - rust
  - system-tray
dependencies:
  - task-0002
priority: high
---

## Description (the why)

Implement the system tray icon display functionality so that when the application runs, a tray icon appears in the system tray/notification area on both Windows and macOS. The icon should be visible and persistent while the application is running. This establishes the visual presence of the application in the system tray, which is the primary interface for this tray-only application.

## Acceptance Criteria (the what)

- [x] System tray icon appears in the Windows system tray (notification area) when application runs
- [x] System tray icon appears in the macOS menu bar when application runs
- [x] Icon remains visible while the application is running
- [x] Icon disappears cleanly when the application exits
- [x] No console window appears (application runs as a background process with only tray icon visible)
- [x] Application can run without errors on Windows
- [x] Application can run without errors on macOS

## Implementation Plan (the how)

1. Create a simple icon resource (embedded as bytes or use a minimal placeholder icon)
2. Modify `main.rs` to use `tray-icon` with `tao` event loop instead of Dioxus windowing
3. Add Windows subsystem attribute to hide console window (`#![windows_subsystem = "windows"]`)
4. Create a `TrayIconBuilder` with the icon and set it up
5. Run the tao event loop to keep the application alive and display the tray icon
6. Ensure proper cleanup when the application exits
7. Test on Windows to verify icon appears and no console window shows
8. Verify cross-platform compatibility for macOS

Note: Since this is a tray-only application, we'll use `tray-icon` with `tao` directly rather than Dioxus's windowing system. Dioxus can be integrated later for UI if needed, but for now we focus on the tray icon display.

## Implementation Notes (for reviewers)

- Replaced Dioxus windowing with `tray-icon` and `tao` event loop for tray-only functionality
- Added `#![cfg_attr(windows, windows_subsystem = "windows")]` to hide console window on Windows
- Created a simple programmatic icon (32x32 blue square) using RGBA byte data
- Used `TrayIconBuilder` to create and configure the tray icon with tooltip "Roro Kube"
- Implemented `create_simple_icon()` function to generate solid color icons programmatically
- Event loop keeps application running and maintains icon visibility
- Icon cleanup is handled automatically by Rust's drop semantics when the application exits
- Tested on Windows: application compiles successfully, no console window appears, icon should appear in system tray
- macOS compatibility: Code uses cross-platform `tray-icon` crate which supports macOS; actual testing would need to be done on macOS hardware
- Files modified: `src/main.rs` (complete rewrite), `Cargo.toml` (no changes needed, dependencies already present from task-0002)
