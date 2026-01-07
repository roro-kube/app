// Config tests for Delete command
//
// These tests verify that the Delete command works correctly end-to-end,
// including file system operations and configuration persistence.

use std::env;

use roro_cli::{AddCommand, Command, DeleteCommand};
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

/// Helper to verify config file contents
async fn verify_config(expected_apps: &[(&str, &str)]) {
    let config = load_workstation_config()
        .await
        .expect("Failed to load config");
    
    assert_eq!(config.len(), expected_apps.len(), "Config length mismatch");
    
    for (expected_name, expected_git_url) in expected_apps {
        let app = config
            .iter()
            .find(|a| a.name == *expected_name)
            .expect(&format!("App '{}' not found in config", expected_name));
        
        assert_eq!(app.git_url, *expected_git_url);
    }
}

#[serial]
#[tokio::test]
async fn test_delete_command_success() {
    let (_temp_dir, original_home) = setup_test_env();
    
    // Add an app first
    let add_cmd = AddCommand::new(
        "to-delete".to_string(),
        "https://github.com/user/repo.git".to_string(),
        None,
        None,
        None,
        false,
    );
    add_cmd.execute().await.expect("Should add app successfully");
    
    // Verify it's there
    verify_config(&[("to-delete", "https://github.com/user/repo.git")]).await;
    
    // Delete it
    let delete_cmd = DeleteCommand::new("to-delete".to_string());
    let result = delete_cmd.execute().await;
    assert!(result.is_ok(), "Delete command should succeed");
    
    // Verify it's gone
    let config = load_workstation_config()
        .await
        .expect("Failed to load config");
    
    assert_eq!(config.len(), 0, "Config should be empty after delete");
    assert!(config.iter().find(|a| a.name == "to-delete").is_none());
    
    teardown_test_env(original_home);
}

#[serial]
#[tokio::test]
async fn test_delete_command_non_existent() {
    let (_temp_dir, original_home) = setup_test_env();
    
    // Try to delete an app that doesn't exist
    let delete_cmd = DeleteCommand::new("non-existent".to_string());
    let result = delete_cmd.execute().await;
    
    assert!(result.is_err(), "Should fail when deleting non-existent app");
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("not found"), "Error should mention not found");
    assert!(error_msg.contains("non-existent"), "Error should mention app name");
    
    teardown_test_env(original_home);
}

#[serial]
#[tokio::test]
async fn test_delete_command_from_multiple_apps() {
    let (_temp_dir, original_home) = setup_test_env();
    
    // Add multiple apps
    let add_cmd1 = AddCommand::new(
        "app1".to_string(),
        "https://github.com/user/repo1.git".to_string(),
        None,
        None,
        None,
        false,
    );
    add_cmd1.execute().await.expect("Should add app1");
    
    let add_cmd2 = AddCommand::new(
        "app2".to_string(),
        "https://github.com/user/repo2.git".to_string(),
        None,
        None,
        None,
        false,
    );
    add_cmd2.execute().await.expect("Should add app2");
    
    let add_cmd3 = AddCommand::new(
        "app3".to_string(),
        "https://github.com/user/repo3.git".to_string(),
        None,
        None,
        None,
        false,
    );
    add_cmd3.execute().await.expect("Should add app3");
    
    // Delete middle app
    let delete_cmd = DeleteCommand::new("app2".to_string());
    delete_cmd.execute().await.expect("Should delete app2");
    
    // Verify only app1 and app3 remain
    let config = load_workstation_config()
        .await
        .expect("Failed to load config");
    
    assert_eq!(config.len(), 2, "Should have 2 apps remaining");
    assert!(config.iter().any(|a| a.name == "app1"));
    assert!(config.iter().any(|a| a.name == "app3"));
    assert!(!config.iter().any(|a| a.name == "app2"));
    
    teardown_test_env(original_home);
}

#[serial]
#[tokio::test]
async fn test_add_delete_add_workflow() {
    let (_temp_dir, original_home) = setup_test_env();
    
    // Add app
    let add_cmd1 = AddCommand::new(
        "workflow-app".to_string(),
        "https://github.com/user/repo.git".to_string(),
        None,
        None,
        None,
        false,
    );
    add_cmd1.execute().await.expect("Should add app");
    
    // Delete it
    let delete_cmd = DeleteCommand::new("workflow-app".to_string());
    delete_cmd.execute().await.expect("Should delete app");
    
    // Add it again (should work, as it no longer exists)
    let add_cmd2 = AddCommand::new(
        "workflow-app".to_string(),
        "https://github.com/user/repo2.git".to_string(),
        None,
        None,
        None,
        false,
    );
    add_cmd2.execute().await.expect("Should add app again");
    
    // Verify final state
    verify_config(&[("workflow-app", "https://github.com/user/repo2.git")]).await;
    
    teardown_test_env(original_home);
}
