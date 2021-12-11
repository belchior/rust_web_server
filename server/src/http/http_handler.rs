use actix_web::HttpResponse;
use log;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct HttpError {
  pub message: String,
}
impl HttpError {
  pub fn new(message: String) -> Self {
    Self {
      message: message.to_string(),
    }
  }
}

pub fn to_response<T, E>(result: Result<Option<T>, E>, model_name: &'static str) -> HttpResponse
where
  T: Serialize + std::fmt::Debug,
  E: std::fmt::Debug,
{
  if let Err(err) = result {
    log::error!("Internal Server Error: {:#?}", err);
    let result_error = HttpError::new("Internal Server Error".to_string());
    return HttpResponse::InternalServerError().json(result_error);
  }

  let result = result.unwrap();
  if let None = result {
    log::info!("{} not found", model_name);
    let error_message = format!("{} not found", model_name);
    let result_error = HttpError::new(error_message);
    return HttpResponse::NotFound().json(result_error);
  }

  let result = result.unwrap();
  HttpResponse::Ok().json(result)
}
