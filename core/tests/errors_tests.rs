// Core layer error tests
//
// Tests for CoreError enum variants, error transformations, and display formatting.

use roro_core::CoreError;
use roro_domain::DomainError;
use roro_persistence::PersistenceError;

#[test]
fn test_core_error_domain_conversion() {
    let domain_error = DomainError::Processing("Test processing error".to_string());
    let core_error: CoreError = domain_error.into();

    match core_error {
        CoreError::Domain(e) => {
            match e {
                DomainError::Processing(msg) => assert_eq!(msg, "Test processing error"),
                _ => panic!("Expected Processing variant"),
            }
        }
        _ => panic!("Expected Domain variant"),
    }
}

#[test]
fn test_core_error_persistence_conversion() {
    let persistence_error = PersistenceError::NotFound("test-id".to_string());
    let core_error: CoreError = persistence_error.into();

    match core_error {
        CoreError::Persistence(e) => {
            match e {
                PersistenceError::NotFound(id) => assert_eq!(id, "test-id"),
                _ => panic!("Expected NotFound variant"),
            }
        }
        _ => panic!("Expected Persistence variant"),
    }
}

#[test]
fn test_core_error_validation() {
    let error = CoreError::Validation("Invalid input format".to_string());
    let error_msg = format!("{}", error);
    assert!(error_msg.contains("Validation error"));
    assert!(error_msg.contains("Invalid input format"));
}

#[test]
fn test_core_error_bridge() {
    let error = CoreError::Bridge("Transformation failed".to_string());
    let error_msg = format!("{}", error);
    assert!(error_msg.contains("Bridge error"));
    assert!(error_msg.contains("Transformation failed"));
}

#[test]
fn test_core_error_display_formatting() {
    // Test Domain error display
    let domain_error = DomainError::HandlerNotFound("unknown-handler".to_string());
    let core_error: CoreError = domain_error.into();
    let msg = format!("{}", core_error);
    assert!(msg.contains("Domain error"));
    assert!(msg.contains("Handler not found"));

    // Test Persistence error display
    let persistence_error = PersistenceError::Database("Connection failed".to_string());
    let core_error: CoreError = persistence_error.into();
    let msg = format!("{}", core_error);
    assert!(msg.contains("Persistence error"));
    assert!(msg.contains("Database error"));
}

#[test]
fn test_core_error_all_variants() {
    // Test all CoreError variants can be created and formatted
    let errors = vec![
        CoreError::Validation("test".to_string()),
        CoreError::Bridge("test".to_string()),
        CoreError::Domain(DomainError::Processing("test".to_string())),
        CoreError::Persistence(PersistenceError::NotFound("test".to_string())),
    ];

    for error in errors {
        let msg = format!("{}", error);
        assert!(!msg.is_empty());
    }
}

