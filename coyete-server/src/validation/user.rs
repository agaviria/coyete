use rocket::data::{self, FromData};
use rocket::{Request, Data};
use rocket::http::Status;
use rocket::Outcome::*;
use rocket_contrib::JSON;
use serde_json::Value;
use std::collections::HashMap;

use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UserSerializer {
    #[serde(skip_deserializing)]
    pub id: Option<i64>,
    #[validate(email)]
    pub username: String,
    pub password: String,
}

impl FromData for UserSerializer {
    type Error = Value;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, Value> {
        let user = JSON::<UserSerializer>::from_data(req, data).map_failure(|_| {
                             (Status::UnprocessableEntity,
                              json!({"_schema": "Error while serialzing."}))
                         })?;

        let mut errors = HashMap::new();
        if user.username == "" {
            errors
                .entry("username")
                .or_insert(vec![])
                .push("Must not be empty.");
        }

        if user.password == "" {
            errors
                .entry("password")
                .or_insert(vec![])
                .push("Must not be empty.");
        }

        if !errors.is_empty() {
            return Failure((Status::UnprocessableEntity, json!(errors)));
        }

        return Success(UserSerializer {
                           id: None,
                           username: user.username.clone(),
                           password: user.password.clone(),
                       });
    }
}
