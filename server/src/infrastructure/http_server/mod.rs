mod cors;
mod middleware;
mod route;
pub mod utils;

use crate::infrastructure::telemetry;
use crate::infrastructure::{database::db_client_connection, http_server::cors::get_cors};
use actix_web::{App, HttpServer, web};
use std::env;
use tracing;
use tracing_actix_web::TracingLogger;

pub struct AppState {
  pub(crate) db: mongodb::Database,
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
  telemetry::start_traicing();

  let db = db_client_connection().await.unwrap();
  let server_uri = format!(
    "{}:{}",
    env::var("SERVER_HOST").unwrap(),
    env::var("SERVER_PORT").unwrap()
  );

  tracing::info!("HTTP server started at {}", server_uri);
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

  Ok(())
}
