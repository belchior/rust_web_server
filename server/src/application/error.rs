use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
  #[error("database error")]
  Database(mongodb::error::Error),
}
