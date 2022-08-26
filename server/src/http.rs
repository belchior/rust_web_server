mod cors;
pub mod http_handler;
mod middleware;
mod route;

use crate::db::db_client_connection;
use crate::http::cors::get_cors;
use actix_web::{web, App, HttpServer};
use log;
use std::env;

pub struct AppState {
  db: mongodb::Database,
}

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
      .app_data(web::Data::new(AppState { db: db.clone() }))
      .configure(route::config_route)
      .wrap(actix_web::middleware::Logger::new("%U"))
      .default_service(route::not_found())
  })
  .bind(server_uri)?
  .run()
  .await
}

#[cfg(test)]
mod route_spec;
