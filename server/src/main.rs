mod application;
mod infrastructure;

use dotenvy::dotenv;

#[tokio::main]
async fn main() -> () {
  dotenv().ok();
  infrastructure::telemetry::start_tracing();
  infrastructure::database::get_connection().await;
  infrastructure::http_server::main().await.expect("start http server");
}

#[cfg(test)]
mod all_tests_that_depends_on_db;
