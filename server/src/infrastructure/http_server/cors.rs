use actix_cors::Cors;
use actix_web::http;
use std::env;

pub fn get_cors() -> Cors {
  let origins = env::var("CORS_ALLOWED_ORIGINS").unwrap();
  Cors::default()
    .allowed_origin(origins.as_str())
    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
    .allowed_header(http::header::CONTENT_TYPE)
    .max_age(3600)
}
