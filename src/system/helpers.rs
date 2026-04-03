//! Helper utilities used across the system module.

use std::fmt;

/// Format a raw byte slice as a colon-separated MAC string.
/// e.g. [0x1A, 0x2B, 0x3C, 0x4D, 0x5E, 0x6F] -> "1A:2B:3C:4D:5E:6F"
pub fn format_mac(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }
    let parts: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    parts.join(":")
}

/// Normalize a MAC string into a canonical form for comparison.
/// Accepts colon-separated, hyphen-separated, or dot-separated, and
/// case-insensitive. Returns uppercase colon-separated.
pub fn normalize_mac(mac: &str) -> String {
    mac.to_uppercase()
        .replace(['-', '.', ' '], ":")
        .replace("::", ":") // collapse accidental double colons
}

/// Validate that a MAC string is syntactically correct.
/// Accepts 6 colon/hyphen/space separated bytes.
pub fn is_valid_mac(mac: &str) -> bool {
    let cleaned = mac.replace(['-', '.', ' '], "");
    if cleaned.len() != 12 {
        return false;
    }
    cleaned
        .chars()
        .all(|c| c.is_ascii_hexdigit())
}

/// Parse a normalized MAC string into its 6 raw bytes.
/// Returns Err if the string is invalid.
pub fn parse_mac_to_bytes(mac: &str) -> Result<[u8; 6], String> {
    let cleaned = mac.replace('-', ":").replace(' ', ":").replace('.', ":");
    let parts: Vec<&str> = cleaned.split(':').collect();
    if parts.len() != 6 {
        return Err(format!(
            "Expected 6 hex octets separated by colons, got '{}'",
            mac
        ));
    }
    let mut bytes = [0u8; 6];
    for (i, part) in parts.iter().enumerate() {
        match u8::from_str_radix(part, 16) {
            Ok(b) => bytes[i] = b,
            Err(_) => return Err(format!("Invalid hex byte '{}' at position {}", part, i)),
        }
    }
    Ok(bytes)
}

/// Generate a random alphanumeric string of the given length.
/// Used for generating unique profile IDs.
pub fn random_hex(len: usize) -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let digit = rng.gen_range(0..16);
            format!("{:x}", digit)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_mac() {
        assert_eq!(
            format_mac(&[0x1A, 0x2B, 0x3C, 0x4D, 0x5E, 0x6F]),
            "1A:2B:3C:4D:5E:6F"
        );
    }

    #[test]
    fn test_normalize_mac() {
        assert_eq!(normalize_mac("aa-bb-cc-dd-ee-ff"), "AA:BB:CC:DD:EE:FF");
        assert_eq!(normalize_mac("aa.bb.cc.dd.ee.ff"), "AA:BB:CC:DD:EE:FF");
    }

    #[test]
    fn test_is_valid_mac() {
        assert!(is_valid_mac("AA:BB:CC:DD:EE:FF"));
        assert!(is_valid_mac("AA-BB-CC-DD-EE-FF"));
        assert!(!is_valid_mac("GG:HH:II:JJ:KK:LL"));
        assert!(!is_valid_mac("AA:BB:CC"));
    }
}
