// trait we use to convert stream of bytes
// into request
use std::convert::TryFrom;
use std::str;

use super::method::Method;
use super::error::ParseError;

#[derive(Debug)]
pub struct Request{
  path: String,
  query_string: Option<String>,
  method: Method
}

// implementing TryFrom trait. It needs
// specific type: in this case we convert from
// &[u8] into a request struct that returns itself or custom self:error
// So we also need to define error struct/object/string?
// NOTE! compiler will implement TryInto trait from this code automatically!
impl TryFrom<&[u8]> for Request{
  // define custom error type
  type Error = ParseError;
  // conversion function from bytes into request struct
  fn try_from(buffer: &[u8]) -> Result<Self, Self::Error>{
    // macro for flagging not done yet
    // unimplemented!();
    // LONG MATCH way
    // match str::from_utf8(buffer){
    //   Ok(req)=>{
    //     //more code here
    //   },
    //   Err(_)=>{Err(ParseError::InvalidEncoding)}
    // }
    // SHORT WAY using ? needs to implement UTF8 error
    let req = str::from_utf8(buffer)?;

    // return new request
    Ok(Request::new(req))
  }
}

impl Request{
 fn new(req:&str)->Self{
  Self{
    path:"/".to_string(),
    query_string:None,
    method: Method::GET
  }
 }
}

