use argon2;
use diesel;
use diesel::prelude::*;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::outcome::Outcome as RocketOutcome;
use rocket::http::Status;
use rwt::Rwt;
use api::response::{self, APIResponse};
use auth::{Claim, Token};
use chrono::prelude::*;
use coyete_data::persistance::schema::users::dsl::*;
use coyete_data::persistance::PgConn;
use coyete_data::models::users::User as UserModel;
use coyete_data::security;
use models::crypto;
use service;

impl UserModel {
    pub fn id(&self) -> { self.id }

    pub fn make_password_hash(new_password: &str) -> Vec<u8> {
        let secret = service::get_nonce();
        security::hash_password(&new_password, &secret)
    }

    pub fn verify_password<T: AsRef<str>>(&mut self, password: T) ->
        Result<bool, APIResponse> {
            let matches = argon2::verify_encoded(
                self.password_hash.as_str(), password.as_ref().as_bytes()).unwrap();

            if !matches {
                map_err(|_| response::conflict()
                        .message("failed to check password for {}", self.id())?);
            }

            Ok(())
        }

    pub fn generate_api_token(&self, id: &str, username: &str, conn: PgConn) ->
        Result<Token, APIResponse> {
            let claim = Claim::new(id, username);
            let salt = crypto::generate_salt().as_ref();
            let new_auth_token = Token(Rwt::with_payload(&claim, &salt)?);
            self.current_auth_token = Some(new_auth_token.clone());
            self.last_action = Some(Utc::now());
            self.save_changes::<UserModel>(conn)?;

            Ok(new_auth_token)
        }

    // TODO: fn has_valid_token(); ?
}

/// token has this format:
/// <auth token>.<signature>
impl<'a 'r> FromRequest<'a 'r> for UserContext {
    type Error = APIResponse;

    fn from_req(req: &'a Request<'r>) -> Outcome<Self, APIResponse> {
        use auth::UserToken;
        let token = req.headers()
            .get("Authorization")
            .nth(0)
            .map(|s| s[7..].parse::<UserToken>());

        match token {
            Some(Ok(token)) => {
                if !token.is_valid(service::get_nonce().unwrap()) {
                    return RocketOutcome::Failure((
                            Status::Unauthorized,
                            response::unauthorized(),
                            ));
                }

                if !token.payload().is_valid() {
                    return RocketOutcome::Failure((
                            Status::Unauthorized,
                            response::bad_token().message("payload claims failure."),
                            ));
                }

                let user_queried = users
                    .filter(id.eq(user_id))
                    .first::<UserModel>(*PgConn);

                match user.is_ok() {
                    Ok(user) => user,
                    Err(_) => return RocketOutcome::Failure((
                            Status::Unauthorized,
                            response::forbidden()
                            .message("Token expired"),
                            )),
                };

                RocketOutcome::Success(UserContext {
                    id: user.id,
                    email: token.user().into,
                })
            },

            _ => RocketOutcome::Failure((Status::Unauthorized, Error::unauthorized()))
        }
    }
}
