use crate::gui::QuickLaunchApp;
use eframe::Frame;
use egui::{Context};

impl eframe::App for QuickLaunchApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.top_panel(ctx);
        self.action_panel(ctx);

    }
    
    
}
