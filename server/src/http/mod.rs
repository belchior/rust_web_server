mod cors;
pub mod http_handler;
mod middleware;
mod route;

use crate::http::cors::get_cors;
use crate::setup::{db::db_client_connection, tracing::start_tracing};
use actix_web::{web, App, HttpServer};
use std::env;
use tracing;
use tracing_actix_web::TracingLogger;

pub struct AppState {
  db: mongodb::Database,
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
  start_tracing();

  let db = db_client_connection().await.unwrap();
  let server_uri = format!(
    "{}:{}",
    env::var("SERVER_HOST").unwrap(),
    env::var("SERVER_PORT").unwrap()
  );

  tracing::info!("Web server REST started at {}", server_uri);
  HttpServer::new(move || {
    App::new()
      .wrap(get_cors())
      .app_data(web::Data::new(AppState { db: db.clone() }))
      .configure(route::config_route)
      .wrap(TracingLogger::default())
      .default_service(route::not_found())
  })
  .bind(server_uri)?
  .run()
  .await
  .unwrap();

  opentelemetry::global::shutdown_tracer_provider();

  Ok(())
}

#[cfg(test)]
mod route_spec;
