use hyper::{Body, Response};

use super::JsonResponse;

pub fn get_json() -> Response<Body> {
  let json = JsonResponse::new(Some(200), Some("OK"), Some("This is my payload content"));

  // let resp = json.to_str();

  Response::builder()
    .status(200)
    .header("Content-Type", "application/json")
    .header("x-server", "hyper-rust")
    .body(serde_json::to_string(&json).unwrap().into())
    .unwrap()
}
