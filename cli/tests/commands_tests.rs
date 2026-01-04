// CLI commands tests
//
// Tests for CLI command implementations to verify they work correctly with core layer APIs.

use roro_cli::commands::{Command, StatusCommand, SyncCommand};
use roro_domain::{AppReference, WorkstationConfig};

#[tokio::test]
async fn test_status_command_executes() {
    let cmd = StatusCommand::new();
    let result = cmd.execute().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_sync_command_app_not_found() {
    let empty_config: WorkstationConfig = Vec::new();
    let cmd = SyncCommand::new("nonexistent-app".to_string(), empty_config);
    let result = cmd.execute().await;

    let Err(error_msg) = result else {
        panic!("Expected error for nonexistent app");
    };
    assert!(error_msg.contains("not found"));
    assert!(error_msg.contains("nonexistent-app"));
}

#[tokio::test]
async fn test_sync_command_app_found_but_sync_fails() {
    let app_ref = AppReference {
        name: "test-app".to_string(),
        git_url: "https://invalid-url-that-does-not-exist.git".to_string(),
        local_path: None,
        sync_interval: None,
        kubectl_context: None,
    };
    let config: WorkstationConfig = vec![app_ref];
    let cmd = SyncCommand::new("test-app".to_string(), config);
    let result = cmd.execute().await;

    // The sync should fail (invalid URL), but error should be formatted correctly
    let Err(error_msg) = result else {
        panic!("Expected error for invalid git URL");
    };
    // Error should be user-friendly (not raw CoreError)
    assert!(!error_msg.contains("CoreError"));
    assert!(!error_msg.contains("Persistence error: Persistence error:"));
}

#[tokio::test]
async fn test_sync_command_uses_core_apis() {
    // Verify that SyncCommand uses core layer APIs by checking it compiles
    // and handles CoreError correctly
    let app_ref = AppReference {
        name: "test-app".to_string(),
        git_url: "https://example.com/repo.git".to_string(),
        local_path: Some("/tmp/test-repo".to_string()),
        sync_interval: None,
        kubectl_context: None,
    };
    let config: WorkstationConfig = vec![app_ref];
    let cmd = SyncCommand::new("test-app".to_string(), config);

    // The command should be created successfully
    // Actual sync may fail, but error handling should work
    let result = cmd.execute().await;

    // Result may be Ok or Err depending on whether git operations succeed
    // But error messages should be properly formatted if it fails
    if let Err(e) = result {
        // Verify error message doesn't have double "Persistence error:" prefix
        assert!(!e.contains("Persistence error: Persistence error:"));
        // Verify it's a user-friendly error message
        assert!(!e.contains("CoreError::"));
    }
}

#[test]
fn test_status_command_new() {
    let cmd = StatusCommand::new();
    // Just verify it can be created
    let _ = cmd;
}

#[test]
fn test_sync_command_new() {
    let config: WorkstationConfig = Vec::new();
    let cmd = SyncCommand::new("test".to_string(), config);
    // Just verify it can be created
    let _ = cmd;
}
