// Health checking for port forwarding
//
// This module provides health checking functionality for port forwarding connections.

use crate::api::kubernetes::portforwarding::types::{PortForwardingState, PortForwardingStatus};
use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Check if a port forward is healthy by attempting to connect to the local port
pub async fn health_check_forward(
    forwards: &Arc<RwLock<HashMap<String, PortForwardingState>>>,
    forward_id: &str,
) -> bool {
    let state = {
        let f = forwards.read().await;
        f.get(forward_id).cloned()
    };

    if let Some(state) = state {
        if state.status != PortForwardingStatus::Active {
            return false;
        }

        if TcpStream::connect(format!("127.0.0.1:{}", state.config.local_port)).is_ok() {
            return true;
        }
    }

    false
}
