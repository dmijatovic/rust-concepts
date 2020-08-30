// implement display trait
use std::fmt::{Display,Formatter,Result as FmtResult};

//implement copy,close to make Display possible
#[derive(Copy,Clone,Debug)]
pub enum StatusCode{
  OK = 200,
  BadRequest = 400,
  NotFound = 404,
}

impl StatusCode {
  pub fn reason_phrase(&self) -> &str{
    match self {
      Self::OK => "OK",
      Self::BadRequest => "Bad Request",
      Self::NotFound => "Not Found"
    }
  }
  pub fn to_str(&self) -> &str{
    match self {
      Self::OK => "200 OK",
      Self::BadRequest => "400 Bad Request",
      Self::NotFound => "404 Not Found"
    }
  }
}

impl Display for StatusCode {
  fn fmt(&self, f: &mut Formatter) -> FmtResult{
    write!(f,"{}", *self as u16)
  }
}

