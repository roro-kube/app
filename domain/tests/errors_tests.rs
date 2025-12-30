// Domain errors tests
//
// Tests for DomainError enum variants, error message formatting, and error conversion.

use roro_domain::DomainError;

#[test]
fn test_domain_error_processing() {
    let error = DomainError::Processing("Test processing error".to_string());
    let error_msg = format!("{}", error);
    
    assert!(error_msg.contains("Processing error"));
    assert!(error_msg.contains("Test processing error"));
}

#[test]
fn test_domain_error_validation() {
    let error = DomainError::Validation("Invalid input".to_string());
    let error_msg = format!("{}", error);
    
    assert!(error_msg.contains("Validation error"));
    assert!(error_msg.contains("Invalid input"));
}

#[test]
fn test_domain_error_handler_not_found() {
    let error = DomainError::HandlerNotFound("unknown-handler".to_string());
    let error_msg = format!("{}", error);
    
    assert!(error_msg.contains("Handler not found"));
    assert!(error_msg.contains("unknown-handler"));
}

#[test]
fn test_domain_error_invalid_state() {
    let error = DomainError::InvalidState("Entity is in wrong state".to_string());
    let error_msg = format!("{}", error);
    
    assert!(error_msg.contains("Invalid entity state"));
    assert!(error_msg.contains("Entity is in wrong state"));
}

#[test]
fn test_domain_error_timeout() {
    let error = DomainError::Timeout("Operation timed out after 30s".to_string());
    let error_msg = format!("{}", error);
    
    assert!(error_msg.contains("Operation timeout"));
    assert!(error_msg.contains("Operation timed out after 30s"));
}

#[test]
fn test_domain_error_all_variants() {
    let errors = vec![
        DomainError::Processing("test".to_string()),
        DomainError::Validation("test".to_string()),
        DomainError::HandlerNotFound("test".to_string()),
        DomainError::InvalidState("test".to_string()),
        DomainError::Timeout("test".to_string()),
    ];
    
    for error in errors {
        let msg = format!("{}", error);
        assert!(!msg.is_empty());
        assert!(msg.contains("test"));
    }
}

#[test]
fn test_domain_error_debug() {
    let error = DomainError::Processing("Test error".to_string());
    let debug_str = format!("{:?}", error);
    
    assert!(debug_str.contains("Processing"));
    assert!(debug_str.contains("Test error"));
}

#[test]
fn test_domain_error_display_vs_debug() {
    let error = DomainError::Validation("Test validation".to_string());
    
    let display = format!("{}", error);
    let debug = format!("{:?}", error);
    
    // Display should be user-friendly
    assert!(display.contains("Validation error"));
    assert!(display.contains("Test validation"));
    
    // Debug should include more details
    assert!(debug.contains("Validation"));
}

