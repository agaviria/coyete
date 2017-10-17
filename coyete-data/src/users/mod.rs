use chrono::NaiveDateTime;
use uuid::Uuid;
use schema::users;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    email: &'a str,
    password: Vec<u8>,
}

#[derive(Associations, Identifiable, Queryable)]
#[primary_key(id)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: Vec<u8>,
    pub is_verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
