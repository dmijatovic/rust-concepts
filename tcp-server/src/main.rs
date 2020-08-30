use std::env;

mod server;
mod http;

use server::Server;
use http::WebHandler;

fn main() {
  println!("Hello, world!");
  let default_path = format!("{}/public",env!("CARGO_MANIFEST_DIR"));
  // get env var if provided otherwise use default
  let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
  //create server
  let srv = Server::new("localhost:8080".to_string());
  //listen
  srv.listen(WebHandler::new(public_path));
}
