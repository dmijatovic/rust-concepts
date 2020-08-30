
// for implementing display trait
// Note! changed to send method on response
// use std::fmt::{Display,Formatter, Result as FmtResult};
//for writing response
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};

pub mod status;
// mod header;

use status::StatusCode;

#[derive(Debug)]
pub struct Response{
  status: StatusCode,
  header: Option<String>,
  body: Option<String>
}

impl Response{
  pub fn new(status: StatusCode, header: Option<String>,
    body:Option<String>) -> Self{
    Response{
      status,
      header,
      body
    }
  }
  //send response
  pub fn send(&self, stream: &mut TcpStream)->IoResult<()>{
     // get body content
    let body = match &self.body{
      Some(b)=>b,
      None=>""
    };
    println!("HTTP/1.1 {} {}\r\n\r\n{}",
      self.status,
      self.status.reason_phrase(),
      body
    );
    //write response
    write!(stream,
      "HTTP/1.1 {} {}\r\n\r\n{}",
      self.status,
      self.status.reason_phrase(),
      body
    )
  }
}

// impl Display for Response{
//   fn fmt(&self,f: &mut Formatter)-> FmtResult{
//     // get body content
//     let body = match &self.body{
//       Some(b)=>b,
//       None=>""
//     };
//     //write response
//     write!(f,"HTTP/1.1 {} {}\r\n\r\n{}",
//       self.status,
//       self.status.reason_phrase(),
//       body
//     )
//   }
// }