// Port forwarding types
//
// This module defines the types used for port forwarding configuration and state.

use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct PortForwardingConfig {
    pub namespace: String,
    pub pod: String,
    pub remote_port: u16,
    pub local_port: u16,
    pub instance_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortForwardingStatus {
    Connecting,
    Active,
    Failed,
    Reconnecting,
}

#[derive(Debug, Clone)]
pub struct PortForwardingState {
    pub id: String,
    pub config: PortForwardingConfig,
    pub status: PortForwardingStatus,
    pub last_health_check: Option<SystemTime>,
    pub retry_count: u32,
}
