// Git credential handling
//
// This module provides functionality for creating Git credential callbacks.

use git2::{Cred, RemoteCallbacks};

/// Create remote callbacks with credential handling
///
/// # Arguments
/// * `creds` - Optional credentials tuple (username, password)
///
/// # Returns
/// A `RemoteCallbacks` instance configured with credential handling
pub fn create_callbacks(creds: Option<(String, String)>) -> RemoteCallbacks<'static> {
    let mut callbacks = RemoteCallbacks::new();
    if let Some((username, password)) = creds {
        callbacks.credentials(move |_url, username_from_url, _allowed_types| {
            let username = username_from_url.unwrap_or(&username);
            if let Ok(cred) = Cred::userpass_plaintext(username, &password) {
                return Ok(cred);
            }
            if let Ok(cred) = Cred::ssh_key_from_agent(username) {
                return Ok(cred);
            }
            Cred::default()
        });
    } else {
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            let username = username_from_url.unwrap_or("git");
            if let Ok(cred) = Cred::ssh_key_from_agent(username) {
                return Ok(cred);
            }
            Cred::default()
        });
    }
    callbacks
}
