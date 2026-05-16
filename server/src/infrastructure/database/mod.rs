pub mod connection;
pub mod cursor_connection;
mod model;

pub use connection::get_connection;
pub use model::{organization, follow, repository, user, utils};
pub type DBConnection = tokio_postgres::Client;
pub type QueryParam<'a> = &'a (dyn tokio_postgres::types::ToSql + Sync);

#[cfg(test)]
mod cursor_connection_spec;
