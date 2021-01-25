#![feature(decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket_contrib;

pub mod auth;
pub mod db;
pub mod options;
pub mod routes;
pub mod util;
