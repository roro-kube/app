---
id: decision-0017
title: Port Forwarding as First-Class Feature
date: '2025-12-30 20:54'
status: accepted
---
## Context

Local development requires access to services running in Kubernetes clusters. Port forwarding is the primary way to access services locally, but existing tools treat it as a secondary feature:
- kubectl port-forward is manual and doesn't persist
- Tools like Lens/K9s have port forwarding but it's not integrated with app lifecycle
- Port forwarding connections are unstable and need reconnection
- No automatic port forwarding based on app configuration

We need port forwarding to be a core, well-integrated feature.

## Decision

Make port forwarding a first-class feature where:
- Port forwarding is automatically configured from app definitions
- Connections are automatically established when apps start
- Stable connections with auto-reconnect on failure
- Lifecycle management - forwards start/stop with apps
- Multiple ports per app are supported
- Port forwarding status is visible in UI
- Configuration-driven (defined in app.json, not manual)

Port forwarding is not an afterthought - it's a core part of the app management experience.

## Consequences

**Positive:**
- Seamless local development experience
- No manual port-forward commands needed
- Automatic connection management
- Stable connections with reconnection logic
- Integrated with app lifecycle
- Multiple ports handled automatically

**Negative:**
- More complex to implement (connection management, reconnection logic)
- Need to handle port conflicts
- Resource usage (maintaining multiple connections)
- Need to clean up connections on app stop

**References:**
- doc-0004 (Talking Points) - Port Forwarding Management section
- doc-0002 (Features) - Key Differentiators section