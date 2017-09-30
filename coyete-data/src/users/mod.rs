use persistance::PgConn;
use diesel::prelude::*;
// use diesel::{ExpressionMethods, LoadDsl, FilterDsl};
use diesel::result::Error as DieselError;
// use uuid::Uuid;
use self::models::User;

pub mod models;

pub fn get_user(conn: &PgConn, user_id: i32) -> Result<User, DieselError> {
    use schema::users::dsl::*;

    Ok(users.filter(id.eq(user_id)).first(&**conn)?)
}


// impl NewUser {
//     pub fn new(email: &str, auth: &Auth, level_: i32) -> NewUser {
//         let uuid_ = Uuid::new_v4();
//         NewUser {
//             auth_id: auth.id,
//             email: email.into(),
//             uuid_: uuid_,
//             level_: level_,
//         }
//     }

//     pub fn save(&self, conn: PgConn) -> Result<User, DieselError> {
//         use diesel::insert;
//         use schema;

//         insert(self)
//             .into(schema::users::table)
//             .get_result(&*conn)
//             .expect("Error! Failed to save new user.")
//     }
// }

// pub fn get_user_by_email(conn: PgConn, get_email: &str) -> Result<User, DieselError> {
//     use schema::users::dsl::*;

//     users
//         .filter(email.eq(get_email))
//         .get_result::<User>(&*conn)
// }
