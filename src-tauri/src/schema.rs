// @generated automatically by Diesel CLI.

diesel::table! {
    jobs (id) {
        id -> Text,
        name -> Text,
        active -> Bool,
        favorite -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

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

diesel::allow_tables_to_appear_in_same_query!(
    jobs,
    vendor,
);
