fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_dir = std::path::Path::new(&manifest_dir).parent().unwrap();
    
    let src_config = workspace_dir.join("tauri.conf.json");
    let dst_config = std::path::Path::new(&manifest_dir).join("tauri.conf.json");
    
    if src_config.exists() {
        if let Err(e) = std::fs::copy(&src_config, &dst_config) {
            eprintln!("Failed to copy tauri.conf.json: {}", e);
        }
    }
    
    std::env::set_current_dir(workspace_dir).unwrap();
    
    tauri_build::build()
}