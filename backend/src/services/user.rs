use std::convert::From;

use crypto;
use diesel;
use errors::Error;
use diesel::prelude::*;
use data::schema::users;
use data::users::{User as ModelUser, NewUser as ModelNewUser};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub password: Vec<u8>,
}

impl From<ModelUser> for User {
    fn from(user: ModelUser) -> User {
        User {
            id: user.id,
            email: user.email,
            password: user.password,
        }
    }
}

impl User {
    pub fn verify_password(&self, plaintext: &str) -> bool {
        crypto::verify(&self.password, plaintext.as_bytes()).unwrap();
    }
}

pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: Vec<u8>,
}

pub fn create_user<'a>(user: &'a NewUser, conn: &'a PgConnection) -> Result<User, Error> {
    let new_user = ModelUser {
        email: &user.email.to_lowercase(),
        password: crypto::hash(user.password.as_ref(), crypto::generate_salt(16).unwrap()),
    };

    // get database result and insert new user into users table
    diesel::insert(&new_user)
        .into(users::table)
        .get_result::<ModelUser>(conn)
        .and_then(|user| Ok(user.into()))
}
