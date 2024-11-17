use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use eframe::egui;
use eframe::App;
use eframe::CreationContext;
use event::Event;
use event::EventManager;
use message::MotorMsg;
use message::MsgBuilder;
use router::Route;
use router::Router;
use ui::render_navbar;
use ui::{render_home, render_settings, render_profile, render_not_found};
use crossbeam::channel;
use std::error::Error;
use std::fmt;

mod router;
mod event;
mod ui;
mod bus;
mod widgets;

use bus::get_bus;


struct ClientApp{
    router:Router,
    sender:Sender<Box<dyn Event>>,
    receiver:Receiver<Box<dyn Event>>,
    events:EventManager
}

impl App for ClientApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            render_navbar(self,ui, ctx, frame);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            // Render the current route's content
            match self.router.current() {
                Route::Home => render_home(self,ctx, frame),
                Route::Settings(sub_route) => render_settings(self, ctx, frame),
                Route::Profile => render_profile(self,ctx, frame),
                // Add more routes as needed
                _ => render_not_found(self,ctx, frame), // Default case for unknown routes
            }
        });
    }
}



impl ClientApp{
    pub fn new(router:Router, sender: Sender<Box<dyn Event>>,receiver: Receiver<Box<dyn Event>>) -> Self{
        let events = EventManager::new();
        Self {
            router,
            sender,
            receiver,
            events
        }
    }
}

#[derive(Debug)]
struct AppError {
    message: String,
}

impl AppError {
    fn new(message: &str) -> Self {
        AppError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AppError {}
unsafe impl Send for AppError {}
unsafe impl Sync for AppError {}


fn app_setup(cc: &CreationContext){
    let mut fonts = egui::FontDefinitions::default();
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
    cc.egui_ctx.set_fonts(fonts);
}


fn main() {
    let native_options = eframe::NativeOptions::default();

    let (ui_sender, server_receiver) = channel::unbounded();
    let (server_sender, ui_receiver) = channel::unbounded();

    let handle = std::thread::spawn(move||{
        tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let mut stream = None;
            if let Ok(appstream) = tokio::net::TcpStream::connect("127.0.0.1:8080").await{
                stream = Some(appstream);
            }
            let mut events = EventManager::new();
            let buf = [0; 1024];
            loop {
                if let Ok(event) = server_receiver.try_recv(){
                    println!("event is {:?}",event);
                    events.add_event(event);
                }
                for event in events.iter(){
                    if let event::EventType::MotorEvent = event.get_type() {
                        let motor_msg = MotorMsg::default();
                        let msg = MsgBuilder::new().msg_type(message::MessageType::Move).data(Box::new(motor_msg)).build().unwrap();            
                        get_bus().lock().unwrap().send_message(&msg);
                    }
                }
            }
        });
    });

    eframe::run_native(
        "Task Manager",
        native_options,
        Box::new(|cc| {
                app_setup(cc);
                Ok(Box::new(ClientApp::new(Router::new(), ui_sender, ui_receiver)))
            }
        ),
    )
    .unwrap();
}
