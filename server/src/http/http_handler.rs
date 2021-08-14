use serde::Serialize;

#[derive(Serialize)]
pub struct HttpError {
  pub status: Option<u32>,
  pub message: String,
}
impl HttpError {
  pub fn new(message: &'static str, status: Option<u32>) -> Self {
    Self {
      status: match status {
        Some(_) => status,
        None => Some(500),
      },
      message: message.to_owned(),
    }
  }
}
