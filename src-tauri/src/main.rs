// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod models;
mod schema;
mod services;

use tauri::Manager;

use commands::vendor_commands::*;

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
        list_vendors,
        get_vendor,
        update_vendor,
        create_vendor,
        delete_vendor
      ]
    )
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
