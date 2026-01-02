// Configuration schema definitions
//
// This module contains Rust structs representing the configuration file schemas:
// - app.json: App-level configuration
// - environment.json: Environment-specific configuration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// App configuration schema
///
/// Represents the structure of app.json within each app directory
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct AppConfig {
    /// App metadata
    pub metadata: AppMetadata,
    /// Manifest configuration
    pub manifests: Vec<ManifestConfig>,
    /// Connection configuration (port forwarding, ingress, etc.)
    pub connections: Vec<ConnectionConfig>,
    /// App dependencies
    pub dependencies: Vec<DependencyConfig>,
    /// Deployment strategy
    pub deployment: DeploymentConfig,
    /// Variables configuration
    pub variables: VariablesConfig,
}

/// App metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct AppMetadata {
    /// App name
    pub name: String,
    /// App version
    pub version: String,
    /// App description
    #[serde(default)]
    pub description: String,
}

/// Manifest configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ManifestConfig {
    /// Manifest type (deployment, service, configmap, etc.)
    pub r#type: String,
    /// Path to manifest file relative to app directory
    pub path: String,
    /// Template engine to use (handlebars, tera)
    #[serde(default = "default_template_engine")]
    pub template_engine: String,
}

fn default_template_engine() -> String {
    "handlebars".to_string()
}

/// Connection configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ConnectionConfig {
    /// Connection type (port_forward, ingress, load_balancer)
    pub r#type: String,
    /// Service name
    pub service: String,
    /// Port mapping (local:remote)
    pub port: String,
    /// Additional connection-specific settings
    #[serde(default)]
    pub settings: HashMap<String, String>,
}

/// Dependency configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct DependencyConfig {
    /// App name this depends on
    pub app: String,
    /// Optional namespace for the dependency
    #[serde(default)]
    pub namespace: Option<String>,
}

/// Deployment strategy configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct DeploymentConfig {
    /// Strategy type (rolling, recreate, etc.)
    #[serde(default = "default_deployment_strategy")]
    pub strategy: String,
    /// Namespace to deploy to
    pub namespace: String,
    /// Name suffix for multi-instance deployments
    #[serde(default)]
    pub name_suffix: Option<String>,
}

fn default_deployment_strategy() -> String {
    "rolling".to_string()
}

/// Variables configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct VariablesConfig {
    /// Static variables (key-value pairs)
    #[serde(default)]
    pub static_vars: HashMap<String, String>,
    /// Environment variables (from environment.json)
    #[serde(default)]
    pub env_vars: Vec<String>,
    /// CRD-based variables (JSONPath expressions)
    #[serde(default)]
    pub crd_vars: Vec<CrdVariable>,
}

/// CRD-based variable configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct CrdVariable {
    /// Variable name
    pub name: String,
    /// CRD API version
    pub api_version: String,
    /// CRD kind
    pub kind: String,
    /// JSONPath expression to extract value
    pub jsonpath: String,
    /// Namespace to look for CRD instance
    #[serde(default)]
    pub namespace: Option<String>,
}

/// Environment configuration schema
///
/// Represents the structure of environment.json files
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct EnvironmentConfig {
    /// Environment name (dev, staging, prod, etc.)
    pub name: String,
    /// Environment-specific values
    pub values: HashMap<String, serde_json::Value>,
}

impl AppConfig {
    /// Validate the app configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.metadata.name.is_empty() {
            return Err("metadata.name cannot be empty".to_string());
        }
        if self.metadata.version.is_empty() {
            return Err("metadata.version cannot be empty".to_string());
        }
        if self.manifests.is_empty() {
            return Err("manifests cannot be empty".to_string());
        }
        if self.deployment.namespace.is_empty() {
            return Err("deployment.namespace cannot be empty".to_string());
        }
        Ok(())
    }
}

impl EnvironmentConfig {
    /// Validate the environment configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("name cannot be empty".to_string());
        }
        Ok(())
    }
}

