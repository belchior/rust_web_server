pub mod organization;
pub mod profile;
pub mod user;

use crate::http::http_handler::HttpError;
use actix_web::{web, HttpResponse, Route};

pub fn not_found() -> Route {
  web::route().to(|| {
    let result_error = HttpError::new("Resource not found".to_string());
    HttpResponse::BadRequest().json(result_error)
  })
}
