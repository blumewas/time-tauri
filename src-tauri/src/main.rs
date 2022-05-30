#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::State;
use std::sync::Mutex;

struct AppState(pub Mutex<AppSettings>);

struct AppSettings {
  mite_app: String,
  api_key: String,
}

#[tauri::command]
fn load_settings(api_key: String, mite_app: String, state: State<'_, AppState>) {
  let mut state_guard = state.0.lock().unwrap();
  // Change field of state struct
  state_guard.api_key = String::from("bar");
  
  // set app state
  *state_guard = AppSettings { mite_app: mite_app.to_string(), api_key: api_key.to_string() };
}

#[tauri::command]
async fn get_projects(state: State<'_, AppState>) -> Result<String, String> {
  let state_guard = state.0.lock().unwrap();

  let url = format!("https://{}.mite.yo.lk/projects.json", state_guard.mite_app);

  let body: String = ureq::get(&url)
    .set("X-MiteApiKey", &state_guard.api_key)
    .set("User-Agent", "time-tauri/v0.1 (schneider@mindtwo.de);")
    .call()
    .unwrap()
    .into_string()
    .unwrap();
  
  Ok(body.into())
}

#[tauri::command]
async fn get_services(state: State<'_, AppState>) -> Result<String, String> {
  let state_guard = state.0.lock().unwrap();

  let url = format!("https://{}.mite.yo.lk/services.json", state_guard.mite_app);

  let body: String = ureq::get(&url)
    .set("X-MiteApiKey", &state_guard.api_key)
    .set("User-Agent", "time-tauri/v0.1 (schneider@mindtwo.de);")
    .call()
    .unwrap()
    .into_string()
    .unwrap();
  
  Ok(body.into())
}

#[tauri::command]
async fn create_time(state: State<'_, AppState>, project_id: i32, service_id: Option<i32>, note: Option<String>, minutes: Option<i32>) -> Result<String, String> {
  let state_guard = state.0.lock().unwrap();

  let url = format!("https://{}.mite.yo.lk/time_entries.json", state_guard.mite_app);

  let resp: String = ureq::post(&url)
    .set("X-MiteApiKey", &state_guard.api_key)
    .set("User-Agent", "time-tauri/v0.1 (schneider@mindtwo.de);")
    .set("Content-Type", "application/json; charset=UTF-8")
    .send_json(ureq::json!({
      "time_entry": {
        "project_id": project_id,
        "service_id": service_id,
        "note": note,
        "minutes": minutes
      }
    }))
    .unwrap()
    .into_string()
    .unwrap();

  Ok(resp.into())
}

#[tauri::command]
async fn start_stop_time(state: State<'_, AppState>, entry_id: i32) -> Result<String, String> {
  let state_guard = state.0.lock().unwrap();
  
  let url = format!("https://{}.mite.yo.lk/tracker/{}.json", state_guard.mite_app, entry_id);

  let resp: String = ureq::patch(&url)
    .set("X-MiteApiKey", &state_guard.api_key)
    .set("User-Agent", "time-tauri/v0.1 (schneider@mindtwo.de);")
    .set("Content-Type", "application/json; charset=UTF-8")
    .call()
    .unwrap()
    .into_string()
    .unwrap();

  Ok(resp.into())
}

#[tauri::command]
fn heart_beat() -> String {
  "Hello from Rust!".into()
}

fn main() {
  tauri::Builder::default()
    .manage(AppState(Mutex::new(AppSettings { mite_app: "".to_string(), api_key: "".to_string() })))
    .invoke_handler(tauri::generate_handler![get_projects, heart_beat, create_time, start_stop_time, get_services, load_settings])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
