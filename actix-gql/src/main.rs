use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
// use pretty_env_logger;

mod db;
mod handler;
mod schema;
// mod response;
// mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // enable logger
  std::env::set_var("RUST_LOG", "actix_web=info");
  pretty_env_logger::init();

  // define host
  let host = String::from("127.0.0.1:8080");
  println!("Starting http server at {:?}", host);

  // create db connection
  let pool = db::create_pool().await;
  // let todo = db::get_todos(&pool).await;
  // println!("main todo: {:?}", todo);

  //start http server
  HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .data(pool.clone())
      .configure(handler::register)
      .default_service(web::to(|| async { "404 Page not found".to_string() }))
  })
  .bind(host)?
  .workers(1) // <- Start 2 workers
  .run()
  .await
}
