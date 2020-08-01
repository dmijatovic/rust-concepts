use actix_web::{web, get, post, Responder, HttpResponse};

// mod response;

use crate::response::{ApiResponse, ApiResponse2};
use crate::models::{TodoList, TodoItem, BaseTodoList};

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


#[get("/todos")]
pub async fn get_todos() -> HttpResponse {
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

#[get("/todolist")]
pub async fn get_todolist() -> HttpResponse {
  let tl = TodoList{
    id:1232342,
    title:String::from("This is my title"),
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
    // .body(format!("{:?}", serde_json::to_string_pretty(&resp).unwrap()))
  // resp.send_ok()
}

#[get("/todoitem")]
pub async fn get_todoitem() -> HttpResponse {
  let ti = TodoItem{
    id:1232342,
    list_id: 1,
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