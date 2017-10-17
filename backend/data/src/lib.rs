#![feature(plugin, custom_derive, custom_attribute)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate argon2;
extern crate chrono;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rand;
extern crate dotenv;
extern crate uuid;
extern crate serde_json;

pub mod certification;
pub mod persistance;
pub mod schema;
pub mod users;
