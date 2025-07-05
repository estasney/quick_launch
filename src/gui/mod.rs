mod app;
mod assets;
mod icon_button;

use crate::gui::assets::setup_fonts;
use crate::gui::icon_button::{folder_button, folder_open_dialog};
use crate::preferences::AppPreferences;
use crate::utils::launch::{open_native_file_viewer, pick_folder_async, spawn_script_in_terminal};
use crate::utils::task::Task;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub(crate) struct QuickLaunchApp {
    script_dir: PathBuf,   // Directory containing the scripts
    scripts: Vec<PathBuf>, // List of executable scripts in the directory
    num_cols: usize,
    num_rows: usize,
    pick_folder_task: Option<Task<Option<PathBuf>>>,
}

fn compute_n_rows(n_cols: usize, n_items: usize) -> usize {
    if n_items == 0 {
        0
    } else {
        n_items.div_ceil(n_cols)
    }
}

impl QuickLaunchApp {
    pub fn new(cc: &eframe::CreationContext<'_>, app_preferences: AppPreferences) -> Self {
        let script_dir: PathBuf = app_preferences.script_dir.into();
        let num_cols: usize = app_preferences.num_cols.get();
        let scripts = Self::find_executables_in_dir(&script_dir);
        let num_rows = compute_n_rows(num_cols, scripts.len());
        cc.egui_ctx.set_fonts(setup_fonts());
        QuickLaunchApp {
            scripts,
            script_dir,
            num_cols,
            num_rows,
            pick_folder_task: None,
        }
    }

    fn top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            let script_dir_text = self.script_dir.display().to_string();
            ui.horizontal_centered(|ui| {
                ui.label(script_dir_text);
                if folder_button(ui)
                    .on_hover_text("Open Script Folder")
                    .clicked()
                {
                    open_native_file_viewer(&self.script_dir).expect("Failed to open directory");
                }
                if folder_open_dialog(ui).on_hover_text("Pick Script Folder").clicked() && self.pick_folder_task.is_none() {
                    self.pick_folder_task = Some(pick_folder_async());
                }
                if let Some(task) = &mut self.pick_folder_task {
                    if let Some(new_dir) = task.try_take() {
                        if let Some(new_dir) = new_dir {
                            self.script_dir = new_dir;
                            self.scripts = Self::find_executables_in_dir(&self.script_dir);
                            self.num_rows = compute_n_rows(self.num_cols, self.scripts.len());
                        }
                        self.pick_folder_task = None; // Reset the task
                    }
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
