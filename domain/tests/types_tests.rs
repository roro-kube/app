// Domain types tests
//
// Tests for DomainEntity, EntityState, ProcessingContext, and ProcessingResult.

use roro_domain::{DomainEntity, EntityState, ProcessingContext, ProcessingResult};
use serde_json::json;
use std::collections::HashMap;

#[test]
fn test_domain_entity_creation() {
    let entity = DomainEntity {
        id: "test-id".to_string(),
        name: "Test Entity".to_string(),
        state: EntityState::Pending,
    };
    
    assert_eq!(entity.id, "test-id");
    assert_eq!(entity.name, "Test Entity");
    assert_eq!(entity.state, EntityState::Pending);
}

#[test]
fn test_domain_entity_serialization() {
    let entity = DomainEntity {
        id: "test-id".to_string(),
        name: "Test Entity".to_string(),
        state: EntityState::Complete,
    };
    
    let serialized = serde_json::to_string(&entity).unwrap();
    assert!(serialized.contains("test-id"));
    assert!(serialized.contains("Test Entity"));
    assert!(serialized.contains("Complete"));
}

#[test]
fn test_domain_entity_deserialization() {
    let json = r#"{"id":"test-id","name":"Test Entity","state":"Pending"}"#;
    let entity: DomainEntity = serde_json::from_str(json).unwrap();
    
    assert_eq!(entity.id, "test-id");
    assert_eq!(entity.name, "Test Entity");
    assert_eq!(entity.state, EntityState::Pending);
}

#[test]
fn test_entity_state_variants() {
    let states = vec![
        EntityState::Pending,
        EntityState::Processing,
        EntityState::Complete,
        EntityState::Failed,
    ];
    
    for state in states {
        let serialized = serde_json::to_string(&state).unwrap();
        let deserialized: EntityState = serde_json::from_str(&serialized).unwrap();
        assert_eq!(state, deserialized);
    }
}

#[test]
fn test_entity_state_equality() {
    assert_eq!(EntityState::Pending, EntityState::Pending);
    assert_eq!(EntityState::Complete, EntityState::Complete);
    assert_ne!(EntityState::Pending, EntityState::Complete);
    assert_ne!(EntityState::Processing, EntityState::Failed);
}

#[test]
fn test_processing_context_initialization() {
    let context = ProcessingContext {
        context_id: "ctx-123".to_string(),
        entity_name: "Test Entity".to_string(),
        state: HashMap::new(),
        logs: Vec::new(),
    };
    
    assert_eq!(context.context_id, "ctx-123");
    assert_eq!(context.entity_name, "Test Entity");
    assert!(context.state.is_empty());
    assert!(context.logs.is_empty());
}

#[test]
fn test_processing_context_state_management() {
    let mut context = ProcessingContext {
        context_id: "ctx-123".to_string(),
        entity_name: "Test Entity".to_string(),
        state: HashMap::new(),
        logs: Vec::new(),
    };
    
    context.state.insert("key1".to_string(), json!("value1"));
    context.state.insert("key2".to_string(), json!(42));
    
    assert_eq!(context.state.len(), 2);
    assert_eq!(context.state.get("key1"), Some(&json!("value1")));
    assert_eq!(context.state.get("key2"), Some(&json!(42)));
}

#[test]
fn test_processing_context_logs() {
    let mut context = ProcessingContext {
        context_id: "ctx-123".to_string(),
        entity_name: "Test Entity".to_string(),
        state: HashMap::new(),
        logs: Vec::new(),
    };
    
    context.logs.push("Log entry 1".to_string());
    context.logs.push("Log entry 2".to_string());
    
    assert_eq!(context.logs.len(), 2);
    assert_eq!(context.logs[0], "Log entry 1");
    assert_eq!(context.logs[1], "Log entry 2");
}

#[test]
fn test_processing_result_creation() {
    let mut output = HashMap::new();
    output.insert("result".to_string(), json!("success"));
    
    let result = ProcessingResult {
        entity_id: "entity-123".to_string(),
        state: EntityState::Complete,
        output,
    };
    
    assert_eq!(result.entity_id, "entity-123");
    assert_eq!(result.state, EntityState::Complete);
    assert_eq!(result.output.len(), 1);
}

#[test]
fn test_processing_result_serialization() {
    let mut output = HashMap::new();
    output.insert("result".to_string(), json!("success"));
    output.insert("count".to_string(), json!(5));
    
    let result = ProcessingResult {
        entity_id: "entity-123".to_string(),
        state: EntityState::Complete,
        output,
    };
    
    let serialized = serde_json::to_string(&result).unwrap();
    assert!(serialized.contains("entity-123"));
    assert!(serialized.contains("Complete"));
    assert!(serialized.contains("success"));
    assert!(serialized.contains("5"));
}

#[test]
fn test_processing_result_deserialization() {
    let json = r#"{"entity_id":"entity-123","state":"Complete","output":{"result":"success"}}"#;
    let result: ProcessingResult = serde_json::from_str(json).unwrap();
    
    assert_eq!(result.entity_id, "entity-123");
    assert_eq!(result.state, EntityState::Complete);
    assert_eq!(result.output.get("result"), Some(&json!("success")));
}

#[test]
fn test_processing_context_serialization() {
    let mut state = HashMap::new();
    state.insert("key".to_string(), json!("value"));
    
    let context = ProcessingContext {
        context_id: "ctx-123".to_string(),
        entity_name: "Test Entity".to_string(),
        state,
        logs: vec!["log1".to_string(), "log2".to_string()],
    };
    
    let serialized = serde_json::to_string(&context).unwrap();
    assert!(serialized.contains("ctx-123"));
    assert!(serialized.contains("Test Entity"));
    assert!(serialized.contains("log1"));
}

