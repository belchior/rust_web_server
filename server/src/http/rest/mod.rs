mod route;

use crate::db::db_client_connection;
use crate::http::cors::get_cors;
use actix_web::dev::Service;
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
      // TODO convert this to AppState { db: Database }
      .data(db.clone())
      .service(route::profile::scope())
      .service(route::user::scope())
      .service(route::organization::scope())
      // TODO put this middleware out of the mod file and configure logs base on environment
      .wrap_fn(|req, srv| {
        log::info!("Request {} {}", req.method(), req.uri());
        let fut = srv.call(req);
        async {
          let res = fut.await?;
          Ok(res)
        }
      })
  })
  .bind(server_uri)?
  .run()
  .await
}
