mod cors;
mod cursor_connection;
mod db;
mod model;
mod repository;
mod route;

use actix_web::{App, HttpServer};
use cors::get_cors;
use db::get_connection;
use dotenv::dotenv;

use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
      .service(route::hello)
      .service(route::user::user)
  })
  .bind(server_uri)?
  .run()
  .await
}
