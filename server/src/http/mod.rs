mod cors;
pub mod http_handler;
mod middleware;
mod route;

use crate::http::cors::get_cors;
use crate::setup::db::db_connection_poll;
use actix_web::{web, App, HttpServer};
use deadpool_postgres;
use log;
use std::env;

#[derive(Clone)]
pub struct AppState {
  poll: deadpool_postgres::Pool,
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
  let poll = db_connection_poll().await.unwrap();
  let server_uri = format!(
    "{}:{}",
    env::var("SERVER_HOST").unwrap(),
    env::var("SERVER_PORT").unwrap()
  );

  log::info!("Web server REST started at {}", server_uri);
  HttpServer::new(move || {
    App::new()
      .wrap(get_cors())
      .app_data(web::Data::new(AppState { poll: poll.clone() }))
      .configure(route::config_route)
      .wrap(actix_web::middleware::Logger::new("%U"))
      .default_service(route::not_found())
  })
  .bind(server_uri)?
  .run()
  .await
}
