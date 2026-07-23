fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_dir = std::path::Path::new(&manifest_dir).parent().unwrap();
    
    let src_config = workspace_dir.join("tauri.conf.json");
    let dst_config = std::path::Path::new(&manifest_dir).join("tauri.conf.json");
    
    if src_config.exists() {
        match std::fs::copy(&src_config, &dst_config) {
            Ok(_) => {},
            Err(e) => eprintln!("Failed to copy tauri.conf.json: {}", e),
        }
    }
    
    let src_icons = workspace_dir.join("icons");
    let dst_icons = std::path::Path::new(&manifest_dir).join("icons");
    
    if src_icons.exists() && !dst_icons.exists() {
        match std::fs::create_dir(&dst_icons) {
            Ok(_) => {},
            Err(e) => eprintln!("Failed to create icons directory: {}", e),
        }
    }
    
    if src_icons.exists() {
        if let Ok(entries) = std::fs::read_dir(&src_icons) {
            for entry in entries.flatten() {
                let file_name = entry.file_name();
                let src_file = entry.path();
                let dst_file = dst_icons.join(&file_name);
                match std::fs::copy(&src_file, &dst_file) {
                    Ok(_) => {},
                    Err(e) => eprintln!("Failed to copy icon {:?}: {}", file_name, e),
                }
            }
        }
    }
    
    let src_capabilities = workspace_dir.join("capabilities");
    let dst_capabilities = std::path::Path::new(&manifest_dir).join("capabilities");
    
    if src_capabilities.exists() && !dst_capabilities.exists() {
        match std::fs::create_dir_all(&dst_capabilities) {
            Ok(_) => {},
            Err(e) => eprintln!("Failed to create capabilities directory: {}", e),
        }
    }
    
    if src_capabilities.exists() {
        if let Ok(entries) = std::fs::read_dir(&src_capabilities) {
            for entry in entries.flatten() {
                let file_name = entry.file_name();
                let src_file = entry.path();
                let dst_file = dst_capabilities.join(&file_name);
                match std::fs::copy(&src_file, &dst_file) {
                    Ok(_) => {},
                    Err(e) => eprintln!("Failed to copy capability {:?}: {}", file_name, e),
                }
            }
        }
    }
    
    tauri_build::build()
}