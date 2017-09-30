table! {
    auth (id) {
        id -> Int4,
        salt -> Bytea,
        password_hash -> Bytea,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    sessions (id) {
        id -> Int4,
        proposed_token -> Bytea,
        current_token -> Bytea,
        retired_token -> Bytea,
        access_version -> Int4,
        user_id -> Int4,
        started -> Timestamp,
        last_seen -> Timestamp,
        last_ip -> Bytea,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
joinable!(auth -> users (id));
joinable!(sessions -> users (user_id));
