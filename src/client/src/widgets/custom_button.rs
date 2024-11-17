
use egui::{FontId, Stroke};

pub struct CustomButton {
    text: String,
    is_clicked: bool,
}


impl CustomButton {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            is_clicked: false,
        }
    }

    pub fn is_clicked(&self) -> bool {
        self.is_clicked
    }

    pub fn reset_clicked(&mut self) {
        self.is_clicked = false;
    }
}

impl egui::Widget for CustomButton {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        // 定义按钮的内边距
        let button_padding = egui::Vec2::new(10.0, 5.0);
        
        // 计算按钮的大小，包括内边距
        let button_size = egui::Vec2::new(
            ui.spacing().icon_width, // 按钮的宽度
            ui.spacing().interact_size.y, // 按钮的高度
        ) + 2.0 * button_padding; // 加上内边距

        // 分配按钮的精确大小，并获取响应区域和响应对象
        let (rect, response) = ui.allocate_exact_size(button_size, egui::Sense::click());

        // 如果按钮被点击，设置is_clicked为true
        if response.clicked() {
            self.is_clicked = true;
        }

        // 获取按钮的视觉样式
        let visuals = ui.style().interact(&response);
        
        // 绘制按钮的背景填充
        ui.painter().rect_filled(rect, 4.0, visuals.bg_fill);

        let stroke = Stroke {
            width: 2.0,
            color: visuals.bg_stroke.color,
        };
        
        // 绘制按钮的边框
        ui.painter().rect_stroke(rect, 4.0, stroke);

        let text_size = ui.fonts(|font| font.font_image_size());

        // 计算文本的位置，使其居中
        let text_pos = rect.center() - egui::Vec2::new(
            0.0,
            0.0,
        );
        
        let font_id = FontId::default();
        // 绘制按钮的文本
        ui.painter().text(
            text_pos, // 文本的位置
            egui::Align2::CENTER_CENTER, // 文本的对齐方式
            self.text, // 文本内容
            font_id, // 文本样式
            visuals.text_color(), // 文本颜色
        );

        // 返回响应对象
        response
    }
}
