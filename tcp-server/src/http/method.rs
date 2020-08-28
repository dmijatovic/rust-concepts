
use std::str::FromStr;
use super::error::ParseError;


#[derive(Debug)]
pub enum Method{
  GET,
  POST,
  PUT,
  DELETE,
  OPTIONS
}

//Cannot be used with referenced strings with a lifetime
impl FromStr for Method{
  //required error type
  type Err = ParseError;
  //from_str implementation
  fn from_str(str_method:&str)->Result<Self, Self::Err>{
    match str_method.trim(){
      "GET"=>Ok(Method::GET),
      "POST"=>Ok(Method::POST),
      "PUT"=>Ok(Method::PUT),
      "DELETE"=>Ok(Method::DELETE),
      "OPTIONS"=>Ok(Method::OPTIONS),
      _=>Err(ParseError::InvalidMethod),
    }
  }
}