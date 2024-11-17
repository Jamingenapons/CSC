use lazy_static::lazy_static;
use message::Msg;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex, Once};
use std::thread;
use std::time::Duration;

lazy_static! {
    static ref BUS: Arc<Mutex<Bus>> = Arc::new(Mutex::new(Bus::new()));
    static ref INIT: Once = Once::new();
}

pub struct Bus {
    stream: Option<TcpStream>,
}

impl Bus {
    fn new() -> Self {
        Bus { stream: None }
    }

    fn start(&mut self) {
        // 尝试连接到本地 socket
        let stream = match TcpStream::connect("127.0.0.1:8080") {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Failed to connect to socket: {}", e);
                // 启动一个定时器来定期检测连接
                self.start_connection_checker();
                return;
            }
        };
        self.stream = Some(stream);
        println!("Connected to socket");
        // 启动一个独立的线程来处理消息
        let bus_arc = Arc::clone(&BUS);
        thread::spawn(move || {
            loop {
                let mut bus = bus_arc.lock().unwrap();
                if let Some(ref mut stream) = bus.stream {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(size) => {
                            let received_message: Msg =
                                serde_json::from_slice(&buffer[..size]).unwrap();
                            println!("Received message: {:?}", received_message);
                            // 处理接收到的消息
                        }
                        Err(e) => {
                            eprintln!("Failed to read from socket: {}", e);
                            break;
                        }
                    }
                }
            }
        });
    }

    fn start_connection_checker(&self) {
        let bus_arc = Arc::clone(&BUS);
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(5)); // 每5秒检测一次连接
                let mut bus = bus_arc.lock().unwrap();
                if bus.stream.is_none() {
                    match TcpStream::connect("127.0.0.1:8080") {
                        Ok(stream) => {
                            bus.stream = Some(stream);
                            bus.start(); // 重新启动消息处理线程
                            break;
                        }
                        Err(e) => {
                            eprintln!("Failed to reconnect to socket: {}", e);
                        }
                    }
                }
            }
        });
    }

    pub fn send_message(&mut self, message: &Msg) {
        if let Some(ref mut stream) = self.stream {
            let bytes = serde_json::to_vec(message).unwrap();
            if let Err(e) = stream.write_all(&bytes) {
                eprintln!("Failed to send message: {}", e);
            }
        }
    }
}

pub fn get_bus() -> Arc<Mutex<Bus>> {
    INIT.call_once(|| {
        let mut bus = BUS.lock().unwrap();
        bus.start();
    });
    Arc::clone(&BUS)
}
