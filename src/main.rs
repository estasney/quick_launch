mod config;
mod gui;
mod utils;

use crate::config::app_config::QuickLaunchAppSettings;
use serde::{Deserialize, Serialize};

fn run_gui() -> eframe::Result {
    let mut cfg: QuickLaunchAppSettings =
        QuickLaunchAppSettings::load().expect("Failed to load config file");
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([370.0, 600.0])
            .with_min_inner_size([250.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Quick Launch",
        native_options,
        Box::new(|cc| Ok(Box::new(gui::QuickLaunchApp::new(cc)))),
    )
}

fn main() {
    run_gui().expect("Failed to start gui");
}
