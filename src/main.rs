mod gui;
mod utils;

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use utils::arg_parser::Args;

fn run_gui(script_dir: PathBuf) -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([660.0, 800.0]).with_min_inner_size([240.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Quick Launch",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_style({
                let mut style = (*cc.egui_ctx.style()).clone();
                style.spacing.item_spacing = egui::vec2(8.0, 8.0);
                style.spacing.button_padding = egui::vec2(8.0, 4.0);
                style
            });
            Ok(Box::new(gui::QuickLaunchApp::new(cc, script_dir)))
        }),
    )
}

fn main() {
    let args = Args::parse();
    // run_gui(args.path).expect("Failed to launch GUI");
    run_gui("/home/eric/Downloads".parse().unwrap()).expect("Failed to launch GUI");
}
