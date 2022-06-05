#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;
use commands::{load_settings, get_projects, create_time, get_services, start_timer};

use std::sync::Mutex;

pub struct AppState(pub Mutex<AppSettings>);

pub struct AppSettings {
  mite_app: String,
  api_key: String,
}

#[tauri::command]
fn heart_beat() -> String {
  "Hello from Rust!".into()
}

fn main() {
  tauri::Builder::default()
    .manage(AppState(Mutex::new(AppSettings { mite_app: "".to_string(), api_key: "".to_string() })))
    .invoke_handler(tauri::generate_handler![get_projects, heart_beat, create_time, start_timer, get_services, load_settings])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
