mod gui;
mod utils;
use std::path::PathBuf;
use eframe::icon_data;
use crate::utils::arg_parser::build_cli;

fn run_gui(script_dir: PathBuf) -> eframe::Result {
    const ICON_BYTES: &[u8] = include_bytes!("assets/icon.png");
    let icon = icon_data::from_png_bytes(ICON_BYTES)
        .expect("icon must be valid 32-bit PNG");
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([660.0, 800.0]).with_min_inner_size([240.0, 240.0]).with_icon(icon).with_title("Quick Launch"),
        ..Default::default()
    };

    eframe::run_native(
        "Quick Launch",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_style({
                let mut style = (*cc.egui_ctx.style()).clone();
                style.spacing.item_spacing = egui::vec2(8.0, 8.0);
                style.spacing.button_padding = egui::vec2(8.0, 8.0);
                style
            });
            Ok(Box::new(gui::QuickLaunchApp::new(cc, script_dir)))
        }),
    )
}

fn main() {
    let parsed_args = build_cli().get_matches();
    let script_dir = parsed_args.get_one::<PathBuf>("dir").expect("We have a default dir").clone();
    {
        if !script_dir.exists() {
            eprintln!("The specified directory does not exist: {:?}", script_dir);
            std::fs::create_dir_all(&script_dir).expect("Failed to create the directory");
        }
    }
    run_gui(script_dir).expect(
        "Failed to run the GUI. Please make sure you have the latest version of the GUI installed.",
    )
}
