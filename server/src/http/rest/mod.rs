pub mod route;

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
      .data(db.clone())
      // TODO find a better way to register a route that don't require one by one router registration
      .service(route::root)
      .service(route::user::user)
      .service(route::user::repositories)
      .service(route::user::starred_repositories)
      .service(route::user::followers)
      .service(route::user::following)
      .service(route::organization::organization)
      .service(route::organization::people)
      .service(route::organization::repositories)
      // TODO makes this middleware execute only for development mode
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
