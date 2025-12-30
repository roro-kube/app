---
id: decision-0016
title: App-Centric Model
date: '2025-12-30 20:54'
status: accepted
---
## Context

Kubernetes is resource-centric - users work with individual resources (Deployments, Services, ConfigMaps, etc.). However, applications are logical groupings of related resources that work together. Managing individual resources is tedious and doesn't match how developers think about applications.

Tools like Docker Desktop group related containers into "apps" which is more intuitive. We need a similar model for Kubernetes.

## Decision

Adopt an app-centric model where:
- An "app" is a logical grouping of related Kubernetes resources (deployments, services, configmaps, etc.)
- Users manage apps, not individual resources
- Each app has its own configuration, manifests, and lifecycle
- Apps can be started, stopped, and managed as a unit
- Similar to Docker Desktop's grouped applications

This is the primary abstraction users interact with, hiding the complexity of individual Kubernetes resources.

## Consequences

**Positive:**
- More intuitive mental model - matches how developers think
- Easier to manage - one app instead of many resources
- Clear boundaries - each app is self-contained
- Better organization - related resources grouped together
- Familiar pattern from Docker Desktop

**Negative:**
- Need to define what constitutes an "app"
- Some resources might not fit neatly into apps
- Need to handle app-level operations (start/stop all resources)
- Abstraction hides Kubernetes details (can be good or bad)

**References:**
- doc-0004 (Talking Points) - App Grouping section
- doc-0002 (Features) - Key Differentiators section