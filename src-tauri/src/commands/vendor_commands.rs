use crate::models::vendor::{NewVendor, Vendor};
use crate::services::vendor_service;
use tauri::Manager;
use uuid::Uuid;

// #[tauri::command]
// pub fn list_vendors() -> Vec<Vendor> {
//     vendor_service::list_vendors()
// }

// #[tauri::command]
// pub fn get_vendor(id: String) -> Option<Vendor> {
//     vendor_service::get_vendor(&id)
// }

#[tauri::command]
pub fn create_vendor(new_vendor: NewVendor) -> Vendor {
    let new_vendor = NewVendor {
        id: Uuid::new_v4().to_string(),
        name: new_vendor.name,
        created_at: chrono::Utc::now().naive_utc(),
    };

    vendor_service::create_vendor(&new_vendor);

    new_vendor.into()
}

#[tauri::command]
pub fn delete_vendor(app_handle: tauri::AppHandle, vendor_id: String) {
    vendor_service::delete_vendor(vendor_id.clone());

    // app_handle.emit_all("vendor_deleted", {}).unwrap();
}
