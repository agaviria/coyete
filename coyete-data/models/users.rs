use chrono::prelude::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Serialize, Associations)]
#[primary_key(id)]
pub struct User {
    pub id: Int8,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: Bytea,
    pub salt: Bytea,
    pub created_at: chrono::DateTime<UTC>,
    pub updated_at: Option<chrono::DateTime<UTC>>,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub email: String,
    pub password_hash: Bytea,
    pub salt: Bytea,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserLoginToken {
    user_id: Int8,
}

#[derive(Debug, Queryable, Serialize)]
#[table_name="api_tokens"]
pub struct ApiToken {
    pub id: Int8,
    pub user_id: Int8,
    pub created_at: chrono::DateTime<UTC>,
}

#[derive(Insertable)]
#[table_name="api_tokens"]
pub struct NewApiToken {
    pub user_id: Int8,
}
