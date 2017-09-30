// #![feature(plugin, custom_derive, custom_attribute)]
// #![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate chrono;
extern crate r2d2;
extern crate r2d2_diesel;
// extern crate argon2;
// extern crate rand;
extern crate dotenv;
extern crate uuid;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;

pub mod persistance;
pub mod users;
pub mod auth;
pub mod schema;
