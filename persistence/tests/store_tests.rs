// Persistence store trait tests
//
// Tests for Store trait with mock implementation.
// These tests verify trait method signatures, async behavior, and error handling patterns.

use roro_persistence::{PersistenceError, Store};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Mock entity for testing
#[derive(Debug, Clone)]
struct TestEntity {
    id: String,
    name: String,
}

// Mock Store implementation for testing
struct MockStore {
    data: Arc<Mutex<HashMap<String, TestEntity>>>,
}

impl MockStore {
    fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Store for MockStore {
    async fn create<T>(&self, entity: &T) -> Result<(), PersistenceError>
    where
        T: Send + Sync,
    {
        // This is a simplified mock - in real tests, you'd use proper type conversion
        // For now, we'll just verify the method signature works
        let _ = entity;
        Ok(())
    }

    async fn read<T>(&self, id: &str) -> Result<Option<T>, PersistenceError>
    where
        T: Send + Sync,
    {
        let _ = id;
        // Mock returns None (not found)
        Ok(None)
    }

    async fn update<T>(&self, id: &str, entity: &T) -> Result<(), PersistenceError>
    where
        T: Send + Sync,
    {
        let _ = (id, entity);
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), PersistenceError> {
        let _ = id;
        Ok(())
    }

    async fn list<T>(&self) -> Result<Vec<T>, PersistenceError>
    where
        T: Send + Sync,
    {
        // Mock returns empty list
        Ok(Vec::new())
    }
}

#[tokio::test]
async fn test_store_trait_create() {
    let store = MockStore::new();
    let entity = TestEntity {
        id: "test-1".to_string(),
        name: "Test".to_string(),
    };
    
    let result = store.create(&entity).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_store_trait_read() {
    let store = MockStore::new();
    
    let result: Result<Option<TestEntity>, PersistenceError> = store.read("test-id").await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_store_trait_update() {
    let store = MockStore::new();
    let entity = TestEntity {
        id: "test-1".to_string(),
        name: "Updated".to_string(),
    };
    
    let result = store.update("test-1", &entity).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_store_trait_delete() {
    let store = MockStore::new();
    
    let result = store.delete("test-id").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_store_trait_list() {
    let store = MockStore::new();
    
    let result: Result<Vec<TestEntity>, PersistenceError> = store.list().await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[tokio::test]
async fn test_store_trait_async_behavior() {
    // Verify that all methods are async and can be awaited
    let store = MockStore::new();
    
    // All operations should complete without blocking
    let _ = store.create(&TestEntity { id: "1".to_string(), name: "Test".to_string() }).await;
    let _ = store.read::<TestEntity>("1").await;
    let _ = store.update("1", &TestEntity { id: "1".to_string(), name: "Updated".to_string() }).await;
    let _ = store.delete("1").await;
    let _ = store.list::<TestEntity>().await;
    
    // If we get here, all async operations completed
    assert!(true);
}

#[tokio::test]
async fn test_store_trait_error_handling_pattern() {
    // Verify that errors are properly typed
    let store = MockStore::new();
    
    let result: Result<Option<TestEntity>, PersistenceError> = store.read("nonexistent").await;
    
    match result {
        Ok(option) => {
            // Should return None for not found
            assert!(option.is_none());
        }
        Err(e) => {
            // If error occurs, it should be PersistenceError
            let _error_msg = format!("{}", e);
            assert!(true);
        }
    }
}

// Test that Store implementation is Send + Sync (required for trait)
#[test]
fn test_store_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    
    // MockStore should be Send + Sync
    assert_send::<MockStore>();
    assert_sync::<MockStore>();
    
    // Store trait requires Send + Sync bounds
    // Note: Store trait itself is not object-safe due to generic methods,
    // but implementations must be Send + Sync
}

