
use egui::{self};
use egui_dock::{Tree, TabViewer};


pub struct MyDock {
    tree: Tree<String>,
}

impl MyDock {
    pub fn new() -> Self {
        let mut tree = Tree::new(vec!["Tab 1".to_string()]);
        let root = tree.root_node(); // 假设 Tree 提供了 root_node() 方法
        tree.split_left(0.into(), 0.3, vec!["Tab 2".to_string()]);
        tree.split_below(1.into(), 0.7, vec!["Tab 3".to_string()]);

        Self { tree }
    }
}
impl TabViewer for MyDock {
    type Tab = String;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.label(format!("Content of {}", tab));
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }
}