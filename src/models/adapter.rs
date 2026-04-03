use serde::{Deserialize, Serialize};
use std::net::MacAddr;

/// Represents the operational state of a network adapter.
/// Maps roughly to IF_OPER_STATUS on Windows.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AdapterStatus {
    Up,
    Down,
    Testing,
    Unknown,
    Dormant,
    NotPresent,
    LowerLayerDown,
}

impl AdapterStatus {
    /// Human-friendly label for the adapter status.
    pub fn label(&self) -> &str {
        match self {
            Self::Up => "Connected",
            Self::Down => "Disconnected",
            Self::Testing => "Testing",
            Self::Unknown => "Unknown",
            Self::Dormant => "Dormant",
            Self::NotPresent => "Not Present",
            Self::LowerLayerDown => "Lower Layer Down",
        }
    }
}

/// Full network adapter information exposed to Tauri IPC.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterInfo {
    /// Unique device instance path used as the stable identifier for this adapter.
    /// Example format on Windows: "PCI\\VEN_8086&DEV_15B8&..."  
    /// This is preferred over the GUID because it does not change across reboots.
    pub device_id: String,

    /// Human-readable name of the adapter (the "friendly name" shown in GUI).
    /// e.g. "Intel(R) Wi-Fi 6 AX200 160MHz", "Ethernet", etc.
    pub name: String,

    /// Currently active MAC address (as read from the OS).
    /// Returns the *effective* MAC, which may already be a spoofed one.
    pub current_mac: String,

    /// The original MAC address burned into the adapter (NIC).
    /// This is the value we use to restore the adapter to factory state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_mac: Option<String>,

    /// Operational status of the adapter (up/down/etc.).
    pub status: AdapterStatus,

    /// The Windows "NetworkAddress" registry value currently set for spoofing,
    /// if any.  Empty string = no spoof (factory MAC in use).
    /// This field is only populated when explicitly queried from the registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoofed_mac: Option<String>,

    /// Whether this adapter supports MAC address spoofing.
    /// Most physical adapters do; some virtual adapters (Hyper-V, VPN) may not.
    pub supports_spoofing: bool,
}

impl Default for AdapterStatus {
    fn default() -> Self {
        Self::Unknown
    }
}
