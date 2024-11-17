use crate::ClientApp;

use egui_dock::TabViewer;
use egui_extras::{Column, TableBuilder};
use egui::{
    Align, Layout,
};

use crate::widgets::MyDock;


pub fn render_profile(app: &mut ClientApp, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Profile Page");
                
        // 用户名
        ui.label(format!("Username: {}", "name"));
        
        // 电子邮件
        ui.label(format!("Email: {}", "email"));
        
        // 编辑按钮
        if ui.button("Edit Profile").clicked() {
            // 编辑个人信息的逻辑
            // app.edit_profile();
        }

        ui.horizontal(|ui|{
            render_dashboard(app, ctx, frame);
            MyDock::new().ui(ui, &mut "mydock".to_string());
        })
    });
}

pub fn render_dashboard(app: &mut ClientApp, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Dashboard");

        // 创建一个表格来显示数据
        let data = vec![
            ("Total Orders", "1234"),
            ("Pending Orders", "234"),
            ("Completed Orders", "1000"),
            // 添加更多数据...
        ];

    let table = TableBuilder::new(ui)
        .striped(true)
        .cell_layout(Layout::left_to_right(Align::Center))
        .column(Column::auto()) // First column
        .column(Column::auto()) // Second column
        .min_scrolled_height(0.0);

    table
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.heading("Metric");
            });
            header.col(|ui| {
                ui.heading("Value");
            });
        })
        .body(|mut body| {
            for (metric, value) in data {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label(metric);
                    });
                    row.col(|ui| {
                        ui.label(value);
                    });
                });
            }
        });
    });
}
