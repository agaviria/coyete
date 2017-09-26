use self::rocket::Outcome;
use self::rocket::http::Status;
use self::rocket::request::{self, Request, FromRequest};
use chrono::Duration;
use coyete_data::models::*;
use coyete_data::persistance::PgConn;
// use helpers::*; // use crypto module here
use self::diesel::prelude::*;
use std::time::{Instant, UNIX_EPOCH};
use std::time::SystemTime;

#[derive(Debug)]
pub struct Auth {
    pub token: String,
    pub user_id: i64,
}

fn authenticate(conn: PgConn, _token: &str) -> Result<Auth, String> {
    use coyete_data::persistance::schema::auth_tokens;
    use coyete_data::persistance::schema::users;
    use coyete_data::persistance::schema::auth_tokens::dsl::*;
    use coyete_data::persistance::schema::users::dsl::*;

    let auth_token = auth_tokens
        .filter(token.eq(_token))
        .load::<AuthToken>(conn)
        .ok()
        .unwrap();

    if auth_token.len() > 0 {
        let auth_token = auth_token.first().unwrap();
        let now = epoch_now() as i64;

        if now >= auth_token.expires_at {
            return Err(String::from("Token has been expired"));
        } else {
            let _: AuthToken = diesel::update(auth_tokens.find(_token))
                .set(expires_at.eq(now + chrono::Duration::seconds(3600)))
                .get_result(conn)
                .expect("Error increasing token expiry");
        }

        let user_existence: i64 = users
            .filter(users::id.eq(auth_token.user_id))
            .count()
            .get_result(conn)
            .unwrap_or(0) as i64;

        if user_existence == 0 {
            return Err(String::from("User does not exist"));
        }

        let auth = Auth {
            token: auth_token.token.to_string(),
            user_id: auth_token.user_id,
        };
        return Ok(auth);
    } else {
        return Err(String::from("Token is invalid"));
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, ()> {
        let tokens: Vec<_> = request.headers().get("x-authorization").collect();
        if tokens.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let token = tokens.first().unwrap();
        let conn = establish_connection();

        let result = authenticate(&conn, token);

        match result {
            Ok(auth) => return Outcome::Success(auth),
            Err(_msg) => return Outcome::Failure((Status::Unauthorized, ())), // TODO: log the message
        }
    }
}
