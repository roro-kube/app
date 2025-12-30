---
id: decision-0003
title: Use Rust for Backend
date: '2025-12-30 20:53'
status: accepted
---
## Context

The application needs a systems programming language that can:
- Interact efficiently with Kubernetes APIs
- Handle async I/O for Git operations, network requests, and port forwarding
- Provide memory safety without garbage collection overhead
- Support cross-platform compilation
- Integrate well with the chosen UI framework (Dioxus)

Alternatives considered: Go (good K8s ecosystem but different language), Python (too slow, not suitable for desktop app), Node.js (not systems-level enough).

## Decision

Use Rust as the primary backend language for all application logic. Rust provides:
- Excellent async runtime (Tokio) for concurrent operations
- Strong type system and memory safety
- Great performance characteristics
- Strong ecosystem for Kubernetes (kube-rs) and Git operations
- Seamless integration with Dioxus UI framework

## Consequences

**Positive:**
- Single language across entire stack (UI and backend)
- Excellent performance and memory efficiency
- Strong type safety catches errors at compile time
- Great async/await support for concurrent operations
- Growing ecosystem for Kubernetes and systems programming

**Negative:**
- Steeper learning curve compared to higher-level languages
- Longer compile times
- Some ecosystem libraries less mature than Go/Node.js equivalents
- Requires understanding ownership and borrowing concepts

**References:**
- doc-0004 (Talking Points) - Technology Stack section
- doc-0002 (Features) - Technology Stack section