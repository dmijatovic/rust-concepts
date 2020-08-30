//pull read trait
// use std::io::{Read};
use std::net::{TcpListener};
// import train in order to be able to use it
// use std::convert::TryFrom;

use crate::http::{Request, Handler};


#[derive(Debug)]
pub struct Server{
  addr: String,
}

impl Server{
  pub fn new(addr:String) -> Self{
    Self{addr}
  }
  pub fn listen(&self, mut handler: impl Handler){
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
              // let log:String = format!("{} {}",
              //   req.method.to_str(),
              //   req.path
              // );
              // Response::new(
              //   StatusCode::OK,
              //   log,
              //   None,
              //   Some(format!("<h1>Request received</h1><p>{:?}</p>", req))
              // )
              handler.handle_request(&req)
            },
            Err(e)=>{
              //return error response
              handler.handle_bad_request(&e)
            }
          };
          // println!("response {:?}", &response);
          if let Err(e) = response.send(&mut stream){
            println!("Failed to respond {:?}",e);
          } else {
            println!("{}...{}", response.log,response.status.to_str());
            // println!("SEND response 200 OK");
          };
        },
        Err(e)=>{
          println!("Failed to accept {:?}",e);
        }
      }
    }
  }
}