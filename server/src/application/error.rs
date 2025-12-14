use thiserror::Error;
use tokio_postgres::Error as ClientError;

#[derive(Error, Debug)]
pub enum AppError {
  #[error("database error")]
  Database(ClientError),
}
