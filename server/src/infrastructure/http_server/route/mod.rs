pub mod organization;
pub mod profile;
pub mod user;

use crate::infrastructure::http_server::utils::HttpError;
use actix_web::{HttpResponse, Route, web};

pub fn config_route(cfg: &mut web::ServiceConfig) {
  cfg
    .service(profile::scope())
    .service(user::scope())
    .service(organization::scope());
}

pub fn not_found() -> Route {
  web::route().to(|| async {
    let result_error = HttpError::new("Resource not found".to_string());
    HttpResponse::BadRequest().json(result_error)
  })
}
