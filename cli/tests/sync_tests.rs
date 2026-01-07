// Config tests for Sync command
//
// These tests verify that the Sync command works correctly end-to-end,
// including file system operations and configuration persistence.

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

use git2::{Repository, Signature};
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

/// Create a local git repository for testing
/// Returns a file:// URL that can be used to clone it
fn create_test_git_repo(temp_dir: &TempDir) -> String {
    let repo_path = temp_dir.path().join("test_repo");
    
    // Initialize a bare git repository (better for cloning)
    let repo = Repository::init_bare(&repo_path).expect("Failed to initialize bare git repository");
    
    // Create a temporary working repository to make commits
    let work_repo_path = temp_dir.path().join("work_repo");
    let work_repo = Repository::init(&work_repo_path).expect("Failed to initialize work repository");
    
    // Create a test file
    let file_path = work_repo_path.join("test.txt");
    let mut file = fs::File::create(&file_path).expect("Failed to create test file");
    writeln!(file, "Hello, World!").expect("Failed to write to test file");
    drop(file);
    
    // Add and commit the file
    let mut index = work_repo.index().expect("Failed to get repository index");
    index.add_path(Path::new("test.txt")).expect("Failed to add file to index");
    let tree_id = index.write_tree().expect("Failed to write tree");
    let tree = work_repo.find_tree(tree_id).expect("Failed to find tree");
    
    let sig = Signature::now("Test User", "test@example.com")
        .expect("Failed to create signature");
    work_repo
        .commit(
            Some("refs/heads/main"),
            &sig,
            &sig,
            "Initial commit",
            &tree,
            &[],
        )
        .expect("Failed to create commit");
    
    // Push to the bare repository
    let mut remote = work_repo
        .remote("origin", repo_path.to_str().expect("Invalid path"))
        .expect("Failed to create remote");
    remote
        .push(
            &["refs/heads/main:refs/heads/main"],
            None,
        )
        .expect("Failed to push to bare repository");
    
    // Set HEAD in bare repository
    repo.set_head("refs/heads/main")
        .expect("Failed to set HEAD to main");
    
    // Convert path to file:// URL for cross-platform compatibility
    // Use canonicalize to get absolute path
    let canonical_path = repo_path
        .canonicalize()
        .expect("Failed to canonicalize repository path");
    
    #[cfg(unix)]
    let repo_url = {
        // On Unix, file:// URLs need three slashes for absolute paths: file:///path
        let path_str = canonical_path.to_string_lossy();
        format!("file://{}", path_str)
    };
    #[cfg(windows)]
    let repo_url = {
        // On Windows, canonicalize adds \\?\ prefix which we need to strip
        // Also convert backslashes to forward slashes for file:// URLs
        // Format: file:///C:/path/to/repo (three slashes)
        let path_str = canonical_path.to_string_lossy();
        // Strip the \\?\ prefix if present (Windows extended path prefix)
        let clean_path = if path_str.starts_with(r"\\?\") {
            &path_str[4..]
        } else {
            &path_str
        };
        let normalized = clean_path.replace('\\', "/");
        format!("file:///{}", normalized)
    };
    
    repo_url
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

#[serial]
#[tokio::test]
async fn test_sync_command_clone_with_default_path() {
    let (temp_dir, original_home) = setup_test_env();
    
    // Create a local test git repository
    let repo_url = create_test_git_repo(&temp_dir);
    
    // Add an app with the local repository
    let add_cmd = AddCommand::new(
        "test-sync-app".to_string(),
        repo_url.clone(),
        None, // No custom path, should use default ~/.roro/remote/{name}
        None,
        None,
        false,
    );
    add_cmd.execute().await.expect("Should add app successfully");
    
    // Load config and sync
    let config = load_workstation_config()
        .await
        .expect("Failed to load config");
    
    let sync_cmd = SyncCommand::new("test-sync-app".to_string(), config);
    let result = sync_cmd.execute().await;
    
    assert!(result.is_ok(), "Sync should succeed with local repository");
    
    // Verify repository was cloned to default path
    let home = temp_dir.path();
    let expected_path = home.join(".roro").join("remote").join("test-sync-app");
    
    assert!(
        expected_path.exists(),
        "Repository should be cloned to default path"
    );
    assert!(
        expected_path.join(".git").exists(),
        ".git directory should exist in cloned repo"
    );
    
    // Verify the test file was cloned
    assert!(
        expected_path.join("test.txt").exists(),
        "Test file should exist in cloned repo"
    );
    
    teardown_test_env(original_home);
}

#[serial]
#[tokio::test]
async fn test_sync_command_clone_with_custom_path() {
    let (temp_dir, original_home) = setup_test_env();
    
    // Create a local test git repository
    let repo_url = create_test_git_repo(&temp_dir);
    
    // Add an app with a custom local path
    let custom_path = temp_dir.path().join("custom").join("repo").to_string_lossy().to_string();
    let add_cmd = AddCommand::new(
        "custom-path-app".to_string(),
        repo_url,
        Some(custom_path.clone()),
        None,
        None,
        false,
    );
    add_cmd.execute().await.expect("Should add app successfully");
    
    // Load config and sync
    let config = load_workstation_config()
        .await
        .expect("Failed to load config");
    
    let sync_cmd = SyncCommand::new("custom-path-app".to_string(), config);
    let result = sync_cmd.execute().await;
    
    assert!(result.is_ok(), "Sync should succeed with local repository");
    
    // Verify repository was cloned to custom path
    let expected_path = std::path::PathBuf::from(&custom_path);
    
    assert!(
        expected_path.exists(),
        "Repository should be cloned to custom path"
    );
    assert!(
        expected_path.join(".git").exists(),
        ".git directory should exist in cloned repo"
    );
    
    // Verify the test file was cloned
    assert!(
        expected_path.join("test.txt").exists(),
        "Test file should exist in cloned repo"
    );
    
    teardown_test_env(original_home);
}

#[serial]
#[tokio::test]
async fn test_sync_command_fetch_existing_repository() {
    let (temp_dir, original_home) = setup_test_env();
    
    // Create a local test git repository
    let repo_url = create_test_git_repo(&temp_dir);
    
    // Add an app with the local repository
    let add_cmd = AddCommand::new(
        "fetch-test-app".to_string(),
        repo_url.clone(),
        None,
        None,
        None,
        false,
    );
    add_cmd.execute().await.expect("Should add app successfully");
    
    // Load config and sync (first time - clone)
    let config = load_workstation_config()
        .await
        .expect("Failed to load config");
    
    let sync_cmd1 = SyncCommand::new("fetch-test-app".to_string(), config.clone());
    let result1 = sync_cmd1.execute().await;
    
    assert!(result1.is_ok(), "First sync (clone) should succeed");
    
    // Add a new commit to the source repository using the work repository
    // (We can't modify bare repositories directly)
    let work_repo_path = temp_dir.path().join("work_repo");
    let work_repo = Repository::open(&work_repo_path).expect("Failed to open work repo");
    
    let new_file_path = work_repo_path.join("new_file.txt");
    let mut new_file = fs::File::create(&new_file_path).expect("Failed to create new file");
    writeln!(new_file, "New content").expect("Failed to write to new file");
    drop(new_file);
    
    // Get the current HEAD commit as parent
    let head = work_repo.head().expect("Failed to get HEAD");
    let parent_oid = head.target().expect("HEAD has no target");
    let parent_commit = work_repo.find_commit(parent_oid).expect("Failed to find HEAD commit");
    
    let mut index = work_repo.index().expect("Failed to get index");
    index.add_path(Path::new("new_file.txt")).expect("Failed to add file");
    let tree_id = index.write_tree().expect("Failed to write tree");
    let tree = work_repo.find_tree(tree_id).expect("Failed to find tree");
    
    let sig = Signature::now("Test User", "test@example.com")
        .expect("Failed to create signature");
    work_repo
        .commit(
            Some("refs/heads/main"),
            &sig,
            &sig,
            "Second commit",
            &tree,
            &[&parent_commit],
        )
        .expect("Failed to create commit");
    
    // Push to the bare repository
    let mut remote = work_repo
        .find_remote("origin")
        .expect("Failed to find origin remote");
    remote
        .push(
            &["refs/heads/main:refs/heads/main"],
            None,
        )
        .expect("Failed to push to bare repository");
    
    // Sync again (should fetch, not clone)
    let sync_cmd2 = SyncCommand::new("fetch-test-app".to_string(), config);
    let result2 = sync_cmd2.execute().await;
    
    // Second sync should also succeed (fetch operation)
    assert!(result2.is_ok(), "Second sync (fetch) should succeed");
    
    // Verify repository still exists
    let home = temp_dir.path();
    let repo_path = home.join(".roro").join("remote").join("fetch-test-app");
    
    assert!(
        repo_path.exists(),
        "Repository should still exist after fetch"
    );
    assert!(
        repo_path.join(".git").exists(),
        ".git directory should still exist after fetch"
    );
    
    // Verify the new commit was fetched by checking the remote tracking branch
    let cloned_repo = Repository::open(&repo_path).expect("Failed to open cloned repo");
    let remote_branch = cloned_repo
        .find_branch("origin/main", git2::BranchType::Remote)
        .expect("Failed to find origin/main branch");
    let remote_commit = remote_branch
        .get()
        .peel_to_commit()
        .expect("Failed to get commit from remote branch");
    
    // Verify the commit message contains "Second commit"
    assert_eq!(
        remote_commit.message(),
        Some("Second commit"),
        "Remote branch should have the second commit"
    );
    
    teardown_test_env(original_home);
}

#[serial]
#[tokio::test]
async fn test_sync_command_multiple_apps() {
    let (temp_dir, original_home) = setup_test_env();
    
    // Create a local test git repository
    let repo_url = create_test_git_repo(&temp_dir);
    
    // Add multiple apps with the same repository (different names)
    let apps = vec![
        ("multi-app-1", repo_url.clone()),
        ("multi-app-2", repo_url),
    ];
    
    for (name, url) in &apps {
        let add_cmd = AddCommand::new(
            name.to_string(),
            url.clone(),
            None,
            None,
            None,
            false,
        );
        add_cmd.execute().await.expect(&format!("Should add {}", name));
    }
    
    // Load config and sync each app
    let config = load_workstation_config()
        .await
        .expect("Failed to load config");
    
    for (name, _) in &apps {
        let sync_cmd = SyncCommand::new(name.to_string(), config.clone());
        let result = sync_cmd.execute().await;
        
        let error_msg = format!("Sync should succeed for {}", name);
        assert!(result.is_ok(), "{}", error_msg);
        
        // Verify repository was cloned to the correct path
        let home = temp_dir.path();
        let expected_path = home.join(".roro").join("remote").join(name);
        
        let repo_msg = format!("Repository for {} should exist", name);
        assert!(
            expected_path.exists(),
            "{}", repo_msg
        );
        let git_msg = format!(".git directory should exist for {}", name);
        assert!(
            expected_path.join(".git").exists(),
            "{}", git_msg
        );
        
        // Verify the test file was cloned
        let file_msg = format!("Test file should exist for {}", name);
        assert!(
            expected_path.join("test.txt").exists(),
            "{}", file_msg
        );
    }
    
    teardown_test_env(original_home);
}

#[serial]
#[tokio::test]
async fn test_sync_command_with_default_path_creates_directories() {
    let (temp_dir, original_home) = setup_test_env();
    
    // Create a local test git repository
    let repo_url = create_test_git_repo(&temp_dir);
    
    // Add an app - directories should be created during sync
    let add_cmd = AddCommand::new(
        "dir-creation-app".to_string(),
        repo_url,
        None,
        None,
        None,
        false,
    );
    add_cmd.execute().await.expect("Should add app successfully");
    
    // Verify .roro/remote directory doesn't exist yet
    let home = temp_dir.path();
    let remote_dir = home.join(".roro").join("remote");
    assert!(!remote_dir.exists(), "Remote directory should not exist before sync");
    
    // Load config and sync
    let config = load_workstation_config()
        .await
        .expect("Failed to load config");
    
    let sync_cmd = SyncCommand::new("dir-creation-app".to_string(), config);
    let result = sync_cmd.execute().await;
    
    assert!(result.is_ok(), "Sync should succeed with local repository");
    
    // Verify directories were created
    assert!(
        remote_dir.exists(),
        "Remote directory should be created during sync"
    );
    
    let repo_path = remote_dir.join("dir-creation-app");
    assert!(
        repo_path.exists(),
        "Repository directory should be created"
    );
    
    // Verify the test file was cloned
    assert!(
        repo_path.join("test.txt").exists(),
        "Test file should exist in cloned repo"
    );
    
    teardown_test_env(original_home);
}