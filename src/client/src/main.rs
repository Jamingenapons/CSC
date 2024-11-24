use business::BusinessLogic;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use eframe::egui;
use eframe::App;
use eframe::CreationContext;
use event::Event;
use event::EventManager;
use router::Route;
use router::Router;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use ui::render_navbar;
use ui::{render_home, render_settings, render_profile, render_not_found};
use tokio::sync::mpsc::unbounded_channel;
use std::error::Error;
use std::fmt;

mod router;
mod event;
mod ui;
mod business;
mod widgets;

struct ClientApp {
    router: Router,
    sender: UnboundedSender<Box<dyn Event>>,
    receiver: UnboundedReceiver<Box<dyn Event>>,
    events: EventManager,
}

impl App for ClientApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            render_navbar(self, ui, ctx, frame);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            // Render the current route's content
            match self.router.current() {
                Route::Home => render_home(self, ctx, frame),
                Route::Settings(sub_route) => render_settings(self, ctx, frame),
                Route::Profile => render_profile(self, ctx, frame),
                // Add more routes as needed
                _ => render_not_found(self, ctx, frame), // Default case for unknown routes
            }
        });
    }
}

impl ClientApp {
    pub fn new(router: Router, sender:UnboundedSender<Box<dyn Event>> , receiver:UnboundedReceiver<Box<dyn Event>> ) -> Self {
        let events = EventManager::new();
        Self {
            router,
            sender,
            receiver,
            events,
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

fn app_setup(cc: &CreationContext) {
    let mut fonts = egui::FontDefinitions::default();
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
    cc.egui_ctx.set_fonts(fonts);
}

#[tokio::main]
async fn main() {
    let native_options = eframe::NativeOptions::default();

    let (ui_sender, server_receiver) = unbounded_channel::<Box<dyn Event>>();
    let (server_sender, ui_receiver) = unbounded_channel::<Box<dyn Event>>();

    let handle = tokio::spawn(async move {
        let mut business_logic = BusinessLogic::new(server_receiver).await;
        business_logic.run().await;
    });

    eframe::run_native(
        "Task Manager",
        native_options,
        Box::new(|cc| {
            app_setup(cc);
            Ok(Box::new(ClientApp::new(Router::new(), ui_sender, ui_receiver)))
        }),
    )
    .expect("Failed to run native application");
}