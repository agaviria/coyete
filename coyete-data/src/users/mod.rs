use persistance::PgConn;
use persistance::schema::users;
use peristance::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;
use self::models::{NewUser, User};

pub mod users;
pub mod models;


/// creates a new user in our database
/// Usernames are unique and will return a Result(Err) if there is a duplicate.
/// Hash and Salt should be generated in the coyete::security module.
pub fn create_user(conn: PgConn,
                   username: String,
                   hash: String,
                   salt: String)
                   -> Result<i32, Error> {
    let new_user = NewUser {
        username: username,
        password_hash: password_hash,
        salt: salt,
    };

    match diesel::insert(&new_user)
              .into(users::table)
              .execute(conn) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    match users::table.order(users::id.desc()).first::<User>(conn) {
        Ok(user) => Ok(user.id),
        Err(e) => Err(e),
    }
}


/// finds user through search of username in the database.
pub fn find_user(conn: PgConn, _username: &String) -> Option<User> {
    match users::table
              .filter(users::username.eq(_username))
              .first::<User>(conn) {
        Ok(stored_user) => Some(stored_user),
        Err(_) => None,
    }
}
