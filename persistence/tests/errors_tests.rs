// Persistence errors tests
//
// Tests for PersistenceError enum variants, error message formatting, and error conversion.

use roro_persistence::PersistenceError;

#[test]
fn test_persistence_error_database() {
    let error = PersistenceError::Database("Connection failed".to_string());
    let error_msg = format!("{}", error);
    
    assert!(error_msg.contains("Database error"));
    assert!(error_msg.contains("Connection failed"));
}

#[test]
fn test_persistence_error_not_found() {
    let error = PersistenceError::NotFound("entity-123".to_string());
    let error_msg = format!("{}", error);
    
    assert!(error_msg.contains("Not found"));
    assert!(error_msg.contains("entity-123"));
}

#[test]
fn test_persistence_error_invalid_input() {
    let error = PersistenceError::InvalidInput("Invalid JSON format".to_string());
    let error_msg = format!("{}", error);
    
    assert!(error_msg.contains("Invalid input"));
    assert!(error_msg.contains("Invalid JSON format"));
}

#[test]
fn test_persistence_error_serialization() {
    let error = PersistenceError::Serialization("Deserialization failed".to_string());
    let error_msg = format!("{}", error);
    
    assert!(error_msg.contains("Serialization error"));
    assert!(error_msg.contains("Deserialization failed"));
}

#[test]
fn test_persistence_error_all_variants() {
    let errors = vec![
        PersistenceError::Database("test".to_string()),
        PersistenceError::NotFound("test".to_string()),
        PersistenceError::InvalidInput("test".to_string()),
        PersistenceError::Serialization("test".to_string()),
    ];
    
    for error in errors {
        let msg = format!("{}", error);
        assert!(!msg.is_empty());
        assert!(msg.contains("test"));
    }
}

#[test]
fn test_persistence_error_debug() {
    let error = PersistenceError::Database("Test error".to_string());
    let debug_str = format!("{:?}", error);
    
    assert!(debug_str.contains("Database"));
    assert!(debug_str.contains("Test error"));
}

#[test]
fn test_persistence_error_display_vs_debug() {
    let error = PersistenceError::NotFound("entity-123".to_string());
    
    let display = format!("{}", error);
    let debug = format!("{:?}", error);
    
    // Display should be user-friendly
    assert!(display.contains("Not found"));
    assert!(display.contains("entity-123"));
    
    // Debug should include more details
    assert!(debug.contains("NotFound"));
}

#[test]
fn test_persistence_error_error_messages() {
    // Test that error messages are descriptive
    let errors = vec![
        (PersistenceError::Database("Connection timeout".to_string()), "Database error", "Connection timeout"),
        (PersistenceError::NotFound("user-456".to_string()), "Not found", "user-456"),
        (PersistenceError::InvalidInput("Missing required field".to_string()), "Invalid input", "Missing required field"),
        (PersistenceError::Serialization("Invalid UTF-8".to_string()), "Serialization error", "Invalid UTF-8"),
    ];
    
    for (error, expected_prefix, expected_detail) in errors {
        let msg = format!("{}", error);
        assert!(msg.contains(expected_prefix));
        assert!(msg.contains(expected_detail));
    }
}

