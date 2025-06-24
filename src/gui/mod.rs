mod app;

use egui::{Button, Widget};
use std::env;
use std::path::PathBuf;

pub(crate) struct QuickLaunchApp {
    script_dir: PathBuf,
    scripts: Vec<PathBuf>,
    num_cols: usize,
    num_rows: usize,
    spacing: f32
}

impl QuickLaunchApp {
    pub fn new(cc: &eframe::CreationContext<'_>, script_dir: PathBuf) -> Self {
        let script_copy = script_dir.clone();
        let scripts = Self::find_executables_in_dir(&script_copy);
        let num_cols = 3; // Default number of columns
        let num_rows = (scripts.len() + num_cols - 1) / num_cols;

        QuickLaunchApp {
            script_dir,
            scripts,
            num_cols,
            num_rows,
            spacing: 8.0
        }
    }
    fn action_panel(&mut self, ctx: &egui::Context) {
        let ui = egui::CentralPanel::default()
            .show(ctx, |ui| {
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
                                    let script_name = script_path
                                        .file_name()
                                        .and_then(|s| s.to_str())
                                        .unwrap_or("Unknown");
                                    if ui.add_sized(button_size, egui::Button::new(script_name)).clicked() {
                                        if let Err(e) =
                                            std::process::Command::new(script_path).spawn()
                                        {
                                            eprintln!(
                                                "Failed to execute script {:?}: {}",
                                                script_path, e
                                            );
                                        }
                                    }
                                    
                                }
                            }
                            ui.end_row();
                        }
                    })
            })
            .inner;
    }

    /// Scans the given directory and returns a Vec of PathBufs for all executable files.
    pub fn find_executables_in_dir(dir: &PathBuf) -> Vec<PathBuf> {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;
        println!("Scanning directory: {:?}", dir);
        let mut executables = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Ok(metadata) = fs::metadata(&path) {
                        let permissions = metadata.permissions();
                        // Check if owner, group, or others have execute permission
                        if permissions.mode() & 0o111 != 0 {
                            executables.push(path);
                        }
                    }
                }
            }
        }
        executables
    }
}

impl Default for QuickLaunchApp {
    fn default() -> Self {
        let script_dir = env::current_dir().expect("Failed to get current directory");
        let script_copy = script_dir.clone();
        let scripts = Self::find_executables_in_dir(&script_copy);
        let num_cols = 3; // Default number of columns
        let num_rows = (scripts.len() + num_cols - 1) / num_cols;
        QuickLaunchApp {
            script_dir,
            scripts,
            num_cols,
            num_rows,
            spacing: 8.0
        }
    }
}
