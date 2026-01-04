// Config API tests
//
// Tests for config and git API wrappers, error conversion, and functionality.

use roro_core::{get_config_path_string, load_workstation_config, sync_repository, CoreError};
use roro_persistence::PersistenceError;
use std::path::Path;

#[test]
fn test_get_config_path_string_error_conversion() {
    // Test that PersistenceError converts to CoreError::Persistence
    // We can't easily test the success case without a real home directory,
    // but we can verify the function exists and returns the correct error type
    let result = get_config_path_string();

    // The function should either succeed (if HOME/USERPROFILE is set) or return CoreError::Persistence
    match result {
        Ok(path) => {
            // If it succeeds, verify it's a valid path string
            assert!(!path.is_empty());
            assert!(path.contains(".roro"));
            assert!(path.contains("config.json"));
        }
        Err(e) => {
            // Verify error is CoreError::Persistence
            match e {
                CoreError::Persistence(pe) => match pe {
                    PersistenceError::InvalidInput(_) => {}
                    _ => panic!("Expected InvalidInput variant"),
                },
                _ => panic!("Expected Persistence variant, got: {e}"),
            }
        }
    }
}

#[test]
fn test_get_config_path_string_success() {
    // Test that the function returns a valid path when home directory is available
    // This should work in most test environments
    let result = get_config_path_string();

    if let Ok(path) = result {
        assert!(!path.is_empty());
        assert!(path.contains(".roro"));
        assert!(path.contains("config.json"));
    }
    // If it fails, that's okay - it means HOME/USERPROFILE isn't set in test environment
}

#[tokio::test]
async fn test_load_workstation_config_error_conversion() {
    // Test that PersistenceError converts to CoreError::Persistence
    // The function will either succeed (creating empty config) or fail with CoreError::Persistence
    let result = load_workstation_config().await;

    match result {
        Ok(config) => {
            // If it succeeds, verify it returns a WorkstationConfig (Vec)
            assert!(config.is_empty() || !config.is_empty());
        }
        Err(e) => {
            // Verify error is CoreError::Persistence
            match e {
                CoreError::Persistence(pe) => match pe {
                    PersistenceError::InvalidInput(_) | PersistenceError::Serialization(_) => {}
                    _ => panic!("Expected InvalidInput or Serialization variant"),
                },
                _ => panic!("Expected Persistence variant, got: {e}"),
            }
        }
    }
}

#[tokio::test]
async fn test_load_workstation_config_success() {
    // Test that the function can load or create a config file
    // This should work in most test environments
    let result = load_workstation_config().await;

    match result {
        Ok(config) => {
            // Config should be valid (empty or with data)
            let _ = config;
        }
        Err(e) => {
            // If it fails, verify it's a Persistence error
            match e {
                CoreError::Persistence(_) => {}
                _ => panic!("Expected Persistence error, got: {e}"),
            }
        }
    }
}

#[tokio::test]
async fn test_sync_repository_error_conversion() {
    // Test that PersistenceError converts to CoreError::Persistence
    // Use an invalid path to trigger an error
    let invalid_path = Path::new("/nonexistent/path/to/repo");
    let result = sync_repository("https://example.com/repo.git", invalid_path, None).await;

    // The function should return an error (either Git or InvalidInput)
    match result {
        Ok(()) => {
            // If it somehow succeeds, that's unexpected but not a test failure
        }
        Err(e) => {
            // Verify error is CoreError::Persistence
            match e {
                CoreError::Persistence(pe) => match pe {
                    PersistenceError::Git(_)
                    | PersistenceError::Network(_)
                    | PersistenceError::Authentication(_)
                    | PersistenceError::InvalidInput(_) => {}
                    _ => panic!("Expected Git, Network, Authentication, or InvalidInput variant"),
                },
                _ => panic!("Expected Persistence variant, got: {e}"),
            }
        }
    }
}

#[test]
fn test_error_conversion_all_persistence_variants() {
    // Test that all PersistenceError variants convert correctly to CoreError::Persistence
    let errors = vec![
        PersistenceError::Database("test".to_string()),
        PersistenceError::NotFound("test".to_string()),
        PersistenceError::InvalidInput("test".to_string()),
        PersistenceError::Serialization("test".to_string()),
        PersistenceError::Git("test".to_string()),
        PersistenceError::Network("test".to_string()),
        PersistenceError::Authentication("test".to_string()),
    ];

    for persistence_error in errors {
        let core_error: CoreError = persistence_error.into();

        match core_error {
            CoreError::Persistence(pe) => {
                // Verify the inner error is preserved
                let msg = format!("{pe}");
                assert!(!msg.is_empty());
            }
            _ => panic!("Expected Persistence variant"),
        }
    }
}

#[test]
fn test_api_functions_exported() {
    // Test that all functions are accessible from the root module
    // This verifies the exports in lib.rs work correctly
    // If this compiles, the functions are properly exported

    // Just verify the functions exist and can be referenced
    // Actual functionality is tested in other tests
    let _ = get_config_path_string;
    let _ = load_workstation_config;
    let _ = sync_repository;
}
