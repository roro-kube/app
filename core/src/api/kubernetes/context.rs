use crate::errors::CoreError;
use kube::config::{Kubeconfig, KubeconfigError};
use std::path::PathBuf;

pub struct ContextManager;

impl ContextManager {
    /// # Errors
    /// Returns an error if the kubeconfig file cannot be read or parsed
    pub fn load_kubeconfig() -> Result<Kubeconfig, CoreError> {
        let kubeconfig_path = Self::default_kubeconfig_path()?;
        Self::load_kubeconfig_from_path(&kubeconfig_path)
    }

    /// # Errors
    /// Returns an error if the kubeconfig file cannot be read or parsed
    pub fn load_kubeconfig_from_path(path: &PathBuf) -> Result<Kubeconfig, CoreError> {
        let original_kubeconfig = std::env::var("KUBECONFIG").ok();
        std::env::set_var("KUBECONFIG", path);

        let result = Kubeconfig::read().map_err(|e| match e {
            KubeconfigError::ReadConfig(io_err, _path) => {
                CoreError::Kubeconfig(format!("Failed to read kubeconfig file: {io_err}"))
            }
            _ => CoreError::Kubeconfig(format!("Kubeconfig error: {e}")),
        });

        if let Some(original) = original_kubeconfig {
            std::env::set_var("KUBECONFIG", original);
        } else {
            std::env::remove_var("KUBECONFIG");
        }

        result
    }

    /// # Errors
    /// Returns an error if the home directory cannot be determined
    pub fn default_kubeconfig_path() -> Result<PathBuf, CoreError> {
        if let Ok(path) = std::env::var("KUBECONFIG") {
            return Ok(PathBuf::from(path));
        }

        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .map_err(|_| {
                CoreError::Kubeconfig(
                    "Unable to determine home directory for default kubeconfig path".to_string(),
                )
            })?;

        let mut path = PathBuf::from(home);
        path.push(".kube");
        path.push("config");

        Ok(path)
    }

    /// # Errors
    /// Returns an error if the kubeconfig cannot be loaded
    pub fn list_contexts() -> Result<Vec<String>, CoreError> {
        let kubeconfig = Self::load_kubeconfig()?;
        Ok(kubeconfig
            .contexts
            .iter()
            .map(|ctx| ctx.name.clone())
            .collect())
    }

    /// # Errors
    /// Returns an error if:
    /// - The kubeconfig cannot be loaded
    /// - No current context is set in kubeconfig
    pub fn current_context_name() -> Result<String, CoreError> {
        let kubeconfig = Self::load_kubeconfig()?;
        kubeconfig.current_context.ok_or_else(|| {
            CoreError::Kubeconfig("No current context set in kubeconfig".to_string())
        })
    }

    /// # Errors
    /// Returns an error if the context is not found in the kubeconfig
    pub fn validate_context(context_name: &str) -> Result<(), CoreError> {
        let kubeconfig = Self::load_kubeconfig()?;
        let context_exists = kubeconfig
            .contexts
            .iter()
            .any(|ctx| ctx.name == context_name);

        if context_exists {
            Ok(())
        } else {
            Err(CoreError::ContextNotFound(format!(
                "Context '{context_name}' not found in kubeconfig"
            )))
        }
    }
}
