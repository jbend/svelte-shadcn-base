use crate::{
  db::establish_db_connection, 
  models::jobs::Job, 
  schema::jobs,
  schema::jobs::dsl
};
use diesel::prelude::*;

pub fn create_job(new_job: &Job) {
  let connection = &mut establish_db_connection();
  diesel::insert_into(jobs::table)
    .values(new_job)
    .execute(connection)
    .expect("Error saving new job");
}

pub fn list_jobs() -> Vec<Job> {
  let connection = &mut establish_db_connection();

  dsl::jobs
    .order_by(dsl::name.desc())
    .load::<Job>(connection)
    .expect("Error loading jobs")
}

pub fn count_jobs() -> i64 {
  let connection = &mut establish_db_connection();

  dsl::jobs
    .count()
    .get_result(connection)
    .expect("Error counting jobs")

}

pub fn get_job(id: &str) -> Option<Job> {
  let connection = &mut establish_db_connection();

  dsl::jobs
    .find(id)
    .first(connection)
    .optional()
    .expect("Error loading job")
}

pub fn update_job_name(id: &str, name: &str) {
  let connection = &mut establish_db_connection();

  diesel::update(dsl::jobs.find(id))
    .set(dsl::name.eq(name))
    .execute(connection)
    .expect("Error updating job name");
}

pub fn update_job_active(id: &str, active: &bool) {
  let connection = &mut establish_db_connection();

  diesel::update(dsl::jobs.find(id))
    .set(dsl::active.eq(active))
    .execute(connection)
    .expect("Error updating job active");
}

pub fn update_job_favorite(id: &str, favorite: &bool) {
  let connection = &mut establish_db_connection();

  diesel::update(dsl::jobs.find(id))
    .set(dsl::favorite.eq(favorite))
    .execute(connection)
    .expect("Error updating job favorite");
}

pub fn delete_job(id: String) {
  let connection = &mut establish_db_connection();

  diesel::delete(dsl::jobs.find(id))
    .execute(connection)
    .expect("Error deleting vendor");
}



