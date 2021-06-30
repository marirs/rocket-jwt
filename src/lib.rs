#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_okapi;

#[macro_use]
extern crate diesel;

#[macro_use]
pub(crate) mod macros;

pub mod backends;
pub mod db;
pub mod secure;

pub mod server;

pub mod error;
pub type Result<T> = std::result::Result<T, error::Error>;
