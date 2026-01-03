// Port forwarding manager tests
//
// This module contains tests for the PortForwardingManager.

#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::single_match,
        clippy::match_wild_err_arm
    )
)]

mod basic;
mod health;
mod instances;
mod ports;

use roro_core::api::kubernetes::{
    portforwarding::PortForwardingManager,
    KubernetesClient,
};

/// Create a test manager for use in tests
pub async fn create_test_manager() -> PortForwardingManager {
    let client = KubernetesClient::new().await;
    match client {
        Ok(c) => PortForwardingManager::new(&c),
        Err(_) => {
            panic!("Failed to create Kubernetes client for tests");
        }
    }
}

