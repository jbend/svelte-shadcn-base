use crate::schema::jobs;

use chrono::NaiveDateTime;
use diesel::{query_builder::AsChangeset, Insertable, Queryable};
use serde::Serialize;

#[derive(Queryable, Serialize, Insertable, Debug)]
#[diesel(table_name = jobs)]
pub struct Job {
    pub id: String,
    pub name: String,
    pub active: bool,
    pub favorite: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = jobs)]
pub struct UpdateJob {
    pub id: String,
    pub name: Option<String>,
    pub active: Option<bool>,
    pub favorite: Option<bool>,
}

