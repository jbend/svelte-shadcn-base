// @generated automatically by Diesel CLI.

diesel::table! {
    vendor (id) {
        id -> Text,
        name -> Text,
        contact -> Text,
        email -> Text,
        phone -> Text,
        created_at -> Timestamp,
    }
}
