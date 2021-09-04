use actix_web::HttpResponse;
use log;
use serde::Serialize;

#[derive(Serialize)]
pub struct HttpError {
  message: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  status: Option<u32>,
}
impl HttpError {
  pub fn new(message: String) -> Self {
    Self {
      message: message.to_owned(),
      status: None,
    }
  }

  pub fn status(mut self, status: u32) -> Self {
    self.status = Some(status);
    self
  }
}

pub fn to_response<T, E>(result: Result<Option<T>, E>, model_name: &'static str) -> HttpResponse
where
  T: Serialize,
  E: std::fmt::Debug,
{
  if let Err(err) = result {
    log::error!("Internal Server Error: {:#?}", err);
    let result_error = HttpError::new("Internal Server Error".to_string()).status(500);
    return HttpResponse::InternalServerError().json(result_error);
  }

  let result = result.unwrap();
  if let None = result {
    log::info!("{} not found", model_name);
    let error_message = format!("{} not found", model_name);
    let result_error = HttpError::new(error_message).status(404);
    return HttpResponse::NotFound().json(result_error);
  }

  HttpResponse::Ok().json(result)
}
