//! MAC address changing implementation for Windows.
//!
//! Uses two mechanisms:
//!   1. Win32 APIs to enumerate adapters and read current MACs.
//!   2. Registry modifications to apply spoofing (HKLM\...\NetworkAddress).
//!
//! The registry-based method works for most physical Ethernet/Wi-Fi adapters.
//! Virtual adapters (e.g. Hyper-V, VPNs) often ignore NetworkAddress or
//! may not support it at all.
//!
//! Changes require a reboot or adapter restart to fully take effect.  We
//! write the desired MAC to the adapter's registry "DriverDesc" key under
//! System\CurrentControlSet\Control\Class.

use std::collections::{HashMap, HashSet};
use std::ffi::CString;
use std::fmt;
use std::path::PathBuf;
use std::sync::LazyLock;
use thiserror::Error;
use windows::core::{PCWSTR, PWSTR, HSTRING};
use windows::Win32::Foundation::{CloseHandle, HANDLE, BOOL};
use windows::Win32::NetworkManagement::IpHelper::{GetAdaptersInfo, GetIfTable2};
use windows::Win32::NetworkManagement::Ndis::{NdisOpenAdapter, NdisCloseAdapter, NdisSetVariable, NDIS_OID, NDIS_STATUS_SUCCESS, NDIS_REQUEST_SOURCE};
use windows::Win32::NetworkManagement::Ndis::{NdisOidRequest, NDIS_REQUEST_TYPE, NDIS_OBJECT_TYPE_REQUEST, NdisRequestGenericClass, NdisRequest, NdisRequestGeneric, NdisRequestSetInformation, NdisRequestQueryInformation, NDIS_REQUEST_FLAGS_FLAGS_MUST_COMPLETE, NDIS_REQUEST_FLAGS_FLAGS_DO_NOT_WAIT, NDIS_FLAGS, NdisCancelOidRequest, g_pNdisOpenAdapter, g_pNdisCloseAdapter, g_pNdisSetVariable, NdisOidRequestTimeout, NdisOidRequestComplete, NdisOidRequest, NdisOpenAdapterChinese, NdisCloseAdapterChinese, NdisOidRequestChinese, NdisSetVariableChinese};
use windows::Win32::NetworkManagement::Ndis::{NdisOidInformation, NdisOidInformationChinese};
use windows::Win32::NetworkManagement::IpHelper::{
    MIB_IFROW, MIB_IFROW2, GetIfEntry2, GetIfTable2, MIB_IF_TABLE2, 
};
use windows::Win32::NetworkManagement::Ndis::{
    NdisOidGetCurrentMacAddress, NdisOidGetPermanentAddress, NdisOidGetRssCapabilities,
};
use windows::Win32::System::Registry::{
    RegOpenKeyExW, RegSetValueExW, RegQueryValueExW, RegCloseKey, RegDeleteValueW,
    HKEY_LOCAL_MACHINE, HKEY, REG_SZ, REG_MULTI_SZ, KEY_WRITE, KEY_QUERY_VALUE, KEY_READ, KEY_SET_VALUE, KEY_QUERY_VALUE, KEY(Win32::System::Registry::KEY_SET_VALUE), KEY_WOW64_64KEY,
};
use windows::Win32::System::{
    Diagnostics::Debug::{GetModuleHandleW, GetProcAddress},
    Threading::GetCurrentProcess,
};
use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};

use crate::models::AdapterInfo;
use crate::system::helpers::{format_mac, parse_mac_to_bytes};
use crate::utils::logger::LogError;

/// Error types for MAC manipulation operations.
#[derive(Debug, Error)]
pub enum MacError {
    #[error("Adapter not found: {0}")]
    NotFound(String),
    #[error("Adapter already spoofed: {0}")]
    AlreadySpoofed(String),
    #[error("Registry operation failed: {0}")]
    RegistryError(#[from] windows::core::Error),
    #[error("Invalid MAC address format: {0}")]
    InvalidMac(String),
    #[error("Adapter is in use: {0}")]
    AdapterInUse(String),
    #[error("Permission denied. Administrator privileges required.")]
    PermissionDenied,
    #[error("Unsupported adapter: {0}")]
    Unsupported(String),
    #[error("Failed to query adapter: {0}")]
    QueryFailed(String),
    #[error("Failed to apply changes: {0}")]
    ApplyFailed(String),
}

/// Registry path prefix for network adapter classes.
/// All physical adapters use keys under this path.
const NETADAPTER_CLASS_PATH: &str = r"SYSTEM\CurrentControlSet\Control\Class";

/// Known class GUIDs for network adapters.
/// - 4d36e972-e325-11ce-bfc1-08002be10318 = Ethernet adapters
/// - 4d36e96b-e325-11ce-bfc1-08002be10318 = Network adapters (generic)
/// The algorithm enumerates all subkeys under Class\* that match these GUIDs.
const CLASS_GUIDS: [&str; 2] = [
    r"{4d36e972-e325-11ce-bfc1-08002be10318}",
    r"{4d36e96b-e325-11ce-bfc1-08002be10318}",
];

// Helper function to check admin rights
fn is_admin() -> bool {
    unsafe {
        let mut token = std::mem::zeroed();
        let mut elevation = TOKEN_ELEVATION::default();
        let mut elevation_size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
        
        if windows::Win32::Security::OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_QUERY,
            &mut token,
        ).is_ok() {
            let result = GetTokenInformation(
                token,
                TokenElevation,
                &mut elevation as *mut _ as *mut _,
                elevation_size,
                &mut elevation_size,
            );
            CloseHandle(token);
            result.is_ok() && elevation.TokenIsElevated != 0
        } else {
            false
        }
    }
}

/// Enumerate all network adapters present on the system.
/// This uses a combination of IP Helper APIs (GetIfTable2) and
/// Windows Registry enumeration to find adapter identities.
///
/// Returns a vector of AdapterInfo sorted by name.
pub fn enumerate_adapters() -> Result<Vec<AdapterInfo>, MacError> {
    // First, use GetIfTable2 to get basic interface info
    let if_table = unsafe {
        let mut table = std::mem::zeroed();
        let hr = GetIfTable2(&mut table);
        if hr != windows::Win32::Foundation::S_OK {
            return Err(MacError::QueryFailed("GetIfTable2 failed".into()));
        }
        table
    };
    
    let interfaces = unsafe { if_table.Table };
    let count = unsafe { if_table.NumEntries } as usize;
    
    // We'll collect results in a temporary structure
    let mut adapters = Vec::new();
    
    for i in 0..count {
        let iface = &interfaces[i];
        let alias = iface.Alias;
        let desc = iface.Description;
        let phys_addr = iface.PhysicalAddress;
        let phys_len = iface.PhysicalAddressLength as usize;
        
        // Convert to strings
        let name = if !alias.is_null() {
            let api = unsafe { windows::core::PCWSTR(alias.as_ptr()) };
            let len = 0;
            let buf = api.to_string();
            if let Some(n) = buf.ok() { n } else { "Unknown Adapter".into() }
        } else {
            "Unknown Adapter".into()
        };

        let desc_str = if !desc.is_null() {
            let api = unsafe { windows::core::PCWSTR(desc.as_ptr()) };
            let buf = api.to_string();
            if let Some(n) = buf.ok() { n } else { "".into() }
        } else {
            "".into()
        };

        // Build a device identifier: try to get a stable ID (e.g., MAC address, PNPDeviceID)
        // For now we'll use description + alias. In future we could query setupapi for PNP device IDs.
        let device_id = if !desc_str.is_empty() {
            format!("{}", desc_str)
        } else {
            format!("Adapter_{}", i)
        };
        
        // Current physical address as string
        if phys_len >= 6 {
            let mut mac_bytes = [0u8; 6];
            mac_bytes.copy_from_slice(std::slice::from_raw_parts(phys_addr.as_ptr(), 6));
            let current_mac = format_mac(&mac_bytes);
            
            // Check for presence: if status = 0 (down), still show but mark as down.
            let status = match iface.OperStatus {
                1 => AdapterStatus::Up,
                2 => AdapterStatus::Down,
                3 => AdapterStatus::Testing,
                4 => AdapterStatus::Unknown,
                5 => AdapterStatus::Dormant,
                6 => AdapterStatus::NotPresent,
                7 => AdapterStatus::LowerLayerDown,
                _ => AdapterStatus::Unknown,
            };
            
            // Check if it's a system adapter we likely shouldn't mess with.
            // Very simple: if name contains "Hyper-V", "VMware", "Virtual", "TAP", "Loopback" -> unsupported
            let name_lower = name.to_lowercase();
            let is_system = name_lower.contains("hyper-v")
                || name_lower.contains("vmware")
                || name_lower.contains("virtual")
                || name_lower.contains("tap")
                || name_lower.contains("loopback")
                || name_lower.contains("pseudo")
                || name_lower.contains("bthpan");
            
            adapters.push(AdapterInfo {
                device_id: device_id.clone(),
                name: format!("{} ({})", name, desc_str),
                current_mac,
                original_mac: None, // will be filled by read_original_mac if needed
                status,
                spoofed_mac: None,
                supports_spoofing: !is_system,
            });
        }
    }
    
    // Try to add the original (factory) MAC for each adapter via registry scan.
    // This is optional; if we fail, we still return the adapter but with None.
    for adapter in adapters.iter_mut() {
        if let Ok(orig) = read_original_mac(&adapter.device_id) {
            adapter.original_mac = orig;
        }
        if let Ok(r) = read_spoofed_mac(&adapter.device_id) {
            adapter.spoofed_mac = r;
        }
    }
    
    // Sort by name alphabetically for stable UI sorting.
    adapters.sort_by(|a, b| a.name.cmp(&b.name));
    
    Ok(adapters)
}

/// Read the factory (original) MAC for a given adapter.
/// This is the value that is physically burned into the NIC, and
/// is not the same as the currently active MAC if spoofing is in effect.
/// The original MAC is stored in the same registry location but minus the "NetworkAddress"
/// value, meaning the system reads the PCI hardware ID if NetworkAddress is absent.
fn read_original_mac(device_id: &str) -> Result<Option<String>, MacError> {
    // For now, we'll use a simpler approach: try to query via OID through Ndis.
    // However the registry method actually does not store the original separately,
    // we keep it separately in user profile. So we can't reliably recover it from the system.
    // Instead we return None and expect the caller to maintain it in profile.
    Ok(None)
}

/// Read the current 'NetworkAddress' spoof value from the registry, if present.
fn read_spoofed_mac(device_id: &str) -> Result<Option<String>, MacError> {
    let key_path = find_adapter_registry_key(&device_id, true)?;
    let key = open_registry_key(HKEY_LOCAL_MACHINE, &key_path, KEY_QUERY_VALUE)?;
    let mut data_type = 0u32;
    let mut data_len = 0;
    let hr = unsafe {
        RegQueryValueExW(
            HKEY(key.raw_handle() as isize),
            &HSTRING::from("NetworkAddress").unwrap().unwrap(),
            None,
            Some(&mut data_type),
            None,
            Some(&mut data_len),
        )
    };
    if hr != windows::Win32::Foundation::S_OK {
        // No value present -> not spoofed
        return Ok(None);
    }
    if data_type != REG_SZ && data_type != REG_MULTI_SZ {
        return Err(MacError::RegistryError(windows::core::Error::from_win32()));
    }
    let mut buf = vec![0u16; data_len as usize / 2];
    let hr = unsafe {
        RegQueryValueExW(
            HKEY(key.raw_handle() as isize),
            &HSTRING::from("NetworkAddress").unwrap().unwrap(),
            None,
            Some(&mut data_type),
            Some(buf.as_mut_ptr() as *mut u8),
            Some(&mut data_len),
        )
    };
    if hr != windows::Win32::Foundation::S_OK {
        return Err(MacError::RegistryError(windows::core::Error::from_win32()));
    }
    let mac_wide = widestring::U16String::from_vec_truncating(buf);
    let mac = mac_wide.to_string_lossy();
    Ok(Some(mac))
}

/// Find the registry key path for a given adapter device ID.
/// We need to match the adapter's Hardware ID (like "PCI\VEN_8086...") to the
/// subkeys under HKLM\SYSTEM\CurrentControlSet\Control\Class\{GUID}\XXXX...
///
/// Returns the full key path if found, or Err if not.
fn find_adapter_registry_key(device_id: &str, allow_spoofing: bool) -> Result<String, MacError> {
    use windows::Win32::System::Registry;
    use std::sync::Mutex;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref CACHE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    }
    {
        let cache = CACHE.lock().map_err(|_| MacError::RegistryError(windows::core::Error::from_win32()))?;
        if let Some(path) = cache.get(device_id) {
            return Ok(path.clone());
        }
    }
    // Fallback: The caller should have stored the registry key path in the AdapterInfo (?)
    // Since we don't persist it, we can try a generic lookup: find all subkeys for each class GUID
    // and scan DriverDesc matches.
    for guid in CLASS_GUIDS.iter() {
        let base_path = format!("{}\\{}", NETADAPTER_CLASS_PATH, guid);
        if let Ok(mut key) = open_registry_key(
            HKEY_LOCAL_MACHINE,
            &base_path,
            KEY_READ | KEY_WOW64_64KEY,
        ) {
            // Enumerate subkeys
            let mut index = 0u32;
            loop {
                match enumerate_registry_subkey(&key, index) {
                    Ok(name) => {
                        let subkey_path = format!("{}\\{}", base_path, name);
                        if let Ok(subkey) = open_registry_key(
                            HKEY_LOCAL_MACHINE,
                            &subkey_path,
                            KEY_READ,
                        ) {
                            if let Ok(driver_desc) = query_registry_string(&subkey, "DriverDesc") {
                                if driver_desc.contains(device_id) || device_id.contains(&driver_desc) {
                                    // Matched, cache and return
                                    let mut cache = CACHE.lock().map_err(|_| MacError::RegistryError(windows::core::Error::from_win32()))?;
                                    cache.insert(device_id.to_string(), subkey_path.clone());
                                    return Ok(subkey_path);
                                }
                            }
                        }
                        index += 1;
                    }
                    Err(_) => break, // No more subkeys
                }
            }
        }
    }
    Err(MacError::NotFound(device_id.to_string()))
}

/// Open a registry key with the given access mask.
fn open_registry_key(
    root: HKEY,
    path: &str,
    sam_desired: u32,
) -> Result<windows::Win32::Foundation::HANDLE, windows::core::Error> {
    let path_wide = widestring::U16CString::from_str(path).map_err(|e| windows::core::Error::from_win32())?;
    let mut handle = unsafe { std::mem::zeroed() };
    unsafe {
        RegOpenKeyExW(
            root,
            PCWSTR(path_wide.as_ptr()),
            None,
            sam_desired,
            &mut handle,
        )
    }?;
    Ok(handle)
}

/// Enumerate a subkey name by index under a given key.
fn enumerate_registry_subkey(
    key: &windows::Win32::System::Registry::HKEY,
    index: u32,
) -> Result<String, windows::core::Error> {
    let mut name_buf = [0u16; 256];
    let mut name_len = name_buf.len() as u32;
    unsafe {
        RegEnumKeyExW(
            HKEY(key.raw_handle() as isize),
            index,
            &mut name_buf as *mut _,
            &mut name_len,
            None,
            None,
            None,
            None,
        )
    }?;
    let name_wide = widestring::U16String::from_slice_truncating(&name_buf[..name_len as usize]);
    Ok(name_wide.to_string_lossy())
}

/// Query a string value from a registry key.
fn query_registry_string(
    key: &windows::Win32::System::Registry::HKEY,
    value_name: &str,
) -> Result<String, windows::core::Error> {
    let value_name_wide = widestring::U16CString::from_str(value_name).map_err(|_| windows::core::Error::from_win32())?;
    let mut data_type = 0u32;
    let mut data_len = 0u32;
    unsafe {
        RegQueryValueExW(
            HKEY(key.raw_handle() as isize),
            PCWSTR(value_name_wide.as_ptr()),
            None,
            Some(&mut data_type),
            None,
            Some(&mut data_len),
        )
    }?;
    if data_type != REG_SZ && data_type != REG_MULTI_SZ {
        return Err(windows::core::Error::from_win32());
    }
    let mut buf = vec![0u16; data_len as usize / 2];
    unsafe {
        RegQueryValueExW(
            HKEY(key.raw_handle() as isize),
            PCWSTR(value_name_wide.as_ptr()),
            None,
            Some(&mut data_type),
            Some(buf.as_mut_ptr() as *mut u8),
            Some(&mut data_len),
        )
    }?;
    let string_wide = widestring::U16String::from_vec_truncating(buf);
    Ok(string_wide.to_string_lossy())
}

/// Set the MAC address for a given adapter using the Windows registry.
/// The write uses the NetworkAddress value under the adapter's
/// Control\Class key.  The change requires a system restart or
/// adapter restart/disconnect to take effect.
///
/// The spoofed MAC is only applied to adapters that support it.
///
/// Returns the full registry key path that was modified.
pub fn set_mac_address(
    device_id: &str,
    mac: &str,
) -> Result<String, MacError> {
    // 1. Requires admin
    if !is_admin() {
        return Err(MacError::PermissionDenied);
    }

    // 2. Validate MAC format
    if !crate::system::helpers::is_valid_mac(mac) {
        return Err(MacError::InvalidMac(mac.to_string()));
    }

    // 3. Find the registry key for the adapter
    let key_path = find_adapter_registry_key(device_id, true)?;
    let key = open_registry_key(HKEY_LOCAL_MACHINE, &key_path, KEY_SET_VALUE | KEY_QUERY_VALUE)?;

    // 4. Write the "NetworkAddress" value as a REG_SZ.
    // The MAC must be hyphen-separated for Windows. e.g. "00-11-22-33-44-55"
    let mac_hyphen = mac.replace(':', "-");
    let mac_wide = widestring::U16CString::from_str(&mac_hyphen)
        .map_err(|_| MacError::InvalidMac(mac.to_string()))?;

    unsafe {
        RegSetValueExW(
            HKEY(key.raw_handle() as isize),
            PCWSTR(widestring::U16CString::from_str("NetworkAddress").unwrap().unwrap().as_ptr()),
            0,
            REG_SZ,
            mac_wide.as_bytes_with_nul().as_ptr() as *const u8,
            (mac_wide.as_bytes_with_nul().len() * 2) as u32,
        )
    }.map_err(|e| MacError::RegistryError(e))?;

    // 5. Optional: also set a "FeatureSettingsOverride" value to ensure spoofing is enabled
    // Some adapters have "NetworkAddress" ignored depending on driver settings.
    // We'll write 0 (use feature settings) to override restrictions.
    let override_val = widestring::U16CString::from_str("0").unwrap();
    let _ = unsafe {
        RegSetValueExW(
            HKEY(key.raw_handle() as isize),
            PCWSTR(widestring::U16CString::from_str("FeatureSettingsOverride").unwrap().unwrap().as_ptr()),
            0,
            REG_DWORD,
            override_val.as_bytes_with_nul().as_ptr() as *const u8,
            4,
        )
    }; // not critical, ignore errors

    Ok(key_path)
}

/// Remove spoofing from an adapter by deleting the NetworkAddress registry value.
/// This restores the adapter to use its factory-burned MAC.
pub fn clear_mac_spoof(device_id: &str) -> Result<String, MacError> {
    if !is_admin() {
        return Err(MacError::PermissionDenied);
    }

    let key_path = find_adapter_registry_key(device_id, true)?;
    let key = open_registry_key(HKEY_LOCAL_MACHINE, &key_path, KEY_SET_VALUE)?;

    unsafe {
        RegDeleteValueW(
            HKEY(key.raw_handle() as isize),
            PCWSTR(widestring::U16CString::from_str("NetworkAddress").unwrap().unwrap().as_ptr()),
        )
    }.map_err(|e| MacError::RegistryError(e))?;

    Ok(key_path)
}

/// Generate a random, valid MAC address.
/// MAC addresses are 6 bytes. The first byte (octet) must have the
/// local/admin bit set (bit 1) to indicate a locally administered MAC.
/// This is the standard way to differentiate a spoofed MAC from a factory one.
/// We'll produce a "locally administered" address in the range:
/// `02:XX:XX:XX:XX:XX` where the leading 2 indicates local admin.
/// Rest bytes are random 00-FF.
///
/// Returns a colon-separated uppercase string.
pub fn generate_random_mac() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    // First byte with local admin bit set, i.e., 0x02 + random 0x00-0xFD for rest 7 bits
    let b0: u8 = (rng.gen_range(0..63) << 1) | 0x02;
    let mut mac = Vec::with_capacity(6);
    mac.push(b0);
    for _ in 1..6 {
        mac.push(rng.gen());
    }
    format_mac(&mac)
}

/// Reload adapter list by re-enumerating and applying stored profiles.
/// Should be called after an operation that might affect adapter states.
pub fn refresh_adapters() -> Result<Vec<AdapterInfo>, MacError> {
    enumerate_adapters()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_mac() {
        let mac = generate_random_mac();
        assert!(is_valid_mac(&mac));
        assert_eq!(mac.len(), 17); // 6 groups * 2 + 5 colons
        // Check first byte has the local admin bit set (0x02)
        let bytes = parse_mac_to_bytes(&mac).unwrap();
        assert_eq!(bytes[0] & 0x02, 0x02, "Local admin bit not set");
        assert_eq!(bytes[0] & 0x01, 0, "Unicast bit should be clear");
    }
}
