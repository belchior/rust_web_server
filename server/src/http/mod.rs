mod cors;
pub mod cursor_connection;
pub mod http_handler;
mod rest;

use crate::db::get_connection;
use actix_web::{App, HttpServer};
use cors::get_cors;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
  dotenv().ok();
  let db = get_connection().await.unwrap();
  let server_uri = format!(
    "{}:{}",
    env::var("SERVER_HOST").unwrap(),
    env::var("SERVER_PORT").unwrap()
  );

  println!("\nstarted at {}", server_uri);
  HttpServer::new(move || {
    App::new()
      .wrap(get_cors())
      .data(db.clone())
      .service(rest::route::root)
      .service(rest::route::user::user)
      .service(rest::route::repositories::repositories)
  })
  .bind(server_uri)?
  .run()
  .await
}
