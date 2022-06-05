
use tauri::{command, State};

static REFERER: &'static str = "time-tauri/v0.3 (https://github.com/blumewas/time-tauri);";

// load settings from app folder
#[command]
pub fn load_settings(api_key: String, mite_app: String, state: State<'_, super::AppState>) {
  let mut state_guard = state.0.lock().unwrap();

  
  // set app state
  *state_guard = super::AppSettings { mite_app: mite_app.to_string(), api_key: api_key.to_string() };
}

// get projects from mite api
#[command]
pub async fn get_projects(state: State<'_, super::AppState>) -> Result<String, String> {
  let state_guard = state.0.lock().unwrap();

  let url = format!("https://{}.mite.yo.lk/projects.json", state_guard.mite_app);

  match ureq::get(&url)
    .set("X-MiteApiKey", &state_guard.api_key)
    .set("User-Agent", REFERER)
    .call() {
      Ok(response) => {
        let body = response.into_string().unwrap();

        return Ok(body.into());
      },
      Err(ureq::Error::Status(_code, response)) => {
        let body = response.into_string().unwrap();
        return Err(body.into());
      }
      Err(_) => { Err("".into()) }
    }
  
}

// get services from mite api
#[command]
pub async fn get_services(state: State<'_, super::AppState>) -> Result<String, String> {
  let state_guard = state.0.lock().unwrap();

  let url = format!("https://{}.mite.yo.lk/services.json", state_guard.mite_app);

  match ureq::get(&url)
    .set("X-MiteApiKey", &state_guard.api_key)
    .set("User-Agent", REFERER)
    .call() {
      Ok(response) => {
        let body = response.into_string().unwrap();
        return Ok(body.into());
      },
      Err(ureq::Error::Status(_code, response)) => {
        let body = response.into_string().unwrap();
        return Err(body.into());
      }
      Err(_) => { Err("".into()) }
    }  
}

// create a time entry via mite api
#[command]
pub async fn create_time(state: State<'_, super::AppState>, project_id: Option<i32>, service_id: Option<i32>, note: Option<String>, minutes: Option<i32>) -> Result<String, String> {
  let state_guard = state.0.lock().unwrap();

  let url = format!("https://{}.mite.yo.lk/time_entries.json", state_guard.mite_app);

  match ureq::post(&url)
    .set("X-MiteApiKey", &state_guard.api_key)
    .set("User-Agent", REFERER)
    .set("Content-Type", "application/json; charset=UTF-8")
    .send_json(ureq::json!({
      "time_entry": {
        "project_id": project_id,
        "service_id": service_id,
        "note": note,
        "minutes": minutes
      }
    })) {
      Ok(response) => {
        let body = response.into_string().unwrap();
        return Ok(body.into());
      },
      Err(ureq::Error::Status(_code, response)) => {
        let body = response.into_string().unwrap();
        return Err(body.into());
      }
      Err(_) => { Err("".into()) }
    }  
}

// get timer that runs on start via mite api
#[command]
pub async fn get_timer(state: State<'_, super::AppState>) -> Result<String, String> {
  let state_guard = state.0.lock().unwrap();
  
  let url = format!("https://{}.mite.yo.lk/tracker.json", state_guard.mite_app);

  match ureq::get(&url)
    .set("X-MiteApiKey", &state_guard.api_key)
    .set("User-Agent", REFERER)
    .call() {
      Ok(response) => {
        let body = response.into_string().unwrap();
        return Ok(body.into());
      },
      Err(ureq::Error::Status(_code, response)) => {
        let body = response.into_string().unwrap();
        return Err(body.into());
      }
      Err(_) => { Err("".into()) }
    }
}

// start a time entry
#[command]
pub async fn start_timer(state: State<'_, super::AppState>, entry_id: i32) -> Result<String, String> {
  let state_guard = state.0.lock().unwrap();
  
  let url = format!("https://{}.mite.yo.lk/tracker/{}.json", state_guard.mite_app, entry_id);

  match ureq::patch(&url)
    .set("X-MiteApiKey", &state_guard.api_key)
    .set("User-Agent", REFERER)
    .set("Content-Type", "application/json; charset=UTF-8")
    .call() {
      Ok(response) => {
        let body = response.into_string().unwrap();
        return Ok(body.into());
      },
      Err(ureq::Error::Status(_code, response)) => {
        let body = response.into_string().unwrap();
        return Err(body.into());
      }
      Err(_) => { Err("".into()) }
    }
}

// start a time entry
#[command]
pub async fn stop_timer(state: State<'_, super::AppState>, timer_id: i32) -> Result<String, String> {
  let state_guard = state.0.lock().unwrap();
  
  let url = format!("https://{}.mite.yo.lk/tracker/{}.json", state_guard.mite_app, timer_id);

  match ureq::delete(&url)
    .set("X-MiteApiKey", &state_guard.api_key)
    .set("User-Agent", REFERER)
    .call() {
      Ok(response) => {
        let body = response.into_string().unwrap();
        return Ok(body.into());
      },
      Err(ureq::Error::Status(_code, response)) => {
        let body = response.into_string().unwrap();
        return Err(body.into());
      }
      Err(_) => { Err("".into()) }
    }
}
