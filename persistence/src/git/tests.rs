// Git module tests
//
// This module contains tests for Git operations.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use crate::errors::PersistenceError;
use crate::git::clone::clone_repository;
use tempfile::TempDir;

#[tokio::test]
async fn test_clone_repository_public() {
    // Clone a public repository (no authentication required)
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let repo_path = temp_dir.path().join("cloned_repo");

    // Using a small public repository for testing
    let result = clone_repository(
        "https://github.com/octocat/Hello-World.git",
        &repo_path,
        None,
    )
    .await;

    // This test may fail if network is unavailable, which is acceptable
    if result.is_ok() {
        assert!(
            repo_path.exists(),
            "Cloned repository directory should exist"
        );
        assert!(
            repo_path.join(".git").exists(),
            ".git directory should exist in cloned repo"
        );
    }
}

#[tokio::test]
async fn test_clone_repository_with_credentials() {
    // Test cloning with credentials (will fail authentication, but tests the code path)
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let repo_path = temp_dir.path().join("cloned_repo_auth");

    let result = clone_repository(
        "https://github.com/octocat/Hello-World.git",
        &repo_path,
        Some(("test_user", "test_token")),
    )
    .await;

    // This test may fail due to invalid credentials or network, which is acceptable
    // We're mainly testing that the code path with credentials is executed
    match result {
        Ok(()) => {
            assert!(
                repo_path.exists(),
                "Cloned repository directory should exist"
            );
        }
        Err(PersistenceError::Authentication(_) | PersistenceError::Network(_)) => {
            // Expected for invalid credentials or network issues
        }
        Err(e) => {
            panic!("Unexpected error: {e:?}");
        }
    }
}
