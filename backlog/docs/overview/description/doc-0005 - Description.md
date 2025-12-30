---
id: doc-0005
title: Description
type: overview
created_date: '2025-12-30 21:01'
updated_date: '2025-12-30 21:01'
---

# Roro Kube - Original Design Document

## Overview

This document contains the original design requirements and proposals for Roro Kube, a "Docker Compose for Kubernetes" desktop application.

## Original Requirements

The application should:
- Follow standards set by Docker Compose for creating "apps" (like grouped apps in Docker Desktop)
- Each app is a kubectl manifest (templated like Helm charts)
- Define how apps connect and establish port forwarding to services
- Store all configuration in Git as JSON files
- Store manifests in relative directories to applications
- Provide system tray interface to glance at installed apps and port forwards
- Expand to full window for detailed management
- Built with Dioxus (Rust-based)
- Support Git-based configuration with local or authenticated user access
- Automatic configuration loading and refresh
- Versioned configuration
- Environment variables and non-secure tokens from CRDs (user-defined structure)
- Support multiple environments
- Deploy same app multiple times using namespaces and name suffixes

## Core Architecture Principles

### 1. Separation of Concerns
- **State Layer**: Git-stored configuration (source of truth)
- **Runtime Layer**: Local state cache with reconciliation
- **Kubernetes Layer**: Actual cluster state
- **UI Layer**: Dioxus tray + full window interface

### 2. Configuration Structure

Workspace-based approach similar to Cargo workspaces:

```
.kube-apps/
├── workspace.json          # Workspace metadata
├── apps/
│   ├── my-app/
│   │   ├── app.json       # App definition
│   │   ├── manifests/     # K8s YAML templates
│   │   │   ├── deployment.yaml
│   │   │   └── service.yaml
│   │   └── values/        # Environment-specific values
│   │       ├── dev.json
│   │       └── prod.json
│   └── another-app/
└── environments/
    ├── dev.json           # Environment config
    └── prod.json
```

### 3. Workspace Configuration

**workspace.json** structure:
```json
{
  "version": "1.0.0",
  "git": {
    "remote": "https://github.com/user/kube-apps",
    "branch": "main",
    "auto_sync": true,
    "sync_interval_seconds": 300
  },
  "user": {
    "identity_provider": "local|github|gitlab",
    "username": "auto-detect-or-manual"
  },
  "cluster": {
    "context": "minikube",
    "namespace_prefix": "apps"
  }
}
```

### 4. App Configuration

**apps/my-app/app.json** includes:
- App metadata (name, version, description)
- Manifest configuration (type, path, template engine)
- Connection configuration (port forwarding, ingress, load balancer)
- Dependencies (apps, CRDs)
- Deployment strategy
- Variables (static, environment, CRD-based)

## CRD Design

### CRD Structure
- User-defined Custom Resource Definitions
- Flexible structure (no hardcoded schema)
- JSONPath-based value extraction
- Environment-specific CRD instances
- Namespace-aware resolution

### CRD Example
```yaml
apiVersion: example.com/v1
kind: AppConfiguration
metadata:
  name: app-name-prod
spec:
  environment: prod
  variables:
    APP_SECRET_KEY: "prod-secret-key-value"
    API_KEY: "prod-api-key"
```

## Key Design Decisions

### Local-First Philosophy
- No cluster-side controllers
- Client-side orchestration
- State stored locally (Git + JSON cache)
- Works entirely from local machine

### Explicit Lifecycle
- User controls when apps start/stop
- No background reconciliation
- No magic automation
- User owns the lifecycle

### Zero Controllers
- No cluster-side components required
- Pure client-side application
- Uses standard Kubernetes APIs only
- No custom operators or controllers

## Validation

The combination of features **does not exist** in the current ecosystem:
- ✔ Git-backed intent
- ✔ App grouping
- ✔ Instance identity (namespace + suffix)
- ✔ Explicit port forwards
- ✔ CRD-based env sources
- ✔ Tray UX for lifecycle
- ✔ Expand-to-window
- ✔ Versioned configuration
- ✔ No magic reconciliation

## Market Position

**Local Kubernetes Application Runtime** - sits above kubectl, below GitOps, beside Docker Desktop

Fills the gap between:
- Dev-loop tools (Tilt/Skaffold) - too heavy for simple app running
- Cluster operators (Argo/Flux) - too opinionated for local dev
- Cluster viewers (Lens/K9s) - no app concept or lifecycle management

## Technology Choices

- **UI Framework**: Dioxus (Rust-based, cross-platform)
- **Backend**: Rust
- **Kubernetes Client**: kube-rs crate
- **Git Operations**: git2 or gix crate
- **Template Engines**: Handlebars, Tera
- **JSONPath**: jsonpath crate for CRD value extraction
- **Async Runtime**: Tokio

## References

- Source: `overview/desc.md`
- Related: doc-0004 (Talking Points), doc-0002 (Features), decision-0010 (Local-First Philosophy)
