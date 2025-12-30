---
id: decision-0007
title: Use JSONPath for CRD Value Extraction
date: '2025-12-30 20:54'
status: accepted
---
## Context

Custom Resource Definitions (CRDs) can have arbitrary structures defined by users. We need to extract specific values from CRDs to inject as environment variables into Kubernetes manifests. The structure of CRDs is not known in advance and varies per user.

We need a flexible way to:
- Extract values from CRD objects of any structure
- Support nested paths and array access
- Allow users to define extraction paths without hardcoding structures
- Work with Kubernetes API response format (JSON)

Alternatives considered: Hardcoded structure (not flexible), XPath (XML-focused, not ideal for JSON), custom DSL (overkill).

## Decision

Use JSONPath expressions for extracting values from CRD objects. Specifically:
- Use a `jsonpath` crate for Rust that supports JSONPath syntax
- Allow users to define JSONPath expressions like `.spec.secrets.appSecret`
- Support standard JSONPath features (nested objects, arrays, filters)
- Map extracted values to environment variable names

## Consequences

**Positive:**
- Maximum flexibility - works with any CRD structure
- Standard JSONPath syntax (familiar to many developers)
- No need to hardcode CRD schemas
- Supports complex nested structures and arrays
- Industry-standard approach (used by kubectl, etc.)

**Negative:**
- Users need to learn JSONPath syntax
- JSONPath expressions can be error-prone if CRD structure changes
- Need to validate JSONPath expressions at runtime
- Some edge cases in JSONPath implementations may differ

**References:**
- doc-0004 (Talking Points) - CRD-Based Environment Variables section
- doc-0005 (Description) - CRD value resolution pattern