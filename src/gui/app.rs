use crate::gui::QuickLaunchApp;
use eframe::Frame;
use egui::{Context, ScrollArea, Style, menu};

impl eframe::App for QuickLaunchApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.top_bar(ctx);
    }
}
