use hyper::{Body, Response};
// use std::convert::Infallible;
// std::fs::read_to_string(path: P);
use std::io::Error;

fn get_content() -> Result<String, Error> {
  std::fs::read_to_string("src/views/home.html")
}

pub fn get_home() -> Response<Body> {
  // get content of html file
  let content = get_content().unwrap();
  // build response
  Response::builder()
    .status(200)
    .header("Content_Type", "text/html")
    .body(content.into())
    .unwrap()
}
