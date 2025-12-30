---
id: decision-0015
title: JSONPath-Based CRD Value Extraction
date: '2025-12-30 20:54'
status: accepted
---
## Context

Custom Resource Definitions (CRDs) allow users to define arbitrary structures for storing configuration data (like environment variables and secrets). The application needs to extract values from these CRDs to inject into Kubernetes manifests, but the structure is user-defined and not known in advance.

We need a flexible extraction mechanism that:
- Works with any CRD structure
- Doesn't require hardcoding schemas
- Allows users to define extraction paths declaratively
- Supports nested structures and arrays

This is an architectural pattern for how CRD values are accessed throughout the system.

## Decision

Use JSONPath-based value extraction as the architectural pattern for accessing CRD values. This means:
- All CRD value access uses JSONPath expressions
- Users define JSONPath mappings in configuration (e.g., `APP_SECRET: ".spec.secrets.appSecret"`)
- The system provides a CRD resolver that takes JSONPath and returns values
- This pattern is used consistently across template rendering, environment variable injection, etc.

This is the architectural approach; the implementation uses the jsonpath crate (see decision-0007).

## Consequences

**Positive:**
- Maximum flexibility - supports any CRD structure
- Consistent pattern across the codebase
- Users have full control over value extraction
- No need to maintain CRD schema definitions
- Works with user-defined CRD structures

**Negative:**
- JSONPath expressions are strings (no compile-time validation)
- Need runtime validation of JSONPath expressions
- Errors only discovered when CRD is accessed
- JSONPath syntax learning curve for users

**References:**
- doc-0005 (Description) - CRD value resolution pattern
- Related: decision-0007 (Use JSONPath for CRD Value Extraction - implementation detail)