#![warn(rust_2018_idioms)]

use futures::{future::try_join, FutureExt, StreamExt};
use std::{env, error::Error};
use tokio::{
    io::{AsyncReadExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listen_addr = env::args().nth(1).unwrap_or("127.0.0.1:8081".to_string());
    let server_addr = env::args().nth(2).unwrap_or("127.0.0.1:8080".to_string());

    println!("Listening on : {}", listen_addr);
    println!("Proxying to: {}", server_addr);

    let mut incomming = TcpListener::bind(listen_addr).await?.incoming();

    while let Some(Ok(inbound)) = incomming.next().await {
        println!("Got connection on");
        let transfer = transfer(inbound, server_addr.clone()).map(|r| {
            if let Err(e) = r {
                println!("Failed to tranfer; err = {}", e);
            }
        });

        tokio::spawn(transfer);
    }
    Ok(())
}

async fn transfer(mut inbound: TcpStream, proxy_addr: String) -> Result<(), Box<dyn Error>> {
    println!("Connecting to {}", proxy_addr);
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = ri.copy(&mut wo);
    let server_to_client = ro.copy(&mut wi);

    try_join(client_to_server, server_to_client).await?;

    Ok(())
}


