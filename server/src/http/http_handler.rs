use actix_web::HttpResponse;
use log;
use serde::Serialize;

#[derive(Serialize)]
pub struct HttpError {
  pub status: Option<u32>,
  pub message: String,
}
impl HttpError {
  pub fn new(message: String, status: Option<u32>) -> Self {
    Self {
      status: match status {
        Some(_) => status,
        None => Some(500),
      },
      message: message.to_owned(),
    }
  }
}

pub fn to_response<T, E>(result: Result<Option<T>, E>, model_name: &'static str) -> HttpResponse
where
  T: Serialize,
  E: std::fmt::Debug,
{
  if let Err(err) = result {
    log::error!("Internal Server Error: {:#?}", err);
    // TODO rethink HttpError struct, maybe it is unnecessary
    return HttpResponse::InternalServerError().json(HttpError::new("Internal Server Error".to_string(), None));
  }

  let result = result.unwrap();
  if let None = result {
    log::info!("{} not found", model_name);
    let error_message = format!("{} not found", model_name);
    return HttpResponse::NotFound().json(HttpError::new(error_message, Some(404)));
  }

  HttpResponse::Ok().json(result)
}
