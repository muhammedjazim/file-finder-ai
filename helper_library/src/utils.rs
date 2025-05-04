use std::path::{Path, PathBuf};
use directories::ProjectDirs;

pub fn get_app_data_dir() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("com", "YourCompanyName", "FileTracker") {
        let data_dir = proj_dirs.data_dir();
        
        std::fs::create_dir_all(data_dir).ok();
        
        return data_dir.to_path_buf();
    }
    
    let mut path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    if path.is_file() {
        path.pop();
    }
    path
}

pub fn get_file_extension(path: &str) -> String {
    Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_string()
}

pub fn is_filepath_or_extension_ignored(path: &str, excluded_patterns: &[&str], ignored_extensions: &[&str]) -> bool {
    if excluded_patterns.iter().any(|&pattern| path.contains(pattern)) {
        return true;
    }

    let extension = get_file_extension(path);
    if ignored_extensions.contains(&extension.as_str()) {
        return true;
    }

    false
}