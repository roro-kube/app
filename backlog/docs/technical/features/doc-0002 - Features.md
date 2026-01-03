---
id: doc-0002
title: Features
type: technical
created_date: '2025-12-30 21:01'
updated_date: '2025-12-30 21:01'
---

# Roro Kube - Feature Set

## Overview

Roro Kube is a **"Docker Compose for Kubernetes"** - a local-first desktop application that manages Kubernetes applications with the simplicity of Docker Compose. This document outlines the complete feature set.

## Core Features

### 1. App Grouping & Management

**App-Centric Organization**
- Define "apps" as logical collections of Kubernetes manifests
- Group related resources (deployments, services, configmaps, etc.) into a single app
- Similar to Docker Desktop's grouped applications
- Each app represents a complete, deployable unit

**App Definition**
- Apps defined via JSON configuration files
- Support for templated Kubernetes manifests
- Per-app manifest directories
- App metadata (name, version, description)

**App Lifecycle**
- Deploy apps to Kubernetes clusters
- Start/stop apps on demand
- Restart apps with updated configurations
- Delete apps and clean up resources
- View app status and health

### 2. Git-Backed Configuration

**Version Control Integration**
- All configuration stored in Git repositories
- JSON-based configuration files
- Versioned app definitions
- Collaborative configuration management

**Git Operations**
- Automatic configuration sync from Git
- Configurable sync intervals
- Manual sync trigger
- Support for authenticated Git access (SSH, HTTPS)
- Local user detection from Git config
- Manual authentication flow (GitHub, GitLab)

**Conflict Resolution**
- Detect configuration conflicts
- UI for conflict resolution
- Merge/rebase strategies
- Local change tracking and push capability

**Workspace Structure**
- Workspace-based configuration (similar to Cargo workspaces)
- Per-app configuration files (`app.json`)
- Environment-specific configuration files
- Manifest templates stored relative to app directory

### 3. System Tray Interface

**Native Desktop Integration**
- Native system tray application built with Dioxus
- Cross-platform support (Windows, macOS, Linux)
- Lightweight and always accessible

**Tray Menu**
- Quick glance at installed apps
- Port-forward status indicators
- App status at a glance (running, stopped, error)
- Quick actions (deploy, stop, sync)

**Expandable Window**
- Click tray icon to expand to full window
- Detailed app management interface
- Instance management
- Port forwarding controls
- Settings and configuration

**Status Indicators**
- Visual status indicators (green/yellow/red)
- Real-time status updates
- Connection status to cluster
- Port-forward active indicators

### 4. Port Forwarding Management

**Automatic Port Forwarding**
- Automatic setup when apps are deployed
- Multiple ports per app/service
- Configurable local and remote port mappings
- Protocol support (TCP, UDP)

**Stable Connections**
- Auto-reconnect on connection failure
- Health monitoring for port forwards
- Persistent port forwards across app restarts
- Port conflict detection and resolution

**Lifecycle Management**
- Start/stop port forwarding on demand
- Manual port forward controls
- Automatic cleanup on app deletion
- Port forward status tracking

**Browser Integration**
- Auto-open browser URLs for forwarded ports
- Configurable browser URLs per port
- Quick access to forwarded services

### 5. CRD-Based Environment Variables

**Flexible CRD Integration**
- Support for user-defined Custom Resource Definitions
- No hardcoded CRD structure requirements
- Works with any CRD schema

**JSONPath-Based Extraction**
- JSONPath-based value extraction from CRDs
- Flexible mapping configuration
- Support for nested CRD structures
- Dynamic value resolution

**Environment Variable Injection**
- Inject CRD values as environment variables
- Support for tokens and non-secure secrets
- Multi-environment support (dev, staging, prod)
- Namespace-aware CRD resolution

**CRD Resolution**
- Automatic CRD fetching from Kubernetes API
- CRD value caching for performance
- Error handling for missing/invalid CRDs
- Support for multiple CRD sources per app

### 6. Multi-Instance Deployments

**Instance Management**
- Deploy the same app multiple times
- Unique instance identification (UUID or user-defined)
- Instance-specific configurations
- Instance status tracking

**Namespace Isolation**
- Different namespace per instance
- Namespace template support
- Automatic namespace creation
- Namespace-per-instance isolation

**Instance Identification**
- Name suffix support for instances
- Environment-specific instance naming
- Instance grouping and filtering
- Instance metadata and labels

**Multi-Environment Support**
- Deploy apps to different environments
- Environment-specific configurations
- Environment switching in UI
- Environment-specific CRD values

### 7. Template Rendering

**Template Engine Support**
- Multiple template engines (Handlebars, Tera)
- Support for raw (non-templated) manifests
- Template engine selection per app
- Template validation

**Variable Injection**
- Static variables from configuration
- Environment variables from system
- CRD values via JSONPath
- Computed values (namespace, instance_id, app_name, etc.)
- Template context building

**Manifest Processing**
- Multi-file manifest directory support
- Template rendering pipeline
- Rendered manifest validation
- Template caching for performance

**Template Features**
- Conditional rendering
- Loops and iterations
- Variable substitution
- Namespace templating
- Instance-specific templating

### 8. State Management & Reconciliation

**State Layers**
- **Git Layer**: Source of truth (configuration)
- **Local Cache**: Runtime state cache
- **Kubernetes Layer**: Actual cluster state
- **UI Layer**: User interface state

**Reconciliation Engine**
- Compare desired state (from Git) vs current state (from cluster)
- Generate reconciliation actions (Create, Update, Delete, PortForwarding)
- Execute reconciliation pipeline
- Error handling and reporting

**State Persistence**
- Local state cache (JSON file)
- Cache persistence across restarts
- Cache invalidation logic
- Cache synchronization with Git config

**Explicit Lifecycle**
- User controls when apps start/stop
- No background reconciliation
- No automatic updates
- User owns the lifecycle

### 9. Kubernetes Integration

**Cluster Management**
- Support for multiple Kubernetes contexts
- Cluster connection validation
- Context switching
- Error handling for cluster unavailability

**Resource Management**
- Apply Kubernetes manifests
- Resource validation before applying
- Update detection and handling
- Rollback capability
- Deployment status tracking

**Namespace Management**
- Automatic namespace creation
- Namespace template resolution
- Namespace cleanup
- Namespace isolation

**Health Monitoring**
- Health check endpoint polling
- Configurable health checks per app
- Health status reporting
- Automatic restart on health check failure (optional)

### 10. Dependency Management

**App Dependencies**
- Define app dependencies
- Dependency resolution and ordering
- Dependency health checking
- Circular dependency detection

**Deployment Ordering**
- Deploy dependencies before dependents
- Dependency status tracking
- Handle dependency failures gracefully

### 11. User Interface Features

**Main Window**
- App list view with status indicators
- Instance detail view
- Port forward management UI
- Deployment controls (start, stop, restart, delete)
- Environment switching UI
- Settings and configuration UI

**UI Components**
- Reusable status indicator component
- App card component
- Port forward list component
- Instance detail panel
- Action buttons (deploy, stop, logs, etc.)
- Error message display
- Loading/spinner components

**Real-time Updates**
- Real-time status updates from backend
- Event system for state changes
- UI refresh on configuration changes
- Notification system for important events

**Logging & Observability**
- Structured logging
- Log aggregation for instances
- Log viewer in UI
- Error tracking and reporting

### 12. Advanced Features

**Workspace Management**
- Multiple workspace support
- Workspace switching
- Workspace-specific configurations
- Workspace isolation

**Configuration Management**
- Configuration file validation
- Configuration schema support
- Configuration file watcher for live updates
- Configuration versioning

**Performance Optimization**
- Template rendering caching
- CRD value caching
- Lazy loading where appropriate
- Performance monitoring

**Error Handling**
- Comprehensive error handling
- Network failure handling
- Cluster unavailability handling
- Git sync failure handling
- Graceful degradation

## Architecture Principles

### Local-First
- No cluster-side controllers required
- Client-side orchestration
- State stored locally (Git + JSON cache)
- Works entirely from local machine

### Zero Controllers
- No cluster-side components
- Pure client-side application
- Uses standard Kubernetes APIs only
- No custom operators or controllers

### Explicit Lifecycle
- User controls when apps start/stop
- No background reconciliation
- No magic automation
- User owns the lifecycle

### Separation of Concerns
- **State Layer**: Git-stored configuration (source of truth)
- **Runtime Layer**: Local state cache with reconciliation
- **Kubernetes Layer**: Actual cluster state
- **UI Layer**: Dioxus tray + full window interface

## Technology Stack

- **UI Framework**: Dioxus (Rust-based, cross-platform)
- **Backend Language**: Rust
- **Kubernetes Client**: kube-rs crate
- **Git Operations**: git2 or gix crate
- **Template Engines**: Handlebars, Tera
- **JSON Processing**: serde, serde_json
- **JSONPath**: jsonpath crate for CRD value extraction
- **Async Runtime**: Tokio

## Key Differentiators

1. **Native Desktop App** - Not a web UI or CLI-only tool
2. **App-Centric** - Groups resources into logical applications
3. **Git as Source of Truth** - Versioned, collaborative configuration
4. **Zero Controllers** - No cluster-side components required
5. **Port Forwarding as First-Class** - Automatic, stable, managed
6. **CRD Integration** - Flexible environment variable injection
7. **Multi-Instance Support** - Deploy same app multiple times
8. **Local-First Philosophy** - User owns the lifecycle
9. **Workspace-Based** - Similar to Cargo workspaces
10. **Template-Agnostic** - Support multiple template engines

## Use Cases

- **Local Development Environment Management**
  - Manage local Kubernetes development environments
  - Quick access to services via port forwarding
  - Easy app lifecycle management

- **Multi-Instance Testing**
  - Run multiple instances of apps for testing
  - Isolated namespaces per instance
  - Environment-specific configurations

- **App Stack Management**
  - Manage app stacks with dependencies
  - Deploy related apps together
  - Coordinate app lifecycles

- **Team Collaboration**
  - Team-shared configurations via Git
  - Versioned app definitions
  - Collaborative configuration management

- **Environment Management**
  - Switch between dev, staging, prod environments
  - Environment-specific configurations
  - CRD-based environment variable injection

## Feature Comparison

| Feature | Roro Kube | Docker Compose | Tilt/Skaffold | Argo CD/Flux | Lens/K9s |
|---------|-----------|----------------|---------------|--------------|----------|
| App Grouping | ✅ | ✅ | ⚠️ | ⚠️ | ❌ |
| Git-Backed Config | ✅ | ❌ | ⚠️ | ✅ | ❌ |
| Native Desktop UI | ✅ | ❌ | ❌ | ❌ | ✅ |
| Auto Port Forwarding | ✅ | ✅ | ✅ | ❌ | ⚠️ |
| CRD Integration | ✅ | ❌ | ❌ | ⚠️ | ❌ |
| Multi-Instance | ✅ | ⚠️ | ⚠️ | ⚠️ | ❌ |
| Local-First | ✅ | ✅ | ✅ | ❌ | ✅ |
| Zero Controllers | ✅ | ✅ | ✅ | ❌ | ✅ |
| Template Support | ✅ | ❌ | ⚠️ | ⚠️ | ❌ |
| Explicit Lifecycle | ✅ | ✅ | ❌ | ❌ | ✅ |

**Legend:**
- ✅ Full support
- ⚠️ Partial support or different approach
- ❌ Not supported

## Future Considerations

Potential future features (not in initial scope):
- Web UI option (in addition to native)
- CLI interface
- Plugin system
- App marketplace
- Metrics and monitoring integration
- Advanced health checking strategies
- Resource quota management
- Multi-cluster support

## References

- Source: `overview/features.md`
- Related: decision-0016 (App-Centric Model), decision-0017 (Port Forwarding as First-Class Feature), decision-0010 (Local-First Philosophy)
