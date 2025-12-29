# Roro Kube - High Level Talking Points

## Core Concept

**"Docker Compose for Kubernetes"** - A local-first, desktop application that manages Kubernetes apps with the simplicity of Docker Compose.

## The Problem

No tool exists that provides:
- Simple, local-first Kubernetes app management
- Git-backed configuration with versioning
- Native system tray interface
- Automatic port forwarding lifecycle
- App grouping (like Docker Desktop)
- CRD-based environment variable injection
- Multi-instance deployments with namespace isolation

## Market Gap

### What Exists (and Why They Don't Fit)

- **Tilt/Skaffold/DevSpace**: Dev-loop automation engines, not app managers
- **Argo CD/Flux**: Continuous reconciliation systems, hostile to local dev
- **Lens/K9s**: Cluster viewers, no app concept or port-forward lifecycle
- **Helm**: Templating engine, not an app runner
- **kubefwd**: Port forwarding only, no app definition or deployment
- **Acorn**: Dead (archived 2024) - closest match but required cluster-side controllers

### The Missing Layer

**Local Kubernetes Application Runtime** - sits above kubectl, below GitOps, beside Docker Desktop

## Key Features

### 1. App Grouping
- Define "apps" as collections of Kubernetes manifests
- Similar to Docker Desktop's grouped applications
- Each app is a templated kubectl manifest

### 2. Git-Backed Configuration
- All configuration stored in Git as JSON
- Automatic sync and refresh
- Versioned configuration
- Support for local or authenticated Git access

### 3. System Tray Interface
- Native Dioxus-based tray application
- Quick glance at installed apps and port-forward status
- Expand to full window for detailed management
- Status indicators and quick actions

### 4. Port Forwarding Management
- Automatic port forwarding setup
- Stable connections with auto-reconnect
- Lifecycle management (start/stop)
- Multiple ports per app

### 5. CRD-Based Environment Variables
- Environment variables and tokens from Custom Resource Definitions
- Flexible CRD structure (user-defined)
- JSONPath-based value extraction
- Multi-environment support (dev, staging, prod)

### 6. Multi-Instance Deployments
- Deploy same app multiple times
- Different namespaces per instance
- Name suffix support for instance identification
- Environment-specific configurations

### 7. Template Rendering
- Support for templated Kubernetes manifests
- Multiple template engines (Handlebars, Tera, etc.)
- Variable injection from static values, environment, and CRDs

## Architecture Principles

### 1. Local-First
- No cluster-side controllers
- Client-side orchestration
- State stored locally (Git + JSON cache)

### 2. Explicit Lifecycle
- User controls when apps start/stop
- No background reconciliation
- No magic automation

### 3. Separation of Concerns
- **State Layer**: Git-stored configuration (source of truth)
- **Runtime Layer**: Local state cache with reconciliation
- **Kubernetes Layer**: Actual cluster state
- **UI Layer**: Dioxus tray + full window interface

### 4. Workspace-Based Configuration
- Similar to Cargo workspaces
- Per-app configuration files
- Environment-specific values
- Manifest templates stored relative to app

## Technology Stack

- **UI**: Dioxus (Rust-based, cross-platform)
- **Backend**: Rust
- **Kubernetes**: k8s crate for API interactions
- **Git**: Git operations for configuration sync
- **Templates**: Handlebars/Tera for manifest rendering

## Key Differentiators

1. **Native Desktop App** - Not a web UI or CLI-only tool
2. **App-Centric** - Groups resources into logical applications
3. **Git as Source of Truth** - Versioned, collaborative configuration
4. **Zero Controllers** - No cluster-side components required
5. **Port Forwarding as First-Class** - Automatic, stable, managed
6. **CRD Integration** - Flexible environment variable injection
7. **Multi-Instance Support** - Deploy same app multiple times
8. **Local-First Philosophy** - User owns the lifecycle

## Use Cases

- Local development environment management
- Running multiple instances of apps for testing
- Managing app stacks with dependencies
- Quick access to services via port forwarding
- Team-shared configurations via Git

## Validation

The combination of features (Git-backed App groups + Native Tray + Auto-Port-Forwarding + CRD Injection) **does not exist** in the current ecosystem.

This fills a legitimate gap between:
- Dev-loop tools (Tilt/Skaffold) - too heavy for simple app running
- Cluster operators (Argo/Flux) - too opinionated for local dev
- Cluster viewers (Lens/K9s) - no app concept or lifecycle management

