// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod models;
mod schema;
mod services;

use commands::vendor_commands::*;

fn main() {
  tauri::Builder::default()
    .setup(|_app| {
      db::init();
      Ok(())
    })
    .invoke_handler(
      tauri::generate_handler![
        // list_vendors,
        // get_vendor,
        create_vendor,
        delete_vendor
      ]
    )
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
