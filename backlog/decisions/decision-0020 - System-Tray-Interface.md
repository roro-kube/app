---
id: decision-0020
title: System Tray Interface
date: '2025-12-30 20:54'
status: accepted
---
## Context

Users need quick access to app status and controls without:
- Opening a full application window
- Using command-line tools
- Navigating complex UIs

System tray applications provide:
- Always-visible status indicator
- Quick access menu
- Minimal screen real estate
- Native OS integration

This is similar to Docker Desktop's tray icon experience.

## Decision

Implement a native system tray interface using Dioxus that provides:
- Tray icon showing overall app status
- Context menu with quick actions (start/stop apps, view status)
- Expand-to-window functionality for detailed management
- Status indicators (which apps are running, port forwarding status)
- Quick actions (start/stop individual apps)

The tray is the primary interface for quick status checks, with the full window for detailed management.

## Consequences

**Positive:**
- Always accessible without taking up screen space
- Quick status at a glance
- Native OS integration feels natural
- Minimal resource usage when minimized
- Familiar pattern from other desktop apps (Docker Desktop, etc.)

**Negative:**
- Platform-specific implementation (Windows, macOS, Linux differ)
- Limited space for information in tray menu
- Need to handle tray icon updates and state
- Platform-specific APIs and behaviors

**References:**
- doc-0004 (Talking Points) - System Tray Interface section
- doc-0002 (Features) - Key Features section