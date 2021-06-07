use serde::Serialize;

#[derive(Serialize)]
pub struct HttpError {
  pub status: i32,
  pub message: String,
}
