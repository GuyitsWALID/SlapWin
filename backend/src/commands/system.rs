use serde::Serialize;
use sysinfo::{System, SystemExt};

#[derive(Serialize)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub hostname: String,
    pub total_memory_gb: f64,
    pub available_memory_gb: f64,
    pub cpu_cores: usize,
    pub cpu_name: String,
    uptime_seconds: u64,
}

#[tauri::command]
pub async fn get_system_info() -> Result<SystemInfo, String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    Ok(SystemInfo {
        os_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
        os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
        hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        total_memory_gb: sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        available_memory_gb: sys.available_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        cpu_cores: System::cpu_cores().unwrap_or(1),
        cpu_name: System::cpu_name().unwrap_or_else(|| "Unknown".to_string()),
        uptime_seconds: System::uptime(),
    })
}
