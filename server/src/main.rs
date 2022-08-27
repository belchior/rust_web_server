mod http;
mod lib;
mod model;
mod setup;

use dotenv::dotenv;

fn main() -> () {
  dotenv().ok();
  env_logger::init();
  http::main().expect("start http server");
}
