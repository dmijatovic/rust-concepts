//pull read trait
use std::io::{Read};
use std::net::{TcpListener, TcpStream};
// import train in order to be able to use it
use std::convert::TryFrom;

use crate::http::{Request, ParseError, Response, StatusCode};


#[derive(Debug)]
pub struct Server{
  addr: String,
}

impl Server{
  pub fn new(addr:String) -> Self{
    Self{addr}
  }
  pub fn listen(&self){
    let mut buffer=[0; 1024];
    // we pass
    let sock:TcpListener = match TcpListener::bind(&self.addr){
      Ok(sock)=>{
        println!("Listening on {}", self.addr);
        sock
      },
      Err(e)=>panic!(e),
    };
    // accept incoming requests
    // in infinite loop (will run untill program ends)
    loop {
      // accept connections
      match sock.accept(){
        Ok((mut stream, _addr))=>{
          let response = match Request::new_from_stream(
            &mut stream, &mut buffer){
            Ok(req)=>{
              // println!("read...path...{}", req);
              Response::new(
                StatusCode::OK,
                None,
                Some(format!("<p>{:?}</p>", req))
              )
            },
            Err(e)=>{
              dbg!(e);
              Response::new(
                StatusCode::BadRequest,
                None,
                None
              )
            }
          };
          // println!("response {:?}", &response);
          if let Err(e) = response.send(&mut stream){
            println!("Failed to respond {:?}",e);
          } else {
            println!("SEND response {:?}", &response);
          };
        },
        Err(e)=>{
          println!("Failed to accept {:?}",e);
        }
      }
    }
  }
}