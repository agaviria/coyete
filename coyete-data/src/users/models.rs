use chrono::prelude::*;
use persistance::schema::*;

#[derive(Identifiable, Queryable, Serialize, Associations, Clone, Debug)]
#[has_many(auth_tokens)]
#[primary_key(id)]
pub struct User {
    pub id: Int8,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: Bytea,
    pub salt: Bytea,
    pub created_at: chrono::DateTime<UTC>,
    pub updated_at: Option<chrono::DateTime<UTC>>,
}

#[derive(Insertable, Clone, Debug)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password_hash: Bytea,
    pub salt: Bytea,
}

#[derive(Queryable, Serialize, Deserialize, Insertable, Clone, Debug)]
#[belongs_to(User)]
#[table_name="auth_tokens"]
pub struct AuthToken {
    pub user_id: Int8,
    pub auth_token: Bytea,
    pub expires_at: chrono::DateTime<UTC>,
}
