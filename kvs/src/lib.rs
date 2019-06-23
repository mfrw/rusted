#[macro_use]
extern crate log;

pub use error::{KvsError, Result};

mod client;
mod common;
mod engines;
mod error;
mod server;
