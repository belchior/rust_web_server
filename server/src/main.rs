mod db;
mod http;
mod lib;
mod model;
mod repository;

use dotenv::dotenv;

fn main() -> () {
  dotenv().ok();
  env_logger::init();
  http::main();
}
