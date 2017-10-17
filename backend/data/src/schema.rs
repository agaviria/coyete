table! {
    use diesel::types::*;
    certification_codes (id) {
        id      -> Int4,
        code    -> Varchar,
        user_id -> Nullable<Uuid>,
    }
}

table! {
    use diesel::types::*;
    users (id) {
        id          -> Uuid,
        email       -> Varchar,
        password    -> Bytea,
        is_verified -> Bool,
        created_at  -> Timestamp,
        updated_at  -> Timestamp,
    }
}

joinable!(certification_codes -> users (user_id));
