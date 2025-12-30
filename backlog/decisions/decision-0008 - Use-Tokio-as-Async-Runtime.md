---
id: decision-0008
title: Use Tokio as Async Runtime
date: '2025-12-30 20:54'
status: accepted
---
## Context

The application needs to handle many concurrent operations:
- Multiple Kubernetes API requests
- Git operations (clone, pull, sync)
- Port forwarding connections (multiple simultaneous forwards)
- UI event handling and updates
- File system watchers for configuration changes

Rust's async/await requires a runtime to execute async tasks. We need a runtime that:
- Supports high concurrency
- Integrates well with kube-rs and other async libraries
- Provides good performance
- Is the de-facto standard for Rust async

## Decision

Use Tokio as the async runtime for the entire application. Tokio provides:
- High-performance async I/O
- Task scheduling and execution
- Timer and time utilities
- File system async operations
- Network primitives
- Excellent ecosystem integration (kube-rs, Dioxus, etc.)

## Consequences

**Positive:**
- De-facto standard for Rust async - excellent ecosystem support
- High performance and scalability
- Well-maintained and battle-tested
- Integrates seamlessly with kube-rs, Dioxus, and other async libraries
- Rich set of utilities (timers, channels, etc.)
- Good documentation and community support

**Negative:**
- Larger dependency footprint
- Learning curve for async Rust concepts
- Some runtime overhead (though minimal)
- Need to understand Tokio's execution model

**References:**
- doc-0002 (Features) - Technology Stack section