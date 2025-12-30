// Persistence layer store trait
// This module defines the Store trait that abstracts database operations.

use async_trait::async_trait;

use crate::errors::PersistenceError;

/// Store trait for database operations
/// 
/// This trait provides an abstraction over database operations, allowing
/// different database implementations (SQLite, PostgreSQL, etc.) to be used
/// interchangeably. All operations are async and return Result types.
#[async_trait]
pub trait Store: Send + Sync {
    /// Create a new entity
    /// 
    /// # Arguments
    /// * `entity` - The entity to create
    /// 
    /// # Returns
    /// * `Ok(())` if the entity was created successfully
    /// * `Err(PersistenceError)` if creation failed
    async fn create<T>(&self, entity: &T) -> Result<(), PersistenceError>
    where
        T: Send + Sync;

    /// Read an entity by ID
    /// 
    /// # Arguments
    /// * `id` - The ID of the entity to read
    /// 
    /// # Returns
    /// * `Ok(Some(entity))` if the entity was found
    /// * `Ok(None)` if the entity was not found
    /// * `Err(PersistenceError)` if the read operation failed
    async fn read<T>(&self, id: &str) -> Result<Option<T>, PersistenceError>
    where
        T: Send + Sync;

    /// Update an existing entity
    /// 
    /// # Arguments
    /// * `id` - The ID of the entity to update
    /// * `entity` - The updated entity data
    /// 
    /// # Returns
    /// * `Ok(())` if the entity was updated successfully
    /// * `Err(PersistenceError)` if update failed
    async fn update<T>(&self, id: &str, entity: &T) -> Result<(), PersistenceError>
    where
        T: Send + Sync;

    /// Delete an entity by ID
    /// 
    /// # Arguments
    /// * `id` - The ID of the entity to delete
    /// 
    /// # Returns
    /// * `Ok(())` if the entity was deleted successfully
    /// * `Err(PersistenceError)` if deletion failed
    async fn delete(&self, id: &str) -> Result<(), PersistenceError>;

    /// List all entities
    /// 
    /// # Returns
    /// * `Ok(Vec<entity>)` if the list operation succeeded
    /// * `Err(PersistenceError)` if the list operation failed
    async fn list<T>(&self) -> Result<Vec<T>, PersistenceError>
    where
        T: Send + Sync;
}

