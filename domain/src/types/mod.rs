// Domain layer types and models
//
// This module defines domain entities and value objects.

mod app_config;
mod entity;
mod port;
mod port_forwarding;

pub use app_config::AppConfig;
pub use entity::{DomainEntity, EntityState, ProcessingContext, ProcessingResult};
pub use port::PortValue;
pub use port_forwarding::PortForwardingConfig;
