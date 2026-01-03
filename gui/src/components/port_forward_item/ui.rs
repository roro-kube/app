// UI rendering utilities for port forward item
//
// This module provides utilities for rendering status displays and colors.

use roro_core::api::kubernetes::PortForwardingStatus;

/// Get status display text and color for a port forwarding status
///
/// # Arguments
/// * `status` - Optional port forwarding status
///
/// # Returns
/// A tuple of (`status_text`, `status_color_class`)
pub fn get_status_display(status: Option<&PortForwardingStatus>) -> (&'static str, &'static str) {
    match status {
        None => ("Not Started", "text-gray-600"),
        Some(PortForwardingStatus::Connecting) => ("Connecting...", "text-yellow-600"),
        Some(PortForwardingStatus::Active) => ("Active", "text-green-600"),
        Some(PortForwardingStatus::Failed) => ("Failed", "text-red-600"),
        Some(PortForwardingStatus::Reconnecting) => ("Reconnecting...", "text-yellow-600"),
    }
}
