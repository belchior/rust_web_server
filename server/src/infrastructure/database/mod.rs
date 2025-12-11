pub mod connection;
pub mod cursor_connection;
mod model;

pub use connection::db_client_connection;
pub use model::{organization, repository, user};
pub type DBConnection = mongodb::Database;

#[cfg(test)]
mod cursor_connection_spec;

#[cfg(test)]
mod connection_spec;
