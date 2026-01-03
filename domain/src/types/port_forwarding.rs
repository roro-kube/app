// Port forwarding configuration
//
// This module defines the PortForwardingConfig type and its validation.

use crate::errors::DomainError;
use crate::types::port::PortValue;
use serde::{Deserialize, Serialize};

/// Port forwarding configuration for a Kubernetes service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortForwardingConfig {
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

impl PortForwardingConfig {
    /// Validate the port forwarding configuration
    ///
    /// # Errors
    /// Returns `DomainError::PortForwardingValidation` if validation fails
    pub fn validate(&self) -> Result<(), DomainError> {
        // Validate local_port is not empty
        if self.local_port.is_empty() {
            return Err(DomainError::PortForwardingValidation(
                "localport cannot be empty".to_string(),
            ));
        }

        // Validate local_port is a valid port number if it's numeric
        if let Ok(port_num) = self.local_port.parse::<u16>() {
            if port_num == 0 {
                return Err(DomainError::PortForwardingValidation(
                    "localport cannot be 0".to_string(),
                ));
            }
        } else {
            return Err(DomainError::PortForwardingValidation(format!(
                "localport '{}' must be a valid port number (1-65535)",
                self.local_port
            )));
        }

        // Validate name is not empty
        if self.name.is_empty() {
            return Err(DomainError::PortForwardingValidation(
                "name cannot be empty".to_string(),
            ));
        }

        // Validate kind is not empty
        if self.kind.is_empty() {
            return Err(DomainError::PortForwardingValidation(
                "kind cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}
