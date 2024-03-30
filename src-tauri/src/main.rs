// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod models;
mod schema;
mod services;

use tauri::Manager;

use commands::vendor_commands::*;
use commands::job_commands::*;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      #[cfg(debug_assertions)]
      {
        let window = app.get_window("main").unwrap();
        window.open_devtools();
      }
      db::init();
      Ok(())
    })
    .invoke_handler(
      tauri::generate_handler![
        // Vendors
        list_vendors,
        get_vendor,
        count_vendors,
        update_vendor,
        create_vendor,
        delete_vendor,
        // Jobs
        list_jobs,
        get_job,
        count_jobs,
        update_job_name,
        update_job_active,
        update_job_favorite,
        create_job,
        delete_job
      ]
    )
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
