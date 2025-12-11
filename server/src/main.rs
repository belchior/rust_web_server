mod application;
mod infrastructure;

use dotenv::dotenv;

fn main() -> () {
  dotenv().ok();
  infrastructure::http_server::main().expect("start http server");
}
