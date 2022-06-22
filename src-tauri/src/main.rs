#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod app_paths;

mod commands;
use commands::{create_time, get_projects, get_services, load_settings, start_timer, get_timer, stop_timer};

mod menu;
mod active_window_log;

use std::sync::Mutex;
use tauri::{WindowEvent, RunEvent, GlobalShortcutManager, Manager};

pub struct AppState(pub Mutex<AppSettings>);

// our app settings
pub struct AppSettings {
  mite_app: String,
  api_key: String,
}

static mut IS_MINIMIZED: bool = false;

#[tauri::command]
fn heart_beat() -> String {
  "Hello from Rust!".into()
}

fn main() {
  // todo disable full size
  #[allow(unused_mut)]
  let mut app = tauri::Builder::default()
    .manage(AppState(Mutex::new(AppSettings {
      mite_app: "".to_string(),
      api_key: "".to_string(),
    })))
    .menu(menu::get_menu())
    .invoke_handler(tauri::generate_handler![
      get_projects,
      heart_beat,
      create_time,
      get_services,
      load_settings,
      start_timer,
      get_timer,
      stop_timer,
    ])
    .build(tauri::generate_context!())
    .expect("error while building tauri application");

  #[cfg(target_os = "macos")]
  app.set_activation_policy(tauri::ActivationPolicy::Regular);

  app.run(|app, e| match e {

    // Application is ready (triggered only once)
    RunEvent::Ready => {
      active_window_log::start_logging();

      let app_handle = app.clone();
      
      app
        .global_shortcut_manager()
        .register("CmdOrCtrl+Shift+Comma", move || {
          let app_handle = app_handle.clone();

          let window = app_handle.get_window("main").unwrap();
          
          unsafe {
            if IS_MINIMIZED {
              window.set_focus().unwrap();
              window.unminimize().unwrap();

              IS_MINIMIZED = false;
            } else {
              window.minimize().unwrap();

              IS_MINIMIZED = true;
            }
          }
        })
        .unwrap();
    }

    // check if somethings happens with the window
    RunEvent::WindowEvent { event, .. } => {
      match event {
        // keep window running on close
        WindowEvent::CloseRequested {api, ..} => {
          api.prevent_close();
        }
        _ => {}
      }

    }

    RunEvent::ExitRequested { api, .. } => {
      api.prevent_exit();
    }
    _ => {}
  })
}
