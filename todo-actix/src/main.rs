// system
use std::io;
// third party
use actix_web::{HttpServer, App};

// local modules
mod config;
mod models;
mod handlers;
mod db;

use dotenv::dotenv;
use tokio_postgres::NoTls;
// use crate::handlers::*;

#[actix_rt::main]
async fn main() -> io::Result<()>{
  // load env variables
  dotenv().ok();
  let config = crate::config::Config::from_env().unwrap();

  // create pg connection pool
  let pool = config.pg.create_pool(NoTls).unwrap();

  //start server
  println!("Starting server on http://{}:{}",config.server.host,config.server.port);

  HttpServer::new(move || {
    App::new()
      .data(pool.clone())
      .service(handlers::home)
      .service(handlers::create_todo_list)
      .service(handlers::get_todos)
  })
  .bind(format!("{}:{}",config.server.host,config.server.port))?
  .run()
  .await
}
