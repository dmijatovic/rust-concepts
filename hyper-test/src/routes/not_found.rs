use hyper::{Body, Response};

pub fn get_not_found() -> Response<Body> {
  Response::builder()
    .status(404)
    .header("Content-Type", "text/html")
    .body("<h1>404 - Not found</h1>".into())
    .unwrap()
}
