mod application;
mod infrastructure;

use dotenvy::dotenv;

fn main() -> () {
  dotenv().ok();
  infrastructure::telemetry::start_tracing();
  infrastructure::http_server::main().expect("start http server");
}
