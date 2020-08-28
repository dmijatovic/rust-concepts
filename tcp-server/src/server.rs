//pull read trait
use std::io::Read;
use std::net::TcpListener;
// import train in order to be able to use it
use std::convert::TryFrom;

use crate::http::Request;

#[derive(Debug)]
pub struct Server{
  addr: String,
}

pub fn read(stream: &mut std::net::TcpStream){
  //create array buffer of 512 filled with 0
  let mut buffer = [0; 512];
  match stream.read(&mut buffer){
    Ok(_)=>{
      // println!("Received request: {}",String::from_utf8_lossy(&buffer));
      //convert incoming stream into request object
      // we need to convert out fixed buffer into slice reference
      // hence &buffer[..]
      let req = Request::try_from(&buffer[..]).unwrap();
      println!("request: {:?}", req);

    },
    Err(e)=>{println!("Failed to read: {:?}", e)}
  }
}

impl Server{
  pub fn new(addr:String) -> Self{
    Self{addr}
  }
  pub fn listen(&self){
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
        Ok((mut stream, addr))=>{
          println!("new client {:?}", addr);
          //read incoming message from stream
          read(&mut stream)
        },
        Err(e)=>{
          dbg!("Failed to accept {:?}",e);
        }
      }
    }
  }
}