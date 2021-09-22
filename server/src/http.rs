mod cors;
pub mod http_handler;
mod rest;

#[allow(unused_must_use)]
pub fn main() -> () {
  rest::main();
}
