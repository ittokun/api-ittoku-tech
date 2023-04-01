// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Uuid,
        body -> Text,
        post_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Uuid,
        title -> Varchar,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(comments -> posts (post_id));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    posts,
);
