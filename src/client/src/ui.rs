mod home;
mod profile;
mod setting;
mod error;

use eframe::egui;

pub use self::{home::render_home, profile::render_profile, setting::render_settings, error::render_not_found};


use egui::Ui;

use crate::{router::Route, ClientApp};

pub fn render_navbar(app: &mut ClientApp, ui: &mut Ui ,ctx: &egui::Context, frame: &mut eframe::Frame) {
    ui.horizontal(|ui| {
        if ui.button("Home").clicked() {
            app.router.set_current(Route::Home);
        }
        if ui.button("Settings").clicked() {
            app.router.set_current(Route::Settings(crate::router::SettingsRoute::General));
        }
        if ui.button("Profile").clicked() {
            app.router.set_current(Route::Profile);
        }
    });
}



