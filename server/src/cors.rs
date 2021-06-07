use actix_cors::Cors;
use actix_web::http;

pub fn get_cors() -> Cors {
  Cors::default()
    .allowed_origin("http://localhost:3000")
    .allowed_methods(vec!["GET", "POST"])
    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
    .allowed_header(http::header::CONTENT_TYPE)
    .max_age(3600)
}
