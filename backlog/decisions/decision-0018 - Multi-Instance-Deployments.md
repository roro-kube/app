---
id: decision-0018
title: Multi-Instance Deployments
date: '2025-12-30 20:54'
status: accepted
---
## Context

Developers often need to run multiple instances of the same app for:
- Testing different configurations side-by-side
- Running different environments (dev, staging) simultaneously
- A/B testing or feature validation
- Isolated development environments per developer

Kubernetes supports this through namespaces and name suffixes, but it's not well-integrated into app management tools.

## Decision

Support multi-instance deployments where:
- Same app can be deployed multiple times with different configurations
- Each instance uses a different namespace (for isolation)
- Name suffixes distinguish instances (e.g., `my-app-dev`, `my-app-staging`)
- Environment-specific values can be applied per instance
- Instances are managed independently (start/stop separately)
- Port forwarding works per instance (different local ports)

This enables running the same app multiple times for different purposes.

## Consequences

**Positive:**
- Enables side-by-side testing and comparison
- Supports multiple environments simultaneously
- Isolated instances (namespace isolation)
- Flexible configuration per instance
- Useful for development workflows

**Negative:**
- More complex to manage multiple instances
- Need to handle namespace creation/cleanup
- Port conflicts need resolution (different local ports per instance)
- Resource usage (multiple instances running)
- Need clear instance identification in UI

**References:**
- doc-0004 (Talking Points) - Multi-Instance Deployments section
- doc-0002 (Features) - Key Features section