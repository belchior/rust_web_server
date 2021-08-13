pub mod route;

use crate::db::db_client_connection;
use crate::http::cors::get_cors;
use actix_web::{App, HttpServer};
use log;
use std::env;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
  let db = db_client_connection().await.unwrap();
  let server_uri = format!(
    "{}:{}",
    env::var("SERVER_HOST").unwrap(),
    env::var("SERVER_PORT").unwrap()
  );

  log::info!("Web server REST started at {}", server_uri);
  HttpServer::new(move || {
    App::new()
      .wrap(get_cors())
      .data(db.clone())
      .service(route::root)
      .service(route::user::user)
      .service(route::repositories::repositories)
  })
  .bind(server_uri)?
  .run()
  .await
}
