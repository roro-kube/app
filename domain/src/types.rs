// Domain layer types and models
// This module defines domain entities and value objects.

use crate::errors::DomainError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
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

/// Port value that can be either a numeric port or a named port string
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortValue {
    /// Numeric port number
    Numeric(u16),
    /// Named port (e.g., "prometheus", "http")
    Named(String),
}

impl Serialize for PortValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            PortValue::Numeric(n) => serializer.serialize_u16(*n),
            PortValue::Named(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> Deserialize<'de> for PortValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::Number(n) => {
                if let Some(u) = n.as_u64() {
                    if u <= u16::MAX as u64 {
                        Ok(PortValue::Numeric(u as u16))
                    } else {
                        Err(serde::de::Error::custom(format!(
                            "Port number {} exceeds maximum value {}",
                            u, u16::MAX
                        )))
                    }
                } else {
                    Err(serde::de::Error::custom("Port number must be a valid u16"))
                }
            }
            Value::String(s) => Ok(PortValue::Named(s)),
            _ => Err(serde::de::Error::custom(
                "Port must be either a number or a string",
            )),
        }
    }
}

/// Port forwarding configuration for a Kubernetes service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortForwardConfig {
    /// Local port to forward to
    #[serde(rename = "localport")]
    pub local_port: String,
    /// Name of the Kubernetes service
    pub name: String,
    /// Port on the service (can be numeric or named)
    pub port: PortValue,
    /// Kind of Kubernetes resource (e.g., "service")
    pub kind: String,
}

impl PortForwardConfig {
    /// Validate the port forward configuration
    ///
    /// # Errors
    /// Returns `DomainError::PortForwardValidation` if validation fails
    pub fn validate(&self) -> Result<(), DomainError> {
        // Validate local_port is not empty
        if self.local_port.is_empty() {
            return Err(DomainError::PortForwardValidation(
                "localport cannot be empty".to_string(),
            ));
        }

        // Validate local_port is a valid port number if it's numeric
        if let Ok(port_num) = self.local_port.parse::<u16>() {
            if port_num == 0 {
                return Err(DomainError::PortForwardValidation(
                    "localport cannot be 0".to_string(),
                ));
            }
        } else {
            return Err(DomainError::PortForwardValidation(format!(
                "localport '{}' must be a valid port number (1-65535)",
                self.local_port
            )));
        }

        // Validate name is not empty
        if self.name.is_empty() {
            return Err(DomainError::PortForwardValidation(
                "name cannot be empty".to_string(),
            ));
        }

        // Validate kind is not empty
        if self.kind.is_empty() {
            return Err(DomainError::PortForwardValidation(
                "kind cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

/// Application configuration structure
/// Represents a single app configuration that can be stored in a Git repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Name of the application
    pub name: String,
    /// Description of the application
    pub description: String,
    /// Path to Kubernetes manifests directory (relative to app config location)
    #[serde(rename = "manifestsPath")]
    pub manifests_path: String,
    /// Port forwarding configurations for this app
    #[serde(rename = "portForward", default)]
    pub port_forward: Vec<PortForwardConfig>,
}

impl AppConfig {
    /// Validate the app configuration
    ///
    /// # Errors
    /// Returns `DomainError::AppConfigValidation` if validation fails
    pub fn validate(&self) -> Result<(), DomainError> {
        // Validate name is not empty
        if self.name.is_empty() {
            return Err(DomainError::AppConfigValidation(
                "name cannot be empty".to_string(),
            ));
        }

        // Validate manifests_path is not empty
        if self.manifests_path.is_empty() {
            return Err(DomainError::AppConfigValidation(
                "manifestsPath cannot be empty".to_string(),
            ));
        }

        // Validate all port forward configurations
        for (index, pf) in self.port_forward.iter().enumerate() {
            pf.validate().map_err(|e| {
                DomainError::AppConfigValidation(format!(
                    "portForward[{}]: {}",
                    index,
                    match e {
                        DomainError::PortForwardValidation(msg) => msg,
                        _ => "validation failed".to_string(),
                    }
                ))
            })?;
        }

        Ok(())
    }
}
