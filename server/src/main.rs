mod http;
mod lib;
mod model;
mod setup;

use dotenv::dotenv;

fn main() -> () {
  dotenv().ok();
  http::main().expect("start http server");
}
