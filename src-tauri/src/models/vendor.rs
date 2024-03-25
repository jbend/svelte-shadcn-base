use crate::schema::vendor;

use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};


#[derive(Queryable, Serialize)]
pub struct Vendor {
    pub id: String,
    pub name: String,
    pub contact: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = vendor)]
pub struct NewVendor {
    pub id: String,
    pub name: String,
    // pub contact: Option<String>,
    // pub email: Option<String>,
    // pub phone: Option<String>,
    pub created_at: NaiveDateTime,
}

// #[derive()]
// #[diesel(table_name = vendor)]
// pub struct UpdateVendor {
//     pub id: String,
//     pub name: String,
//     pub created_at: NaiveDateTime,
// }

impl From<NewVendor> for Vendor {
    fn from(new_vendor: NewVendor) -> Self {
        Vendor {
            id: new_vendor.id,
            name: new_vendor.name,
            contact: None,
            email: None,
            phone: None,
            created_at: new_vendor.created_at,
        }
    }
}

