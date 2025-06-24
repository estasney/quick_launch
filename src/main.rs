mod gui;
mod utils;

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use utils::arg_parser::Args;

fn run_gui(script_dir: PathBuf) -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([660.0, 800.0]),

        ..Default::default()
    };

    eframe::run_native(
        "Quick Launch",
        native_options,
        Box::new(|cc| Ok(Box::new(gui::QuickLaunchApp::new(cc, script_dir)))),
    )
}

fn main() {
    let args = Args::parse();
    // run_gui(args.path).expect("Failed to launch GUI");
    run_gui("/home/eric/Downloads".parse().unwrap()).expect("Failed to launch GUI");
}
