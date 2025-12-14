mod cors;
mod middleware;
mod route;
pub mod utils;

use crate::infrastructure::{database, http_server::cors::get_cors};
use actix_web::{App, HttpServer, web};
use std::env;
use tracing;
use tracing_actix_web::TracingLogger;

#[derive(Clone)]
pub struct AppState {
  pub(crate) poll: deadpool_postgres::Pool,
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
  let poll = database::db_connection_poll().await.unwrap();
  let server_uri = format!(
    "{}:{}",
    env::var("SERVER_HOST").unwrap(),
    env::var("SERVER_PORT").unwrap()
  );

  tracing::info!("Web server REST started at {}", server_uri);
  HttpServer::new(move || {
    App::new()
      .wrap(get_cors())
      .app_data(web::Data::new(AppState { poll: poll.clone() }))
      .configure(route::config_route)
      .wrap(TracingLogger::default())
      .default_service(route::not_found())
  })
  .bind(server_uri)?
  .run()
  .await
}
