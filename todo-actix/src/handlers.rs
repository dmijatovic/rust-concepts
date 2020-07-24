use actix_web::{get, post};
use actix_web::{Responder, web, HttpResponse};
use deadpool_postgres::{Pool, Client};

use crate::models::{Status,CreateTodoList};
use crate::db;


// home returns json as string
// no return needed
#[get("/")]
pub async fn home()->impl Responder{
  // "{\"status\":\"202\"}"
  web::HttpResponse::Ok().json(Status {status: "OK".to_string() })
}


// todos route
#[get("/todos")]
pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {

  let client: Client = db_pool.get().await
    .expect("Error connecting to the database");

  let result = db::get_todos(&client).await;

  match result {
    Ok(todos) => HttpResponse::Ok().json(todos),
    Err(_) => HttpResponse::InternalServerError().into()
  }
}

#[post("/todos")]
pub async fn create_todo_list(db_pool: web::Data<Pool>, json: web::Json<CreateTodoList>) -> impl Responder {

  let client: Client = db_pool.get().await
    .expect("Error connecting to the database");

  let result = db::create_todo_list(&client, json.title.clone()).await;

  match result {
    Ok(todos) => HttpResponse::Ok().json(todos),
    Err(_) => HttpResponse::InternalServerError().into()
  }
}


// todos route
#[get("/todos/{list_id}/items")]
pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {

  let client: Client = db_pool.get().await
    .expect("Error connecting to the database");

  let result = db::get_items(&client, path.0).await;

  match result {
    Ok(items) => HttpResponse::Ok().json(items),
    Err(_) => HttpResponse::InternalServerError().into()
  }
}