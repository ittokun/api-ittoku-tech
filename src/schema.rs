// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Uuid,
        title -> Varchar,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
