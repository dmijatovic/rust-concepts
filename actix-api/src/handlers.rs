use actix_web::{web, get, post, put, Responder, HttpResponse};
// use std::path::PathBuf;
// mod response;

use crate::response::{ApiResponse, ApiResponse2};
use crate::models::{TodoList, TodoItem, BaseTodoList, TodoItemData};

#[get("/")]
pub async fn index() -> impl Responder {
  let resp = ApiResponse{
    status:200,
    status_text: String::from("OK"),
    payload: String::from("This is your name"),
  };
  //return
  format!("{:?}", resp)
}

// #[get("/static")]
// pub async fn content() -> Result<NamedFile, std::io::Error> {
//   let path: PathBuf = req.match_info().query("filename").parse().unwrap();
//   Ok(NamedFile::open(path)?)
// }

#[get("/todos")]
pub async fn get_todolists() -> HttpResponse {
  let tls=vec![
    TodoList{
      id:1232342,
      title:String::from("This is my title"),
    },TodoList{
      id:333332,
      title:String::from("This title of second item"),
    },];

  let resp = ApiResponse2{
    status:200,
    status_text: String::from("OK"),
    payload: tls,
  };
  //return
  HttpResponse::Ok()
    .content_type("application/json")
    .body(serde_json::to_string_pretty(&resp).unwrap())
    // .body(format!("{:?}", serde_json::to_string_pretty(&resp).unwrap()))
  // resp.send_ok()
}


#[post("/todos")]
pub async fn add_todolist(lt:web::Json<BaseTodoList>) -> HttpResponse {
  let tl = TodoList{
    id: 11232334,
    title: String::from(&lt.title),
  };
  let resp = ApiResponse2{
    status:200,
    status_text: String::from("OK"),
    payload: tl,
  };
  //return
  HttpResponse::Ok()
    .content_type("application/json")
    .body(serde_json::to_string_pretty(&resp).unwrap())
}

#[put("/todos")]
pub async fn update_todolist(lt:web::Json<TodoList>) -> HttpResponse {
  let tl = TodoList{
    id: lt.id,
    title: lt.title.clone()
  };
  let resp = ApiResponse2{
    status:200,
    status_text: String::from("OK"),
    payload: tl,
  };
  //return
  HttpResponse::Ok()
    .content_type("application/json")
    .body(serde_json::to_string_pretty(&resp).unwrap())
}

#[get("/todos/{lid}/items")]
pub async fn get_todoitem(lid: web::Path<u64>) -> HttpResponse {
  println!("lid:{:?}", lid);
  let ti = TodoItem{
    id:1232342,
    list_id: *lid,
    title:String::from("This is my title"),
    checked: false,
  };

  let resp = ApiResponse2{
    status:200,
    status_text: String::from("OK"),
    payload: ti,
  };
  //return
  HttpResponse::Ok()
    .content_type("application/json")
    .body(serde_json::to_string_pretty(&resp).unwrap())
}

#[post("/todos/{lid}/items")]
pub async fn add_todoitem(lid: web::Path<u64>, todo:web::Json<TodoItemData>) -> HttpResponse {
  // println!("lid:{:?}", lid);
  let ti = TodoItem{
    id:1232342,
    list_id: *lid,
    title: todo.title.clone(),
    checked: todo.checked.clone(),
  };

  let resp = ApiResponse2{
    status:200,
    status_text: String::from("OK"),
    payload: ti,
  };
  //return
  HttpResponse::Ok()
    .content_type("application/json")
    .body(serde_json::to_string_pretty(&resp).unwrap())
}