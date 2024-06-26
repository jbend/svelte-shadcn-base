use crate::models::vendor::{CreateVendor, Vendor};
use crate::services::vendor_service;
use tauri::Manager;
use uuid::Uuid;

#[tauri::command]
pub fn list_vendors() -> Vec<Vendor> {
    vendor_service::list_vendors()
}

#[tauri::command]
pub fn get_vendor(id: String) -> Option<Vendor> {
    vendor_service::get_vendor(&id)
}

#[tauri::command]
pub fn count_vendors() -> i64 {
    vendor_service::count_vendors()
}

#[tauri::command]
pub fn create_vendor(app_handle: tauri::AppHandle, vendor_arg: CreateVendor) -> Vendor {

    print!("new_vendor: {:?}", vendor_arg);
    // let new_vendor = NewVendor {
    let new_vendor = Vendor {
        id: Uuid::new_v4().to_string(),
        name: vendor_arg.name.to_string(),
        contact: vendor_arg.contact,
        email: vendor_arg.email,
        phone: vendor_arg.phone,
        created_at: chrono::Utc::now().naive_utc(),
    };
    vendor_service::create_vendor(&new_vendor);
    app_handle.emit_all("vendor_created", ()).unwrap();
    new_vendor
}

#[tauri::command]
pub fn update_vendor(app_handle: tauri::AppHandle, vendor_id: String) {
    print!("vendor_id: {:?}", vendor_id);
    // vendor_service::delete_vendor(vendor_id.clone());
    app_handle.emit_all("vendor_updated", ()).unwrap();
}

#[tauri::command]
pub fn delete_vendor(app_handle: tauri::AppHandle, vendor_id: String) {
    vendor_service::delete_vendor(vendor_id.clone());
    app_handle.emit_all("vendor_deleted", ()).unwrap();
}

