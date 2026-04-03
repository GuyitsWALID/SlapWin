use anyhow::{Result, bail, Context};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LicenseData {
    pub license_key: String,
    pub product: String,
    pub issued_at: String, // ISO 8601
    pub expires_at: Option<String>,
    pub features: Vec<String>,
    pub customer: Option<String>,
    pub signature: String, // HMAC-SHA256 of the above fields + secret
}

#[derive(Serialize, Clone)]
pub struct LicenseInfo {
    pub licensed: bool,
    pub trial_active: bool,
    pub trial_days_remaining: Option<u32>,
    pub expires_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct PersistentStorage {
    license: Option<LicenseData>,
    trial_started_at: Option<String>,
}

impl PersistentStorage {
    fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self {
                license: None,
                trial_started_at: None,
            });
        }
        let data = fs::read_to_string(path).context("reading license file")?;
        serde_json::from_str(&data).context("parsing license JSON")
    }

    fn save(&self, path: &Path) -> Result<()> {
        let data = serde_json::to_vec_pretty(self).context("serializing license data")?;
        fs::write(path, data).context("writing license file")?;
        Ok(())
    }
}

pub struct LicenseManager {
    storage: PersistentStorage,
    secret: String,
    storage_path: PathBuf,
}

impl LicenseManager {
    pub fn load(path: &Path) -> Result<Self> {
        let storage = PersistentStorage::load(path).context("load persistent storage")?;
        // In a real app, this would be a proper secret stored securely
        let secret = std::env::var("SIGNING_SECRET")
            .unwrap_or_else(|_| "dev-secret-12345".to_string());
        Ok(Self {
            storage,
            secret,
            storage_path: path.to_path_buf(),
        })
    }

    pub fn get_license_info(&self) -> LicenseInfo {
        LicenseInfo {
            licensed: self.is_licensed(),
            trial_active: self.is_trial_active(),
            trial_days_remaining: self.trial_days_remaining(),
            expires_at: self.storage.license.as_ref().and_then(|l| l.expires_at.clone()),
        }
    }

    pub fn is_licensed(&self) -> bool {
        if let Some(license) = &self.storage.license {
            return self.verify_signature(license) && self.license_is_valid(license);
        }
        false
    }

    pub fn is_trial_active(&self) -> bool {
        if let Some(start_str) = &self.storage.trial_started_at {
            if let Ok(start) = NaiveDateTime::parse_from_str(start_str, "%+") {
                let now = Utc::now();
                let trial_end = start + chrono::Duration::days(7);
                return now.naive_utc() < trial_end;
            }
        }
        false
    }

    pub fn trial_days_remaining(&self) -> Option<u32> {
        if !self.is_trial_active() {
            return None;
        }
        if let Some(start_str) = &self.storage.trial_started_at {
            if let Ok(start) = NaiveDateTime::parse_from_str(start_str, "%+") {
                let now = Utc::now();
                let trial_end = start + chrono::Duration::days(7);
                let days_left = (trial_end.date() - now.date()).num_days();
                return Some(days_left.max(0) as u32);
            }
        }
        None
    }

    pub fn activate(&mut self, license_key: &str) -> Result<bool> {
        // In production, this would validate the key against a server or cryptographic signature
        // For now, simple format check and mock validation

        let mock_key = "SLAPMAC-LEGAL-OWNER-KEY-2026-ALPHA";
        if license_key != mock_key {
            bail!("Invalid license key. Please check your purchase.");
        }

        let mut license_map = HashMap::new();
        license_map.insert("license_key".to_string(), license_key.to_string());
        license_map.insert("product".to_string(), "slapmac".to_string());
        license_map.insert("issued_at".to_string(), Utc::now().to_rfc3339());
        license_map.insert("expires_at".to_string(), "2099-12-31T23:59:59Z".to_string());
        license_map.insert("features".to_string(),
            serde_json::to_value(&["full", "premium"])?);
        license_map.insert("customer".to_string(), "Valid Customer".to_string());

        let signature = self.sign(&license_map);
        let license = LicenseData {
            license_key: license_key.to_string(),
            product: "slapmac".to_string(),
            issued_at: Utc::now().to_rfc3339(),
            expires_at: Some("2099-12-31T23:59:59Z".to_string()),
            features: vec!["full".to_string(), "premium".to_string()],
            customer: Some("Valid Customer".to_string()),
            signature,
        };

        self.storage.license = Some(license);
        self.save().context("saving license")?;
        Ok(true)
    }

    pub fn start_trial(&mut self) -> Result<bool> {
        if self.is_trial_active() {
            bail!("Trial already active");
        }
        // If already licensed, cannot start trial
        if self.is_licensed() {
            bail!("Already licensed");
        }

        self.storage.trial_started_at = Some(Utc::now().to_rfc3339());
        self.save().with_context(|| "saving trial start time")?;
        Ok(true)
    }

    fn save(&self) -> Result<()> {
        self.storage.save(&self.storage_path)
    }

    fn verify_signature(&self, _license: &LicenseData) -> bool {
        // TODO: Implement real signature verification with HMAC-SHA256
        // For now, trust that the file is valid if it exists
        true
    }

    fn sign(&self, data: &HashMap<String, String>) -> String {
        // Mock signature - in production, use proper crypto (HMAC-SHA256)
        let mut combined = String::new();
        let mut keys: Vec<&String> = data.keys().collect();
        keys.sort();
        for k in keys {
            combined.push_str(k);
            combined.push_str(data.get(k).unwrap_or(&"".to_string()));
            combined.push_str(&self.secret);
        }
        format!("sig-{}", combined.len() as u32)
    }

    fn license_is_valid(&self, license: &LicenseData) -> bool {
        // Check expiration if present
        if let Some(expires) = &license.expires_at {
            if let Ok(exp) = DateTime::parse_from_rfc3339(expires) {
                if exp < Utc::now() {
                    return false;
                }
            }
        }
        true
    }
}
