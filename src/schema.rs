// @generated automatically by Diesel CLI.

diesel::table! {
    vendor (id) {
        id -> Text,
        name -> Text,
        created_at -> Timestamp,
    }
}
