mod db;
mod http;
mod model;
mod repository;

fn main() -> std::io::Result<()> {
  http::main()
}
