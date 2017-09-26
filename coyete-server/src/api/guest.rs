use diesel::prelude::*;
use diesel;
use response::{self, APIResponse};
use rocket_contrib::JSON;
use coyete_data::models::users;
use coyete_data::persistance::schema::users;
use coyete_data::persistance::schema::users::dsl::*;
use coyete_data::persistance::PgConn;
use validation::user::UserSerializer;
use super::middleware::Auth;

use coyete_data::models::users::{User, NewUser};
use super::RuntimeConfig;

/// Log the user in and return a response with an auth token.
///
/// Return UNAUTHORIZED in case the user can't be found or if the password is incorrect.
#[post("/login", data = "<user_in>", format = "application/json")]
pub fn login(conn: PgConn, user_in: JSON<UserSerializer>,
             rconfig: State<RuntimeConfig>) -> APIResponse {
    let user_query = users
        .filter(username.eq(user_in.username.clone()))
        .first::<User>(&*conn);

    let mut user = user_query.ok_or(response::unauthorized().message(
            "username or password is incorrect")?);
    if !user.verify_password(user_in.password.as_str()) {
        return response::unauthorized().message(
            "username or password is incorrect");
    }

    // TODO: *pending token below
    match kkkkkkkkkk

        let token = unimplemented!();

    Ok(token) => {
        let response = TokenResponse::from_token(token)
            .map_err(|_| Error::new(ErrorKind::InternalServerError, "Unencodable token"))?;

        Ok(Json(response))
    }
}

/// Creates a new user in the database. Usernames are unique and will return a
/// conflict API response error if there is a duplicate.  Hash and salt methods
/// can be found in the crypto module.
///
#[post("/register", data = "<user>", format = "application/json")]
pub fn create_user(conn: PgConn,
                   username: String,
                   password_hash: Vec<u8>,
                   salt: Vec<u8>)
    -> Result<i32, APIResponse> {
        let user_data = user.map_err(response::unprocessable_entity)?;
        let new_user = NewUser {
            username: username,
            hash: hash.as_bytes(),
            salt: salt.as_bytes(),
        };

        let results = users
            .filter(username.eq(user_data.username.clone()))
            .first::<User>(&*conn);

        match results.is_ok() {
            Ok(_) => {}
            Err(e) => return Err(e.response::conflict()
                                 .message("User already exists.")),
        }

        let user = diesel::insert(&new_user).into(users::table)
            .get_result::<User>(&*conn)?;

        Ok(response::created().data(json!(&user)))
    }

/// returns username of the current user when invoked.
#[get("/whoami")]
fn whoami(conn: PgConn, auth: Auth) -> APIResponse {
    let user = users::User::find(&conn, auth.user_id).unwrap();

    Ok(user) => {
        let response = TokenResponse::from_token(token)
            .map_err(|_| Error::new(ErrorKind::InternalServerError, "Unencodable token"))?;

        Ok(Json(response))
    }


}
