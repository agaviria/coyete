table! {
    auth_tokens (id) {
        user_id -> Int8,
        expires_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int8,
        username -> Varchar,
        password_hash -> Bytea,
        salt-> Bytea,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
