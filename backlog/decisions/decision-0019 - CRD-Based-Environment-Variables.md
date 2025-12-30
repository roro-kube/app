---
id: decision-0019
title: CRD-Based Environment Variables
date: '2025-12-30 20:54'
status: accepted
---
## Context

Applications need environment variables and configuration values that:
- Vary by environment (dev, staging, prod)
- May contain secrets or tokens
- Should be managed in Kubernetes (not hardcoded)
- Need to be injected into manifests at deployment time
- Should support user-defined structures

Hardcoding values in manifests doesn't scale. Using ConfigMaps/Secrets works but requires managing many resources. CRDs provide a flexible way to store environment-specific configuration.

## Decision

Use Custom Resource Definitions (CRDs) as the source for environment variables and configuration values. Specifically:
- Users define CRDs with their own structure
- Values are extracted from CRDs using JSONPath expressions
- Values are injected into Kubernetes manifests during template rendering
- Supports multiple environments (different CRD instances per environment)
- Non-secure tokens and environment variables come from CRDs
- Flexible structure - users define the CRD schema

This provides a flexible, Kubernetes-native way to manage environment-specific configuration.

## Consequences

**Positive:**
- Flexible - users define their own CRD structure
- Kubernetes-native approach
- Supports multiple environments
- Values stored in cluster (not hardcoded)
- Can be managed via GitOps or kubectl
- Works with existing Kubernetes tooling

**Negative:**
- Requires users to define and manage CRDs
- JSONPath expressions need to be correct
- Need to handle CRD availability (what if CRD doesn't exist?)
- Learning curve for users new to CRDs
- Not suitable for highly sensitive secrets (use Secrets instead)

**References:**
- doc-0004 (Talking Points) - CRD-Based Environment Variables section
- doc-0002 (Features) - Key Features section
- Related: decision-0007, decision-0015 (JSONPath extraction)