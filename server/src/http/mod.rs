mod cors;
pub mod cursor_connection;
pub mod http_handler;
mod rest;

pub fn main() -> () {
  rest::main();
}
