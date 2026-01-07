// Config tests for Sync command
//
// These tests verify that the Sync command works correctly end-to-end,
// including file system operations and configuration persistence.

use std::env;

use roro_cli::{AddCommand, Command, SyncCommand};
use roro_domain::WorkstationConfig;
use roro_persistence::load_workstation_config;
use serial_test::serial;
use tempfile::TempDir;

/// Setup a temporary environment for testing
/// Returns the temp directory and the original HOME/USERPROFILE value
fn setup_test_env() -> (TempDir, Option<String>) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let test_home = temp_dir.path().to_path_buf();
    
    // Save original env var value
    let original_home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .ok();
    
    // Set temporary HOME/USERPROFILE
    // The persistence layer checks HOME first, then USERPROFILE
    #[cfg(unix)]
    {
        env::set_var("HOME", test_home.as_os_str());
    }
    #[cfg(windows)]
    {
        // On Windows, set USERPROFILE (HOME will fall back to this if not set)
        env::set_var("USERPROFILE", test_home.as_os_str());
        // Also unset HOME if it exists to ensure consistent behavior
        env::remove_var("HOME");
    }
    
    (temp_dir, original_home)
}

/// Clean up test environment
fn teardown_test_env(original_home: Option<String>) {
    if let Some(home) = original_home {
        #[cfg(unix)]
        env::set_var("HOME", &home);
        #[cfg(windows)]
        env::set_var("USERPROFILE", &home);
    } else {
        #[cfg(unix)]
        env::remove_var("HOME");
        #[cfg(windows)]
        env::remove_var("USERPROFILE");
    }
}

#[serial]
#[tokio::test]
async fn test_sync_command_app_not_found() {
    let (_temp_dir, original_home) = setup_test_env();
    
    // Try to sync an app that doesn't exist
    let empty_config: WorkstationConfig = Vec::new();
    let sync_cmd = SyncCommand::new("non-existent-app".to_string(), empty_config);
    
    let result = sync_cmd.execute().await;
    assert!(result.is_err(), "Should fail when app not found");
    
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("not found"), "Error should mention not found");
    assert!(error_msg.contains("non-existent-app"), "Error should mention app name");
    
    teardown_test_env(original_home);
}

#[serial]
#[tokio::test]
async fn test_sync_command_with_invalid_git_url() {
    let (_temp_dir, original_home) = setup_test_env();
    
    // Add an app with an invalid git URL
    let add_cmd = AddCommand::new(
        "invalid-sync-app".to_string(),
        "https://invalid-url-that-does-not-exist-12345.git".to_string(),
        None,
        None,
        None,
        false,
    );
    add_cmd.execute().await.expect("Should add app (even with invalid URL)");
    
    // Try to sync it - should fail but with proper error message
    let config = load_workstation_config()
        .await
        .expect("Failed to load config");
    
    let sync_cmd = SyncCommand::new("invalid-sync-app".to_string(), config);
    let result = sync_cmd.execute().await;
    
    // Sync should fail, but error should be user-friendly
    assert!(result.is_err(), "Sync should fail with invalid URL");
    let error_msg = result.unwrap_err();
    // Error should not expose internal error types
    assert!(!error_msg.contains("CoreError::"), "Error should not contain CoreError");
    assert!(!error_msg.contains("Persistence error: Persistence error:"), 
            "Error should not have double prefix");
    
    teardown_test_env(original_home);
}
