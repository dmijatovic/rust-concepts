// read file
use std::fs;

use super::request::Request;
use super::response::Response;
use super::error::ParseError;
use super::response::status::StatusCode;
use super::method::Method;

pub trait Handler{
  fn handle_request(&mut self, request: &Request)->Response;
  //default implementation for bad request response
  fn handle_bad_request(&mut self, e: &ParseError)->Response{
    default_bad_response(e)
  }
}

pub struct WebHandler{
  pub public_path:String,
}

impl WebHandler{
  pub fn new(public_path:String)->Self{
    Self{public_path}
  }
  fn read_file(&self, file_path:&str)->Option<String>{
    //define path
    let path = format!("{}/{}",self.public_path,file_path);
    //ensure the path is allowed
    //canonicalize will return abolute path
    match fs::canonicalize(path){
      Ok(path)=>{
        if path.starts_with(&self.public_path){
          //converts result to option
          fs::read_to_string(path).ok()
        }else{
          // path is not allowed
          println!("Directory traversal attack atempt: {:?}", path);
          None
        }
      }
      Err(_) => None
    }
  }
  fn get_static_response(&self, request: &Request)->Response{
    // println!("read...path...{}", req);
    let log:String = format!("{} {}",
      request.method().to_str(),
      request.path()
    );
    match request.path(){
      "/"=>Response::new(
          StatusCode::OK,
          log,
          None,
          self.read_file("index.html")
        ),
      "/hello"=>Response::new(
        StatusCode::OK,
        log,
        None,
        self.read_file("hello.html")
      ),
      path => match self.read_file(path){
        Some(content)=>{
          Response::new(
            StatusCode::OK,
            log,
            None,
            Some(content)
          )
        },
        None=>default_bad_response(&ParseError::InvalidRequest)
      }
    }
  }
}

fn default_bad_response(e: &ParseError)->Response{
  let log = format!("400 Bad Request: {:?}", e);
  Response::new(
    StatusCode::BadRequest,
    log,
    None,
    None
  )
}

impl Handler for WebHandler{
  fn handle_request(&mut self, request: &Request)-> Response{
    // match response
    match request.method(){
      Method::GET=>self.get_static_response(request),
      _=> default_bad_response(&ParseError::InvalidMethod)
    }
  }
}