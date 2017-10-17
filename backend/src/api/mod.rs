use accord::{Accord, Result as AccordResult, MultipleError};
use accord::validators{length, contains};
use rocket::data::{self, FromData};
use rocket::{Request, Data, Outcome};
use rocket::contrib::JSON;

use coyete_data::users::{User, NewUser};

pub mod auth;

// `User` has implemented the `Accord` trait, we can simply do the following,
// which return a collection form of `rules!` macro as `Result<(), MultipleError>`,
// we then serialize to JSON using Serde and print:
impl Accord for User {
    fn validate_login(&self) -> AccordResult {
        rules!{
            "email" => self.email => [length(2..64), contains("@"), contains(".")],
            "password" => self.password => [length(7..24)]
        }

    }
}

impl FromData for User where User: Accord {
    type Error = MultipleError;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        let json = JSON::<User>::from_data(&r, data).unwrap();
        let user = json.unwrap();
        if let Err(error) = user.validate_login() {
            Outcome::Failure((Status::from_code(422).unwrap(), error))
        } else {
            Outcome::Success(user)
        }
    }
}
