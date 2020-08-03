#![feature(decl_macro)]

use rocket::{self, get, routes};

#[get("/")]
fn index() -> String {
  format!("This is the thing to return from home")
}

#[get("/hello")]
fn hello() -> String {
  "Hello, world!".to_string()
}

fn main() {
  rocket::ignite()
    .mount("/",routes![index, hello])
    .launch();
}
