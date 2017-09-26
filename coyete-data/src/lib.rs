#![feature(plugin)]
#![feature(custom_derive)]
#![feature(custom_attribute)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate chrono;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate r2d2_postgres;
extern crate dotenv;

pub mod persistance;
pub mod users;
