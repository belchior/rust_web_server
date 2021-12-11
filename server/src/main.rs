mod db;
mod http;
mod lib;
mod model;

use dotenv::dotenv;

fn main() -> () {
  dotenv().ok();
  env_logger::init();
  http::main();
}

#[cfg(test)]
mod db_spec;
#[cfg(test)]
mod mock;
