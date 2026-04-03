use crate::core::state::AppState;
use serde::Serialize;
use tauri::State;

#[derive(Serialize)]
pub struct AppStateResponse {
    pub version: String,
    pub licensed: bool,
    pub trial_active: bool,
    pub build_date: String,
}

#[derive(Serialize)]
pub struct SlapResult {
    pub success: bool,
    pub message: String,
    pub duration_ms: u64,
}

#[tauri::command]
pub async fn get_app_state(state: State<'_, AppState>) -> Result<AppStateResponse, String> {
    Ok(AppStateResponse {
        version: state.version.clone(),
        licensed: state.license_manager.is_licensed(),
        trial_active: state.license_manager.is_trial_active(),
        build_date: env!("BUILD_DATE").to_string(),
    })
}

#[tauri::command]
pub async fn perform_slap(
    intensity: String,
    state: State<'_, AppState>,
) -> Result<SlapResult, String> {
    let start = std::time::Instant::now();

    if !state.license_manager.is_licensed() && !state.license_manager.is_trial_active() {
        return Err("This feature requires a valid license or active trial. Purchase yours!.".to_string());
    }

    // TODO: Implement core functionality
    let _ = intensity;
    
    let duration_ms = start.elapsed().as_millis() as u64;

    Ok(SlapResult {
        success: true,
        message: "Done!".to_string(),
        duration_ms,
    })
}
