use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use pretty_env_logger;

mod handlers;
mod response;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // enable logger
  std::env::set_var("RUST_LOG", "actix_web=info");
  pretty_env_logger::init();

  let host = String::from("127.0.0.1:8080");

  HttpServer::new(|| {
    App::new()
      .wrap(Logger::default())
      .service(handlers::index)
      .service(handlers::get_todolists)
      .service(handlers::add_todolist)
      .service(handlers::update_todolist)
      .service(handlers::get_todoitem)
      .service(handlers::add_todoitem)
  })
  .bind(host)?
  .workers(1) // <- Start 2 workers
  .run()
  .await
}