mod app;
mod assets;

use crate::gui::assets::ICON_FONT;
use crate::preferences::AppPreferences;
use crate::utils::launch::{open_native_file_viewer, spawn_script_in_terminal};
use egui::RichText;
use egui::{FontData, FontDefinitions, FontFamily};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::Arc;

const FOLDER_ICON_CHAR: char = '\u{ea83}';
const FOLDER_MOVE_ICON_CHAR: char = '\u{e5fc}';

pub(crate) struct QuickLaunchApp {
    script_dir: PathBuf,   // Directory containing the scripts
    scripts: Vec<PathBuf>, // List of executable scripts in the directory
    num_cols: usize,
    num_rows: usize,
}

impl QuickLaunchApp {
    pub fn new(cc: &eframe::CreationContext<'_>, app_preferences: AppPreferences) -> Self {
        let script_dir: PathBuf = app_preferences.script_dir.into();
        let num_cols: usize = app_preferences.num_cols.get();
        let scripts = Self::find_executables_in_dir(&script_dir);
        let num_rows = Self::compute_n_rows(num_cols, scripts.len());
        let icon_font = FontData::from_static(ICON_FONT);
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert("icons".into(), Arc::new(icon_font));
        // Tie the new font to a custom family so we can target it explicitly:
        fonts
            .families
            .entry(FontFamily::Name("icons".into()))
            .or_default()
            .push("icons".into());

        // (optional) make it a fallback for proportional text so you can mix words + icons:
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .push("icons".into());

        cc.egui_ctx.set_fonts(fonts);

        QuickLaunchApp {
            scripts,
            script_dir,
            num_cols,
            num_rows,
        }
    }

    fn compute_n_rows(n_cols: usize, n_items: usize) -> usize {
        if n_items == 0 {
            0
        } else {
            n_items.div_ceil(n_cols)
        }
    }

    fn icon_button(ui: &mut egui::Ui, codepoint: char) -> egui::Response {
        let icon = RichText::new(codepoint.to_string())
            .font(egui::FontId::new(16.0, FontFamily::Name("icons".into())));
        ui.button(icon)
    }

    fn top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            let script_dir_text = self.script_dir.display().to_string();
            ui.horizontal_centered(|ui| {
                ui.label(script_dir_text);
                if Self::icon_button(ui, FOLDER_ICON_CHAR)
                    .on_hover_text("Open Script Folder")
                    .clicked()
                {
                    open_native_file_viewer(&self.script_dir).expect("Failed to open directory");
                }
            })
        });
    }

    fn action_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let spacing = ui.style().spacing.item_spacing.x;
            let total_width = ui.available_width();
            let button_width = (total_width - spacing * 2.0) / self.num_cols as f32;
            let button_size = egui::vec2(button_width, 32.0);
            egui::Grid::new("action_grid")
                .spacing(egui::vec2(spacing, spacing))
                .striped(true)
                .show(ui, |ui| {
                    for row in 0..self.num_rows {
                        for col in 0..self.num_cols {
                            let index = row * self.num_cols + col;
                            if index < self.scripts.len() {
                                let script_path = &self.scripts[index];
                                let tooltip =
                                    script_path.to_str().expect("Failed to convert path to str");
                                let script_name = script_path
                                    .file_name()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or("Unknown");
                                if ui
                                    .add_sized(button_size, egui::Button::new(script_name))
                                    .on_hover_text(tooltip)
                                    .clicked()
                                {
                                    spawn_script_in_terminal(script_path)
                                        .expect("Failed to spawn script in terminal");
                                }
                            }
                        }
                        ui.end_row();
                    }
                })
        });
    }

    /// Scans the given directory and returns a Vec of PathBufs for all executable files.
    pub fn find_executables_in_dir(dir: &PathBuf) -> Vec<PathBuf> {
        println!("Scanning directory: {dir:?}");
        let mut executables = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                let path = entry.unwrap().path();
                if path.is_file() {
                    if let Ok(metadata) = fs::metadata(&path) {
                        let permissions = metadata.permissions();
                        // Check if owner, group, or others have execute permission
                        if permissions.mode() & 0o111 != 0 {
                            executables.push(path);
                        } else {
                            println!("Skipping non-executable file: {path:?}");
                        }
                    }
                }
            }
        } else {
            eprintln!("Failed to read directory: {dir:?}");
        }
        executables
    }
}
