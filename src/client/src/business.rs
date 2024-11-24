// src/business_logic.rs
use crate::event::{self, Event, EventManager};
use message::{MessageType, MotorMsg, MsgBuilder};
use tokio::{
    io::AsyncWriteExt,
    net::TcpStream,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};

pub struct BusinessLogic {
    server_receiver: UnboundedReceiver<Box<dyn Event>>,
    stream: Option<TcpStream>,
}

impl BusinessLogic {
    pub async fn new(server_receiver: UnboundedReceiver<Box<dyn Event>>) -> Self {
        let mut stream = None;
        if let Ok(appstream) = tokio::net::TcpStream::connect("127.0.0.1:8080").await {
            stream = Some(appstream);
        }
        Self {
            server_receiver,
            stream: stream,
        }
    }

    pub async fn run(&mut self) {
        let mut events = EventManager::new();
        let buf = [0; 1024];
        loop {
            if let Ok(event) = self.server_receiver.try_recv() {
                println!("event is {:?}", event);
                events.add_event(event);
            }
            if let Some(event) = events.pop() {
                if let event::EventType::MotorEvent = event.get_type() {
                    let motor_msg = MotorMsg::default();
                    let msg = MsgBuilder::new()
                        .msg_type(MessageType::Move)
                        .data(Box::new(motor_msg))
                        .build()
                        .unwrap();
                    if let Some(stream) = &mut self.stream {
                        let msg = serde_json::to_string(&msg).unwrap();
                        println!("msg send is {}", msg);
                        if let Err(e) = stream.write_all(msg.as_bytes()).await {
                            println!("Failed to write to socket; err = {:?}", e);
                            return;
                        }
                    }
                }
            }
        }
    }
}
