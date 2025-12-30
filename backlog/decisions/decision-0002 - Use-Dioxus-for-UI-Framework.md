---
id: decision-0002
title: Use Dioxus for UI Framework
date: '2025-12-30 20:53'
status: accepted
---
## Context

We need a cross-platform desktop UI framework for the Kubernetes app manager. The application requires:
- Native system tray integration
- Cross-platform support (Windows, macOS, Linux)
- Ability to expand from tray to full window
- Rust-based to match the backend language
- Good performance for real-time status updates

Alternatives considered: Tauri (web-based), egui (immediate mode), Electron (web-based, not Rust-native).

## Decision

Use Dioxus as the UI framework for the desktop application. Dioxus provides:
- Rust-based framework with React-like component model
- Native system tray support via platform-specific integrations
- Cross-platform desktop targets
- Good integration with Rust ecosystem and async runtime (Tokio)
- Type safety across UI and backend code

## Consequences

**Positive:**
- Single language (Rust) for entire application stack
- Type safety across UI and backend boundaries
- Good performance with native rendering (no web engine overhead)
- Active development and growing community
- React-like patterns familiar to many developers

**Negative:**
- Smaller ecosystem compared to Electron/web frameworks
- Learning curve for React-like patterns in Rust
- Less mature than established frameworks like Tauri
- Fewer third-party UI components available

**References:**
- doc-0004 (Talking Points) - Technology Stack section
- doc-0002 (Features) - Technology Stack section