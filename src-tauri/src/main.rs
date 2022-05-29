#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

struct MyState(String);

#[tauri::command]
async fn get_projects(api_key: String) -> Result<String, String> {
  println!("{}", api_key);

  let body: String = ureq::get("https://mindtwo.mite.yo.lk/projects.json")
    .set("X-MiteApiKey", &api_key)
    .set("User-Agent", "time-tauri/v0.1 (schneider@mindtwo.de);")
    .call()
    .unwrap()
    .into_string()
    .unwrap();
  
  Ok(body.into())
}

#[tauri::command]
async fn get_services(api_key: String) -> Result<String, String> {  
  let body: String = ureq::get("https://mindtwo.mite.yo.lk/services.json")
    .set("X-MiteApiKey", &api_key)
    .set("User-Agent", "time-tauri/v0.1 (schneider@mindtwo.de);")
    .call()
    .unwrap()
    .into_string()
    .unwrap();
  
  Ok(body.into())
}

#[tauri::command]
async fn create_time(api_key: String, project_id: i32, service_id: Option<i32>, note: Option<String>, minutes: Option<i32>) -> Result<String, String> {

  let resp: String = ureq::post("https://mindtwo.mite.yo.lk/time_entries.json")
    .set("X-MiteApiKey", &api_key)
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
async fn start_stop_time(api_key: String, entry_id: i32) -> Result<String, String> {
  let url = format!("https://mindtwo.mite.yo.lk/tracker/{}.json", entry_id);

  let resp: String = ureq::patch(&url)
    .set("X-MiteApiKey", &api_key)
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
    .manage(MyState("projects".into()))
    .invoke_handler(tauri::generate_handler![get_projects, heart_beat, create_time, start_stop_time, get_services])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
