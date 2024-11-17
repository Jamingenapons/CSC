use std::fmt::Debug;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::task;

use message::Msg;

struct Server {
    listener: TcpListener,
}

impl Server {
    async fn new(addr: &str) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Server { listener })
    }

    async fn run(&self) {
        loop {
            match self.listener.accept().await {
                Ok((stream, _)) => {
                    println!("New connection established.");
                    tokio::spawn(Self::handle_connection(stream));
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    async fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        loop {
            match stream.read(&mut buffer).await {
                Ok(n) => {
                    if n == 0 {
                        println!("Connection closed by client.");
                        break;
                    }

                    let data = &buffer[..n];
                    let msg: Msg = match serde_json::from_slice(data) {
                        Ok(msg) => msg,
                        Err(e) => {
                            eprintln!("Failed to deserialize message: {}", e);
                            continue;
                        }
                    };
                    println!("Received: {:?}", msg);

                    // Echo the message back to the client
                    if let Err(e) = stream.write_all(data).await {
                        eprintln!("Failed to write to stream: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from stream: {}", e);
                    break;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");
    server.run().await;
    Ok(())
}