use eframe::epaint::FontFamily;
use egui::RichText;
use crate::utils::config::ICON_FONT_NAME;

const FOLDER_ICON_CHAR: char = '\u{ea83}';
const FOLDER_MOVE_ICON_CHAR: char = '\u{e5fc}';

pub fn icon_button(ui: &mut egui::Ui, codepoint: char) -> egui::Response {
    let icon = RichText::new(codepoint.to_string())
        .font(egui::FontId::new(16.0, FontFamily::Name(ICON_FONT_NAME.into())));
    ui.button(icon)
}

pub fn folder_button(ui: &mut egui::Ui) -> egui::Response {
    icon_button(ui, FOLDER_ICON_CHAR)
}

pub fn folder_open_dialog(ui: &mut egui::Ui) -> egui::Response {
    icon_button(ui, FOLDER_MOVE_ICON_CHAR)
}