
mod server;
mod http;

use server::Server;

fn main() {
    println!("Hello, world!");
    //create server
    let srv = Server::new("localhost:8080".to_string());
    //listen
    srv.listen();
}
