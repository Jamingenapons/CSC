use message::Msg;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc::{unbounded_channel, UnboundedReceiver},
    time::{self, Duration},
    task::spawn,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::SubSystem;

pub struct TcpSystem {
    receiver: UnboundedReceiver<Msg>,
    msgs: Vec<Msg>,
    stream: Option<TcpStream>,
    addr: SocketAddr,
    reconnect_interval: Duration,
    heartbeat_interval: Duration,
}

impl TcpSystem {
    pub fn new(receiver: UnboundedReceiver<Msg>, addr: SocketAddr) -> Self {
        TcpSystem {
            receiver,
            msgs: Vec::new(),
            stream: None,
            addr,
            reconnect_interval: Duration::from_secs(5),
            heartbeat_interval: Duration::from_secs(10),
        }
    }

    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(self.addr).await?;
        self.stream = Some(stream);
        Ok(())
    }

    async fn disconnect(&mut self) {
        if let Some(stream) = self.stream.take() {
            drop(stream);
        }
    }

    async fn send_heartbeat(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mut stream) = self.stream {
            // let heartbeat_msg = Msg::new(message::MessageType::Heartbeat, "".to_string());
            // let msg = serde_json::to_vec(&heartbeat_msg).unwrap();
            // stream.write_all(&msg).await?;
        }
        Ok(())
    }

    async fn reconnect(&mut self) {
        loop {
            if let Err(e) = self.connect().await {
                eprintln!("Failed to reconnect: {}", e);
                time::sleep(self.reconnect_interval).await;
            } else {
                break;
            }
        }
    }

    async fn process_messages(&mut self) {
        let mut buffer = [0; 1024];
        loop {
            tokio::select! {
                Some(msg) = self.receiver.recv() => {
                    self.msgs.push(msg);
                }
                result = self.stream.as_mut().unwrap().read(&mut buffer) => {
                    match result {
                        Ok(n) if n > 0 => {
                            let received_msg: Msg = serde_json::from_slice(&buffer[..n]).unwrap();
                            // self.exec(&mut received_msg);
                        }
                        _ => {
                            // Handle disconnection
                            self.disconnect().await;
                            self.reconnect().await;
                            break;
                        }
                    }
                }
            }
            let msg_list = &self.msgs;
            if let Some(ref mut stream) = self.stream {
                for msg in msg_list.iter() {
                    let msg = serde_json::to_vec(msg).unwrap();
                    stream.write_all(&msg).await.unwrap();
                }
                self.msgs.clear();
            }    
        }
    }
}

impl SubSystem for TcpSystem {
    type Msg = Msg;
    fn exec(&mut self, msg: &mut Msg) {
        match msg.get_msg_type() {
            message::MessageType::None => todo!(),
            message::MessageType::Quit => todo!(),
            message::MessageType::Move => todo!(),
            message::MessageType::Join => todo!(),
            // message::MessageType::Heartbeat => {
            //     // Handle heartbeat message
            // }
        }
    }

    fn rollup(&mut self) {
        self.msgs.clear();
    }
}

impl TcpSystem {
    pub async fn run(&mut self) {
        self.connect().await.unwrap();

        let mut heartbeat_interval = time::interval(self.heartbeat_interval);

        // Spawn the process_messages task

        loop {
            tokio::select! {
                _ = heartbeat_interval.tick() => {
                    if let Err(e) = self.send_heartbeat().await {
                        eprintln!("Heartbeat failed: {}", e);
                        self.disconnect().await;
                        self.reconnect().await;
                    }
                }
                _ = self.process_messages() => {
                    // Handle disconnection
                    self.disconnect().await;
                    self.reconnect().await;
                }
            }
        }
    }
}