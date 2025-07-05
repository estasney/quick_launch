mod gui;
mod utils;
mod preferences;


use eframe::icon_data;
use crate::utils::config::APP_TITLE;

fn run_gui() -> eframe::Result {
    const ICON_BYTES: &[u8] = include_bytes!("../assets/icons/icon.png");
    let icon = icon_data::from_png_bytes(ICON_BYTES)
        .expect("icon must be valid 32-bit PNG");
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([660.0, 800.0]).with_min_inner_size([240.0, 240.0]).with_icon(icon).with_title(APP_TITLE),
        ..Default::default()
    };

    let app_preferences = preferences::AppPreferences::load();

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
            Ok(Box::new(gui::QuickLaunchApp::new(cc, app_preferences)))
        }),
    )
}

fn main() {
    run_gui().expect(
        "Failed to run the GUI. Please make sure you have the latest version of the GUI installed.",
    )
}
