use hyper::{Body, Method, Request, Response};
use std::convert::Infallible;

use serde::{Deserialize, Serialize};

mod home;
mod json;
mod not_found;

use home::get_home;
use json::get_json;
use not_found::get_not_found;

pub async fn main(req: Request<Body>) -> Result<Response<Body>, Infallible> {
  match (req.method(), req.uri().path()) {
    (&Method::GET, "/") => Ok(get_home()),
    (&Method::GET, "/json") => Ok(get_json()),
    // Return the 404 Not Found for other routes.
    _ => Ok(get_not_found()),
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonResponse {
  status: u16,
  status_text: String,
  payload: String,
}

impl JsonResponse {
  pub fn new(status: Option<u16>, status_text: Option<&str>, payload: Option<&str>) -> Self {
    match (status, status_text, payload) {
      (None, None, None) => Self {
        status: 500,
        status_text: String::from("Server error"),
        payload: String::from("{}"),
      },
      (None, None, Some(data)) => Self {
        status: 200,
        status_text: String::from("OK"),
        payload: String::from(data),
      },
      (Some(s), Some(t), Some(d)) => Self {
        status: s,
        status_text: String::from(t),
        payload: String::from(d),
      },
      _ => Self {
        status: 501,
        status_text: String::from("Server error: unexpected JsonResponse match"),
        payload: String::from("{}"),
      },
    }
  }
}
