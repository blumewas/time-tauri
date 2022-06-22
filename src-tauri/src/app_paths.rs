use tauri::api::path;
use std::path::PathBuf;
use std::fs;
use std::path::Path;

/**
 * Get our activiry log path
 */
pub fn activity_log_path() -> PathBuf {
  let path = get_app_path(Some("activity-logs".to_string()));

  if !Path::new(&path).exists() {
    fs::create_dir(&path)
      .unwrap_or_else(|err| panic!("Failed to open or create directory {:?}, {}", path, err));
  }

  path
}

/**
 * Get path relative to our apps path
 */
pub fn get_app_path(part: Option<String>) -> PathBuf {
  let mut path = path::config_dir()
    .unwrap()
    .join("blumewas.timetauri");

    if let Some(p) = part {
      path = path.join(p);
    }

  return path;
}
