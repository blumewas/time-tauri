use chrono::{Datelike, Utc};
use std::fs;
use std::fs::{OpenOptions, Permissions};
use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::path::Path;

use crate::app_paths;
// TODO give credit https://github.com/vn971/timeplot/blob/master/src/main.rs
// logging interval in seconds
static INTERVAL: u64 = 15;

// save current info
struct ActiveWindow {
  name: Option<String>,
  log_count: u64,
}

impl ActiveWindow {
  /**
   * Check if the passed name is the current windows name
   */
  fn is(&self, other_name: String) -> bool {
    self.name == Some(other_name)
  }

  /**
   * Get log line
   */
  fn log_line(&self) -> String {
    let passed_time = self.log_count * INTERVAL;
    match &self.name {

      Some(name) => format!("{}, {}s\n", name, passed_time),

      None    => format!("\n")
    }
  }

  fn incr_count(&mut self) {
    self.log_count += 1;
  }
}

static mut ACTIVE_WINDOW: ActiveWindow = ActiveWindow {
  name: None,
  log_count: 1,
};

#[cfg(target_os = "macos")]
pub fn get_active_window() -> String {
  let script_path = app_paths::get_app_path(Some("get_active_window_mac.scpt".to_string()));
  let command = Command::new(script_path)
    .output()
    .expect("window title extraction script failed to launch");

  String::from_utf8_lossy(&command.stdout).to_string()
}

/**
 * Start our logging thread
 */
#[cfg(target_os = "macos")]
pub unsafe fn start_logging() {
  init_logging_script();

  thread::spawn(|| {
    loop {
      // remove new line
      let mut win = get_active_window();
      trim_newline(&mut win);

      // check if name is empty
      if ACTIVE_WINDOW.name == None {
        ACTIVE_WINDOW.name = Some(String::from(&win));
      }

      // check if received window is same as current
      if ACTIVE_WINDOW.is(String::from(&win)) {
        
        ACTIVE_WINDOW.incr_count();

      } else {
        // get log line
        let log_line = ACTIVE_WINDOW.log_line();

        let log_path = app_paths::activity_log_path().join(get_date_filename());

        // append new line to log path
        let mut file = OpenOptions::new()
          .write(true)
          .append(true)
          .create(true)
          .open(log_path)
          .unwrap();

        write!(file, "{}", log_line)
          .unwrap_or_else(|err| panic!("Failed to open or create file {:?}, {}", file, err));

        // re-init current window value
        ACTIVE_WINDOW.name = Some(String::from(&win));
        ACTIVE_WINDOW.log_count = 1;
      }

      // sleep x secs
      thread::sleep(Duration::from_secs(INTERVAL));
    }
  });
}

/**
 * Init logging script
 */
#[cfg(target_os = "macos")]
fn init_logging_script() {
  let script_path = app_paths::get_app_path(Some("get_active_window_mac.scpt".to_string()));

  // check if path exists
  if !Path::new(&script_path).exists() {
    use std::os::unix::fs::PermissionsExt;

    let mut file = OpenOptions::new()
			.create(true)
			.write(true)
			.open(&script_path)
			.unwrap_or_else(|err| panic!("Failed to open or create file {:?}, {}", script_path, err));
		
    let content = &include_str!("./get_active_window_mac.scpt");

    file.write_all(content.as_bytes())
			.unwrap_or_else(|err| panic!("failed to write to file {:?}, {}", script_path, err));

	  fs::set_permissions(&script_path, Permissions::from_mode(0o755))
		  .unwrap_or_else(|err| panic!("failed to set permissions for {:?}, {}", script_path, err));
  }
}

/**
 * Get formatted date to name log file
 */
fn get_date_filename() -> String {
  let now = Utc::now();

  format!(
    "{}-{:02}-{:02}_{:?}.txt",
    now.year(),
    now.month(),
    now.day(),
    now.weekday(),
  )
}

/**
 * Remove line break from string
 */
fn trim_newline(s: &mut String) {
  if s.ends_with('\n') {
    s.pop();
    if s.ends_with('\r') {
      s.pop();
    }
  }
}
