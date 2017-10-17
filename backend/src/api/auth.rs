use rocket_contrib::Json;
use diesel;
use diesel::prelude::*;

use coyete_data::persistance::PgConn;
use coyete_data::schema::users::dsl::*;
use coyete_data::users::models::{User, NewUser};


#[post("/register", data = "<user>", format = "application/json")]
pub fn register(user: Json<UserSerializer>, conn: PgConn) -> APIResponse {
    unimplemented!()
}

//#[post("/register", data = "<user>")]
//pub fn register(post: Option<Form<NewUser>>, conn: PgConn) -> APIResult<User> {
//    users.map_or(APIResult::from_validation_error("User"), |users| {
//        diesel::insert(users.get()).into(users)
//            .get_result::<User>(conn)
//            .map(|new_user| APIResult::new(Status::Created, "New user successfully created", new_user))
//            .unwrap_or_else(|err| APIResult::from_database_error(err, "User"))
//    })
//}
