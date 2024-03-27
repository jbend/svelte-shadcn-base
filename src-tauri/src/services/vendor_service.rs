use crate::{
  db::establish_db_connection, 
  models::vendor::{NewVendor, Vendor}, 
  schema::vendor,
  schema::vendor::dsl
};
use diesel::prelude::*;

pub fn init() {
// TODO: any initialization logic???
}

// pub fn create_vendor(new_vendor: &NewVendor) {
pub fn create_vendor(new_vendor: &Vendor) {
  let connection = &mut establish_db_connection();
  diesel::insert_into(vendor::table)
    .values(new_vendor)
    .execute(connection)
    .expect("Error saving new vendor");
}

pub fn list_vendors() -> Vec<Vendor> {
  let connection = &mut establish_db_connection();

  dsl::vendor
    .order_by(dsl::name.desc())
    .load::<Vendor>(connection)
    .expect("Error loading vendors")

}

// pub fn get_vendor(id: &str) -> Option<Vendor> {
//   let connection = &mut establish_db_connection();

//   dsl::vendor
//     .find(id)
//     .first(connection)
//     .optional()
//     .expect("Error loading vendor")
// }

pub fn update_vendor(id: &str, new_name: &str) {
  let connection = &mut establish_db_connection();

  diesel::update(dsl::vendor.find(id))
    .set(dsl::name.eq(new_name))
    .execute(connection)
    .expect("Error updating vendor");
}

pub fn delete_vendor(id: String) {
  let connection = &mut establish_db_connection();

  diesel::delete(dsl::vendor.find(id))
    .execute(connection)
    .expect("Error deleting vendor");
}


