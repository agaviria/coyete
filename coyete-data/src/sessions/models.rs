use schema::*;
use users::models::User;

#[derive(Debug, Identifiable, Queryable, Associations, AsChangeSet)]
#[table_name="sessions"]
#[belongs_to(User, foreign_key = "user_id")]
#[changeset_options(treat_none_as_null = "true")]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub started: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub secret: Vec<u8>,
}

#[derive(Debug, Insertable)]
#[table_name="sessions"]
pub struct NewSession<'a> {
    pub user_id: i32,
    pub started: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub secret: &'a [u8],
}
