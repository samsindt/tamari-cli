mod error;
mod client;
mod parser;
mod connection;

pub use client::{Client, Response};
pub use error::TamariError;
pub use connection::{Connection, TcpConnection};