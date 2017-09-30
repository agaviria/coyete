// use chrono::{self, Utc};
// use diesel::result::Error as DieselError;
// use diesel::pg::PgConnection;
// use diesel::prelude::*;
use schema::*;
use users::models::User;

#[derive(Debug, Queryable, Identifiable, Insertable, Associations, AsChangeset)]
#[belongs_to(User, foreign_key = "id")]
#[table_name = "auth"]
pub struct Auth {
    pub id: i32,
    pub salt: Vec<u8>,
    pub password_hash: Vec<u8>,
}


// impl Auth {
//     pub fn get_id(conn: &PgConnection, get_id: i32) -> Result<Auth, DieselError> {
//         use diesel::{ExpressionMethods, FilterDsl};
//         use schema::auth;
//         use schema::auth::dsl::id;

//         // let auth_id = id.parse::<i32>().unwrap();
//         auth.filter(id.eq(get_id)).first::<Auth>(conn);
//     }
// }

// impl NewAuth {
//     pub fn new(pwd_string: &str) -> NewAuth {
//         use crypto;

//         let salt = crypto::generate_salt().as_ref();
//         let hashed_password: Vec<u8> = crypto::hash_password(pwd_string, salt);

//         NewAuth {
//             salt: salt.to_vec(),
//             password_hash: hashed_password,
//         }
//     }

//     pub fn save(&self, conn: &PgConnection) -> Result<Auth, DieselError> {
//         use diesel::insert;
//         use schema::auth::dsl::*;

//         insert(self)
//             .into(auth::table)
//             .get_result(conn)
//             .expect("Error! Failed to save credentials into auth table.")
//     }
// }
