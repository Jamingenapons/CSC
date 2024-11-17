use crate::{router::Route, ClientApp};

pub fn render_not_found(app: &mut ClientApp, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("404 - Page Not Found");
        
        // 错误信息
        ui.label("The page you are looking for does not exist.");
        
        // 返回主页的按钮
        if ui.button("Go to Home").clicked() {
            app.router.set_current(Route::Home);
        }
    });
}
