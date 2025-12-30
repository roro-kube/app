---
id: decision-0004
title: Use kube-rs crate for Kubernetes API
date: '2025-12-30 20:53'
status: accepted
---
## Context

The application needs to interact with Kubernetes clusters to:
- Apply and manage Kubernetes manifests
- Query cluster state (pods, services, deployments)
- Manage port forwarding connections
- Read Custom Resource Definitions (CRDs) for environment variables
- Support multiple cluster contexts

We need a Rust-native Kubernetes client library that integrates well with Tokio async runtime.

## Decision

Use the `kube-rs` crate (also known as `kube`) as the Kubernetes client library. This crate provides:
- Native Rust implementation
- Full Kubernetes API coverage
- Async/await support with Tokio
- Strongly-typed resource definitions
- Watch and informer patterns for real-time updates
- Port forwarding support

## Consequences

**Positive:**
- Native Rust implementation with excellent performance
- Strong type safety for Kubernetes resources
- Good async integration with Tokio
- Active development and maintenance
- Comprehensive API coverage
- Built-in support for CRDs and custom resources

**Negative:**
- Learning curve for Kubernetes API concepts
- Some advanced features may require lower-level API access
- Documentation can be sparse for edge cases

**References:**
- doc-0004 (Talking Points) - Technology Stack section
- doc-0002 (Features) - Technology Stack section