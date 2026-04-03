use crate::core::state::AppState;
use serde::Serialize;
use tauri::State;

#[derive(Serialize)]
pub struct LicenseInfoResponse {
    pub licensed: bool,
    pub trial_active: bool,
    pub trial_days_remaining: Option<u32>,
    pub expires_at: Option<String>,
}

#[tauri::command]
pub async fn get_license_info(state: State<'_, AppState>) -> Result<LicenseInfoResponse, String> {
    Ok(state.license_manager.get_license_info())
}

#[tauri::command]
pub async fn activate_license(license_key: String, state: State<'_, AppState>) -> Result<bool, String> {
    state.license_manager.activate(&license_key)
}

#[tauri::command]
pub async fn start_trial(state: State<'_, AppState>) -> Result<bool, String> {
    state.license_manager.start_trial()
}
