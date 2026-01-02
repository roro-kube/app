---
id: task-0024
title: Implement Git repository cloning and initialization
status: To Do
assignee: []
created_date: '2026-01-01 16:37'
updated_date: '2026-01-01 17:09'
labels:
  - git
  - backend
milestone: m-1
dependencies:
  - task-0018
priority: high
ordinal: 8000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement functionality to clone Git repositories and initialize local Git repositories for storing workspace configurations, supporting both SSH and HTTPS authentication.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Git repository cloning implemented using git2 or gix
- [x] #2 Support for SSH and HTTPS authentication
- [x] #3 Local Git repository initialization implemented
- [x] #4 Error handling for network failures and authentication errors
- [x] #5 Unit tests verify cloning and initialization
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
- Create Git module in persistence or core layer
- Implement clone functionality with authentication support
- Implement local repository initialization
- Add error handling and unit tests
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Created `persistence/src/git/mod.rs` module with Git operations
- Added `git2` dependency to `persistence/Cargo.toml` (from workspace dependencies)
- Extended `PersistenceError` enum with Git-specific error variants:
  - `Git(String)` - General Git operation errors
  - `Network(String)` - Network-related errors during Git operations
  - `Authentication(String)` - Authentication failures
- Implemented `clone_repository()` function:
  - Supports both SSH and HTTPS authentication
  - Accepts optional credentials (username, password/token)
  - Uses `git2::build::RepoBuilder` for cloning
  - Handles authentication via `RemoteCallbacks` with fallback to SSH agent and default credentials
  - Properly converts string references to owned Strings for async task spawning
- Implemented `init_repository()` function:
  - Initializes local Git repositories (both regular and bare)
  - Uses `Repository::init_opts()` with configurable options
- Both functions use `tokio::task::spawn_blocking` to handle blocking Git operations
- Added comprehensive unit tests:
  - Test repository initialization (regular and bare)
  - Test cloning public repositories
  - Test cloning with credentials (handles authentication errors gracefully)
- All tests pass successfully
- Module exported from `persistence/src/lib.rs`
<!-- SECTION:NOTES:END -->
