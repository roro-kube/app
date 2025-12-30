// Domain layer types and models
// This module defines domain entities and value objects.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Domain entity representing a business object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEntity {
    pub id: String,
    pub name: String,
    pub state: EntityState,
}

/// State of a domain entity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityState {
    Pending,
    Processing,
    Complete,
    Failed,
}

/// Context for processing domain operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingContext {
    pub context_id: String,
    pub entity_name: String,
    pub state: HashMap<String, serde_json::Value>,
    pub logs: Vec<String>,
}

/// Result of a processing operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingResult {
    pub entity_id: String,
    pub state: EntityState,
    pub output: HashMap<String, serde_json::Value>,
}

