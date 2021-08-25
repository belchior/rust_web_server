pub mod organization;
pub mod profile;
pub mod user;

use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn root() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}
