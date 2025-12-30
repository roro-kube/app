---
id: decision-0006
title: Use Handlebars/Tera for Template Rendering
date: '2025-12-30 20:54'
status: accepted
---
## Context

Kubernetes manifests need to be templated to support:
- Environment-specific values (dev, staging, prod)
- Variable injection from static config, environment variables, and CRDs
- Dynamic namespace and name generation
- Multi-instance deployments with different configurations

We need template engines that are:
- Rust-native (no FFI overhead)
- Support variable injection and conditionals
- Familiar syntax for users (similar to Helm templates)
- Performant for rendering multiple manifests

## Decision

Support both Handlebars and Tera template engines for Kubernetes manifest rendering. This provides:
- **Handlebars**: Mustache-like syntax, familiar to many developers
- **Tera**: Jinja2-like syntax, powerful templating features
- User choice based on preference and complexity needs
- Both are pure Rust implementations with good performance

## Consequences

**Positive:**
- Flexibility for users to choose their preferred syntax
- Both engines are mature and well-maintained
- Good performance with native Rust implementations
- Familiar syntax reduces learning curve
- Supports complex templating needs

**Negative:**
- Need to support and test two different template engines
- Users need to choose which one to use (decision overhead)
- Slightly more complex codebase to support both
- Different syntaxes may confuse users switching between them

**References:**
- doc-0004 (Talking Points) - Template Rendering section
- doc-0005 (Description) - Manifest template configuration