use actix_web::{App, HttpServer};

mod handlers;
mod response;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let host = String::from("127.0.0.1:8080");

  HttpServer::new(|| {
    App::new()
      .service(handlers::index)
      .service(handlers::get_todos)
      .service(handlers::add_todolist)
      .service(handlers::get_todolist)
      .service(handlers::get_todoitem)
  })
  .bind(host)?
  .run()
  .await
}