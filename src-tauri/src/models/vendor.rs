use crate::schema::vendor;

use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};


#[derive(Queryable, Serialize, Insertable, Debug)]
#[diesel(table_name = vendor)]
pub struct Vendor {
    pub id: String,
    pub name: String,
    pub contact: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: NaiveDateTime,
}
#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = vendor)]
pub struct CreateVendor {
    pub name: String,
    pub contact: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = vendor)]
pub struct NewVendor<'a> {
    // pub id: String,
    pub name: &'a str,
    pub contact: Option<&'a str>,
    pub email: Option<&'a str>,
    pub phone: Option<&'a str>,
    // pub created_at: NaiveDateTime,
}
