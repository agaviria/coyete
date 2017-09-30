use chrono::prelude::*;
use uuid::Uuid;
use schema::*;

#[derive(Debug, Queryable, Associations, Identifiable)]
#[primary_key(id)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub uuid_: Uuid,
    pub level_: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub email: String,
    pub uuid_: Uuid,
    pub level_: i32,
}
