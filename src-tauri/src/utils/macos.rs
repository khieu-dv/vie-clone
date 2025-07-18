// File: src/utils/macos.rs
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

/// Cấp quyền cho file trên macOS
pub fn macos_permit_file(filepath: &str, mode: u32) {
    if let Ok(metadata) = fs::metadata(filepath) {
        let mut permissions = metadata.permissions();
        permissions.set_mode(mode);
        let _ = fs::set_permissions(filepath, permissions);
    }
}

/// Copy app bundle trên macOS
pub fn copy_app_bundle(source: &str, dest: &str) -> std::io::Result<()> {
    let output = Command::new("cp")
        .arg("-R")
        .arg(source)
        .arg(dest)
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to copy app bundle"
        ));
    }
    
    Ok(())
}

/// Xóa app bundle trên macOS
pub fn remove_app_bundle(app_path: &str) -> std::io::Result<()> {
    let output = Command::new("rm")
        .arg("-rf")
        .arg(app_path)
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to remove app bundle"
        ));
    }
    
    Ok(())
}

/// Giải nén app bundle từ tar.gz
pub fn extract_app_bundle(archive_path: &str, dest_name: &str) -> std::io::Result<()> {
    // Giải nén file tar.gz
    let output = Command::new("tar")
        .arg("-xzf")
        .arg(archive_path)
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to extract app bundle"
        ));
    }
    
    // Đổi tên nếu cần thiết
    if dest_name != "VieClone.app" {
        let _ = Command::new("mv")
            .arg("VieClone.app")
            .arg(dest_name)
            .output();
    }
    
    Ok(())
}