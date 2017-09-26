use chrono::prelude::*;
use super::persistance::PgConn;

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

impl User {
    pub fn find(conn: PgConn, id: i64) -> Result<User, ()> {
        use super::persistance::schema::users;
        use super::persistance::schema::users::dsl;

        let found_users = dsl::users
            .filter(dsl::id.eq(id))
            .load::<User>(conn)
            .ok()
            .unwrap();

        if found_users.len() > 0 {
            return Ok(found_users.first().unwrap().clone());
        } else {
            return Err(());
        }
    }

    pub fn find_by_login(conn: PgConn, _username: &str, _password: &str) -> Result<User, ()> {
        use super::persistance::schema::users;
        use super::persistance::schema::users::dsl;

        let found_users = dsl::users
            .filter(dsl::username
                        .eq(_username)
                        .and(dsl::password.eq(_password)))
            .load::<User>(conn)
            .ok()
            .unwrap();

        if found_users.len() > 0 {
            return Ok(found_users.first().unwrap().clone());
        } else {
            return Err(());
        }
    }
}
