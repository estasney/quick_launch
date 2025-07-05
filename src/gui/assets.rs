use std::sync::Arc;
use egui::FontDefinitions;
use crate::utils::config::ICON_FONT_NAME;

pub const ICON_FONT: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/fonts/SymbolsNerdFont-Regular.ttf"
));

pub fn setup_fonts() -> FontDefinitions {
    let icon_font = egui::FontData::from_static(ICON_FONT);
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(ICON_FONT_NAME.into(), Arc::new(icon_font));
    // Tie the new font to a custom family so we can target it explicitly:
    fonts
        .families
        .entry(egui::FontFamily::Name(ICON_FONT_NAME.into()))
        .or_default()
        .push(ICON_FONT_NAME.into());
    // (optional) make it a fallback for proportional text so you can mix words + icons:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .push(ICON_FONT_NAME.into());
    fonts
}