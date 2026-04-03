fn main() {
    // Inject build date for versioning
    // This allows the frontend to show when the binary was built
    let build_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    println!("cargo:rustc-env=BUILD_DATE={}", build_date);
    
    // Compile the frontend dist is handled by tauri.conf.json build.frontendDist
    tauri_build::build()
}
