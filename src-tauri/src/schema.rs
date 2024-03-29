// @generated automatically by Diesel CLI.

diesel::table! {
    vendor (id) {
        id -> Text,
        name -> Text,
        contact -> Nullable<Text>,
        email -> Nullable<Text>,
        phone -> Nullable<Text>,
        created_at -> Timestamp,
    }
}
