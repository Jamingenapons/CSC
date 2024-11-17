use egui::Widget;

use crate::{event::{Event, EventType, MotorEvent}, router::Route, ClientApp};

use crate::widgets::CustomButton;

#[derive(Debug)]
struct HomeEvent{
    id:i32
}
impl HomeEvent {
    fn new(id: i32) -> Self {
        Self { id }
    }
}

impl Event for HomeEvent{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn get_type(&self) -> EventType {
        EventType::HomeEvent
    }
}

pub fn render_home(app: &mut ClientApp, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Home Page");
        
        // 欢迎信息
        ui.label(format!("Welcome back"));
        
        // 最近的活动
        ui.separator();
        ui.label("Recent Activities:");
        // egui::ScrollArea::vertical().show(ui, |ui| {
        //     for activity in &app.recent_activities {
        //         ui.label(activity);
        //     }
        // });
        if ui.button("event").clicked(){
            let home_event = Box::new(HomeEvent::new(123));
            app.sender.send(home_event).unwrap();
        }

        ui.add(CustomButton::new("custom"));

        if CustomButton::new("response").ui(ui).clicked(){
            println!("custom click");
        }

        // 快速操作
        ui.separator();
        ui.label("Quick Actions:");
        egui::Grid::new("quick_actions").show(ui, |ui| {
            if ui.button("Create New").clicked() {
                // 处理创建新项目的逻辑
            }
            ui.end_row();

            if ui.button("Edit Existing").clicked() {
                // 处理编辑现有项目的逻辑
            }
            ui.end_row();

            if ui.button("View Details").clicked() {
                // 处理查看详情的逻辑
            }
            ui.end_row();
        });

        if ui.button("motor").clicked(){
            app.sender.send(Box::new(MotorEvent::default())).unwrap();
        }

        // 快捷链接
        ui.separator();
        if ui.button("Go to Settings").clicked() {
            app.router.set_current(Route::Settings(crate::router::SettingsRoute::General));
        }
        if ui.button("Go to Profile").clicked() {
            app.router.set_current(Route::Profile);
        }

        ui.label(egui::RichText::new(format!("FILE_CODE {}", egui_phosphor::regular::FILE_CODE)).size(32.0));
    });
}
