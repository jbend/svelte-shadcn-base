use crate::models::jobs::Job;
use crate::services::jobs_service;
use tauri::Manager;
use uuid::Uuid;

#[tauri::command]
pub fn list_jobs() -> Vec<Job> {
    jobs_service::list_jobs()
}

#[tauri::command]
pub fn get_job(id: String) -> Option<Job> {
    jobs_service::get_job(&id)
}

#[tauri::command]
pub fn count_jobs() -> i64 {
    jobs_service::count_jobs()
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateJobRequest {
    pub name: String,
}

#[tauri::command]
pub fn create_job(app_handle: tauri::AppHandle, job_arg: CreateJobRequest) -> Job {

    print!("new_job: {:?}", job_arg);
    let new_job = Job {
        id: Uuid::new_v4().to_string(),
        name: job_arg.name.to_string(),
        active: true,
        favorite: false,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: None,
    };
    jobs_service::create_job(&new_job);
    app_handle.emit_all("job_created", ()).unwrap();
    new_job
}

#[derive(serde::Deserialize, Debug)]
pub struct UpdateJobRequest {
    pub id: String,
    pub name: String,
    pub active: bool,
    pub favorite: bool,
}

#[tauri::command]
pub fn update_job_name(app_handle: tauri::AppHandle, id: String, name: String) {
    print!("job_id: {:?}", id);
    // jobs_service::update_job_name(id.clone(), name.clone());
    app_handle.emit_all("job_updated", ()).unwrap();
}

#[tauri::command]
pub fn update_job_active(app_handle: tauri::AppHandle, id: String, active: bool) {
    print!("job_id: {:?}", id);
    // jobs_service::update_job_name(id.clone(), active);
    app_handle.emit_all("job_updated", ()).unwrap();
}

#[tauri::command]
pub fn update_job_favorite(app_handle: tauri::AppHandle, id: String, favorite: bool) {
    print!("job_id: {:?}", id);
    // jobs_service::update_job_name(id.clone(), favorite);
    app_handle.emit_all("job_updated", ()).unwrap();
}

#[tauri::command]
pub fn delete_job(app_handle: tauri::AppHandle, job_id: String) {
    jobs_service::delete_job(job_id.clone());
    app_handle.emit_all("job_deleted", ()).unwrap();
}

