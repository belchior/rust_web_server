pub mod organization;
pub mod repository;
pub mod user;
pub mod utils;

use tokio_postgres::types::ToSql;

pub type QueryParam<'a> = &'a (dyn ToSql + Sync);

#[cfg(test)]
mod utils_spec;
