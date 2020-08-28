// trait we use to convert stream of bytes
// into request
use std::convert::TryFrom;
use std::str;

use super::method::Method;
use super::error::ParseError;

#[derive(Debug)]
pub struct Request<'buf>{
  path: &'buf str,
  query_string: Option<&'buf str>,
  method: Method,
  body: &'buf str,
}

// implementing TryFrom trait. It needs
// specific type: in this case we convert from
// &[u8] into a request struct that returns itself or custom self:error
// So we also need to define error struct/object/string?
// NOTE! compiler will implement TryInto trait from this code automatically!
// FOR lifetime we add buf lifetime
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf>{
  // define custom error type
  type Error = ParseError;
  // conversion function from bytes into request struct
  fn try_from(buffer: &'buf [u8]) -> Result<Self, Self::Error>{
    // macro for flagging not done yet
    // unimplemented!();
    // SHORT WAY using ? needs to implement UTF8 error
    let req = str::from_utf8(buffer)?;

    // return new request
    Request::new(req)
  }
}

// split string on char. It return first value and the rest of it
fn split_by_char<'buf>(text:&'buf str, delimiter:char) -> Option<(&'buf str, &'buf str)>{
  for (i, chr) in text.chars().enumerate(){
    if chr == delimiter{
      return Some((&text[..i],&text[i+delimiter.len_utf8()..]));
    }
  }
  // if we are here then none
  None
}

impl<'buf> Request<'buf>{
  fn new(req:&'buf str)->Result<Self, ParseError>{
    let space = " ".chars().next().unwrap();
    let new_line = "\r".chars().next().unwrap();
    let qm = "?".chars().next().unwrap();

    // extract first line from request string
    // if error return it (note ? at the end)
    // example fist_line: GET /home HTTP/1.1
    let (first_line, body) = split_by_char(req, new_line)
      .ok_or(ParseError::InvalidRequest)?;
    // extract method from first_line
    let (str_method, rest) = split_by_char(first_line, space)
      .ok_or(ParseError::InvalidRequest)?;
    // extract path and protocol
    let (mut path, protocol) = split_by_char(rest, space)
      .ok_or(ParseError::InvalidRequest)?;
    // check protocol
    if protocol != "HTTP/1.1"{
      return Err(ParseError::InvalidProtocol)
    }
    // parse method to enum from string
    // this is possible because we implemented FromStr trait
    // see method.rs line 15
    let method:Method = str_method.parse()?;
    // default query string is none
    let mut query_string = None;
    // check if ? split returns Some results
    if let Some(res) = split_by_char(path, qm){
      path = res.0;
      query_string = Some(res.1);
    };
    //return Request object
    Ok(Self{
      path,
      query_string,
      method,
      body
    })
  }
}

