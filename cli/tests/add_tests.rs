// Config tests for Add command
//
// These tests verify that the Add command works correctly end-to-end,
// including file system operations and configuration persistence.

use std::env;

use roro_cli::{AddCommand, Command};
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
async fn test_add_command_success() {
    let (_temp_dir, original_home) = setup_test_env();
    
    // Test adding a basic app reference
    let cmd = AddCommand::new(
        "test-app".to_string(),
        "https://github.com/user/repo.git".to_string(),
        None,
        None,
        None,
        false,
    );
    
    let result = cmd.execute().await;
    assert!(result.is_ok(), "Add command should succeed");
    
    // Verify the app was added
    verify_config(&[("test-app", "https://github.com/user/repo.git")]).await;
    
    teardown_test_env(original_home);
}

#[serial]
#[tokio::test]
async fn test_add_command_with_all_parameters() {
    let (_temp_dir, original_home) = setup_test_env();
    
    // Test adding an app with all optional parameters
    let cmd = AddCommand::new(
        "full-app".to_string(),
        "https://github.com/user/repo.git".to_string(),
        Some("/custom/path".to_string()),
        Some(30000),
        Some("my-context".to_string()),
        false,
    );
    
    let result = cmd.execute().await;
    assert!(result.is_ok(), "Add command should succeed");
    
    // Verify all fields were set correctly
    let config = load_workstation_config()
        .await
        .expect("Failed to load config");
    
    let app = config
        .iter()
        .find(|a| a.name == "full-app")
        .expect("App not found");
    
    assert_eq!(app.git_url, "https://github.com/user/repo.git");
    assert_eq!(app.local_path, Some("/custom/path".to_string()));
    assert_eq!(app.sync_interval, Some(30000));
    assert_eq!(app.kubectl_context, Some("my-context".to_string()));
    
    teardown_test_env(original_home);
}

#[serial]
#[tokio::test]
async fn test_add_command_duplicate_without_force() {
    let (_temp_dir, original_home) = setup_test_env();
    
    // Add first app
    let cmd1 = AddCommand::new(
        "duplicate-app".to_string(),
        "https://github.com/user/repo1.git".to_string(),
        None,
        None,
        None,
        false,
    );
    cmd1.execute().await.expect("First add should succeed");
    
    // Try to add the same app again without force
    let cmd2 = AddCommand::new(
        "duplicate-app".to_string(),
        "https://github.com/user/repo2.git".to_string(),
        None,
        None,
        None,
        false,
    );
    
    let result = cmd2.execute().await;
    assert!(result.is_err(), "Should fail when adding duplicate without force");
    
    let error_msg = result.unwrap_err();
    assert!(error_msg.contains("already exists"), "Error should mention duplicate");
    assert!(error_msg.contains("duplicate-app"), "Error should mention app name");
    assert!(error_msg.contains("--force"), "Error should suggest --force flag");
    
    // Verify original app is still there
    verify_config(&[("duplicate-app", "https://github.com/user/repo1.git")]).await;
    
    teardown_test_env(original_home);
}

#[serial]
#[tokio::test]
async fn test_add_command_duplicate_with_force() {
    let (_temp_dir, original_home) = setup_test_env();
    
    // Add first app
    let cmd1 = AddCommand::new(
        "overwrite-app".to_string(),
        "https://github.com/user/repo1.git".to_string(),
        None,
        None,
        None,
        false,
    );
    cmd1.execute().await.expect("First add should succeed");
    
    // Overwrite with forcemise run test -- --test add_tests test_add_command_duplicate_with_force
    let cmd2 = AddCommand::new(
        "overwrite-app".to_string(),
        "https://github.com/user/repo2.git".to_string(),
        Some("/new/path".to_string()),
        Some(60000),
        Some("new-context".to_string()),
        true, // force
    );
    
    let result = cmd2.execute().await;
    assert!(result.is_ok(), "Should succeed with --force");
    
    // Verify app was overwritten with new values
    let config = load_workstation_config()
        .await
        .expect("Failed to load config");
    
    assert_eq!(config.len(), 1, "Should have only one app");
    let app = &config[0];
    assert_eq!(app.name, "overwrite-app");
    assert_eq!(app.git_url, "https://github.com/user/repo2.git");
    assert_eq!(app.local_path, Some("/new/path".to_string()));
    assert_eq!(app.sync_interval, Some(60000));
    assert_eq!(app.kubectl_context, Some("new-context".to_string()));
    
    teardown_test_env(original_home);
}

#[serial]
#[tokio::test]
async fn test_add_command_multiple_apps() {
    let (_temp_dir, original_home) = setup_test_env();
    
    // Add multiple different apps
    let apps = vec![
        ("app1", "https://github.com/user/repo1.git"),
        ("app2", "https://github.com/user/repo2.git"),
        ("app3", "https://github.com/user/repo3.git"),
    ];
    
    for (name, url) in &apps {
        let cmd = AddCommand::new(
            name.to_string(),
            url.to_string(),
            None,
            None,
            None,
            false,
        );
        cmd.execute().await.expect(&format!("Failed to add {}", name));
    }
    
    // Verify all apps are present
    let config = load_workstation_config()
        .await
        .expect("Failed to load config");
    
    assert_eq!(config.len(), 3, "Should have 3 apps");
    verify_config(&apps).await;
    
    teardown_test_env(original_home);
}
