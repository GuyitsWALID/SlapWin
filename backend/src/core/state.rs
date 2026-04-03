use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

use crate::models::{AdapterInfo, AdapterStatus};

/// A saved MAC profile for an adapter.
/// Profiles persist the user's choice even across restarts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacProfile {
    pub device_id: String,
    pub adapter_name: String,
    /// The original (factory-burned) MAC address.
    pub original_mac: String,
    /// The custom/random MAC the user wants to apply.
    pub target_mac: String,
    /// Whether the profile is currently active (MAC is applied).
    pub active: bool,
    /// Timestamp when the profile was last modified.
    pub created_at: String,
}

/// Shared application state persisted across Tauri IPC calls.
pub struct AppState {
    /// Cache of discovered adapters and their profiles.
    pub adapters: Mutex<Vec<AdapterInfo>>,

    /// Persisted MAC profiles keyed by device_id.
    pub profiles: Mutex<HashMap<String, MacProfile>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            adapters: Mutex::new(Vec::new()),
            profiles: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
