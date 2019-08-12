use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

pub fn hello_world() {
    let addr = "127.0.0.1:6142".parse().unwrap();
    let client = TcpStream::connect(&addr)
        .and_then(|stream| {
            println!("created stream");
            Ok(())
        })
        .map_err(|err| {
            println!("connection error = {:?}", err);
        });
    tokio::run(client);
}
