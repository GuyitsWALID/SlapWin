use tauri::Manager;
use crate::core::state::AppState;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize app state
            let state = AppState::load()?;
            app.manage(state);
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::commands::app::get_app_state,
            crate::commands::app::perform_slap,
            crate::commands::license::get_license_info,
            crate::commands::license::activate_license,
            crate::commands::license::start_trial,
            crate::commands::system::get_system_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
