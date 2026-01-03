// Application configuration
//
// This module defines the AppConfig type and its validation.

use crate::errors::DomainError;
use crate::types::port_forwarding::PortForwardingConfig;
use serde::{Deserialize, Serialize};

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
    #[serde(rename = "portForwarding", default)]
    pub port_forwarding: Vec<PortForwardingConfig>,
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

        // Validate all port forwarding configurations
        for (index, pf) in self.port_forwarding.iter().enumerate() {
            pf.validate().map_err(|e| {
                DomainError::AppConfigValidation(format!(
                    "portForwarding[{}]: {}",
                    index,
                    match e {
                        DomainError::PortForwardingValidation(msg) => msg,
                        _ => "validation failed".to_string(),
                    }
                ))
            })?;
        }

        Ok(())
    }
}
