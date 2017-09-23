table! {
    api_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password_hash -> Varchar,
        salt-> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
