mod organization;
mod profile;
mod user;

use crate::http::http_handler::HttpError;
use actix_web::{web, HttpResponse, Route};

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

#[cfg(test)]
mod organization_spec;
#[cfg(test)]
mod profile_spec;
#[cfg(test)]
mod user_spec;
