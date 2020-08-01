use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use futures::future::{ready, Ready};


#[derive(Serialize, Debug)]
pub struct ApiResponse{
  pub status: u16,
  pub status_text: String,
  pub payload: String,
}

impl ApiResponse{
  #[allow(dead_code)]
  pub fn new(status:u16, status_text:&str, payload:&str)->Self{
    ApiResponse {
      status,
      status_text:String::from(status_text),
      payload:String::from(payload),
    }
  }
  #[allow(dead_code)]
  pub fn send_ok(self)-> HttpResponse{
    let body = serde_json::to_string(&self).unwrap();

    // Create response and set content type
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
  }
}


impl Responder for ApiResponse{
  type Error = Error;
  type Future = Ready<Result<HttpResponse, Error>>;

  fn respond_to(self, _req: &HttpRequest) -> Self::Future {
    let body = serde_json::to_string(&self).unwrap();

    // Create response and set content type
    ready(Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body)))
  }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse2<T>{
  pub status: u16,
  pub status_text: String,
  pub payload: T,
}

// impl <T> ApiResponse2<T>{
//   pub fn send_ok(self)-> HttpResponse{
//     let body = serde_json::to_string_pretty(&self).unwrap();

//     // Create response and set content type
//     HttpResponse::Ok()
//         .content_type("application/json")
//         .body(body)
//   }
// }


// #[derive(Serialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct ApiResponse3{
//   pub status: u16,
//   pub status_text: String,
//   pub payload: TodoList,
// }

// impl ApiResponse3{
//   pub fn send_ok(self)-> HttpResponse{
//     let body = serde_json::to_string_pretty(&self).unwrap();

//     // Create response and set content type
//     HttpResponse::Ok()
//         .content_type("application/json")
//         .body(body)
//   }
// }