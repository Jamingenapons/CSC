use crate::ClientApp;

pub fn render_settings(app: &mut ClientApp, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Settings Page");
        
        // 用户名设置
        ui.label("Username:");
        ui.text_edit_singleline(&mut "Username");
        
        // 密码设置
        ui.label("Password:");
        ui.text_edit_singleline(&mut "Password");
        
        // 通知设置
        ui.label("Enable Notifications:");
        ui.checkbox(&mut true, "");
        
        // 保存按钮
        if ui.button("Save Settings").clicked() {
            // 保存设置的逻辑
        }
    });
}