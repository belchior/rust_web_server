pub mod organization;
pub mod repository;
pub mod user;
pub mod utils;

use tokio_postgres::types::ToSql;

pub type QueryParam<'a> = &'a (dyn ToSql + Sync);

#[cfg(test)]
mod organization_spec;

#[cfg(test)]
mod repository_spec;

#[cfg(test)]
mod user_spec;

#[cfg(test)]
mod utils_spec;
