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
use std::path::{Path, PathBuf};

pub(crate) struct QuickLaunchApp {
    scripts: Vec<PathBuf>, // List of executable scripts in the directory
    num_rows: usize,
    pick_folder_task: Option<Task<Option<PathBuf>>>,
    /// Async handle for selecting folder
    app_preferences: AppPreferences,
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
        let pref_copy = app_preferences.clone();
        let script_dir_copy = app_preferences.script_dir.clone();
        let default_dir_copy = app_preferences.default_script_dir.clone();
        let num_cols: usize = app_preferences.num_cols.get();

        let target_directory: PathBuf = { script_dir_copy.unwrap_or(default_dir_copy) };

        let scripts = Self::find_executables_in_dir(&target_directory);
        let num_rows = compute_n_rows(num_cols, scripts.len());

        cc.egui_ctx.set_fonts(setup_fonts());
        QuickLaunchApp {
            scripts,
            num_rows,
            pick_folder_task: None,
            app_preferences: pref_copy,
        }
    }

    fn save_preferences(&self) {
        self.app_preferences.save()
    }

    fn exit_application(&self, ui: &mut egui::Ui) {
        ui.ctx().request_repaint();
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }

    fn get_script_dir(&self) -> &Path {
        self.app_preferences
            .script_dir
            .as_deref()
            .unwrap_or(&self.app_preferences.default_script_dir)
    }

    fn set_script_dir(&mut self, path: PathBuf) {
        self.app_preferences.script_dir = Some(path);
        self.save_preferences();
    }

    fn num_cols(&self) -> usize {
        self.app_preferences.num_cols.get()
    }

    fn script_dir_component(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            let open_folder_target: &Path = self.get_script_dir();
            let script_dir_text = self.get_script_dir();

            ui.label(script_dir_text.to_string_lossy());
            if folder_button(ui)
                .on_hover_text("Open Script Folder")
                .clicked()
            {
                open_native_file_viewer(open_folder_target).expect("Failed to open directory");
            }
            if folder_open_dialog(ui)
                .on_hover_text("Pick Script Folder")
                .clicked()
                && self.pick_folder_task.is_none()
            {
                self.pick_folder_task = Some(pick_folder_async());
            }
            if let Some(task) = &mut self.pick_folder_task {
                if let Some(new_dir) = task.try_take() {
                    if let Some(new_dir) = new_dir {
                        self.set_script_dir(new_dir);
                        self.rescan_dir();
                    }
                    self.pick_folder_task = None;
                }
            }
        });
    }

    /// Renders top panel showing the Location of the script directory, and buttons for modifying or
    /// viewing
    fn top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.script_dir_component(ui);
        });
    }

    fn action_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let spacing = ui.style().spacing.item_spacing.x;
            let total_width = ui.available_width();
            let num_cols = self.app_preferences.num_cols.get();
            let button_width = (total_width - spacing * 2.0) / num_cols as f32;
            let button_size = egui::vec2(button_width, 32.0);
            egui::Grid::new("action_grid")
                .spacing(egui::vec2(spacing, spacing))
                .striped(true)
                .show(ui, |ui| {
                    for row in 0..self.num_rows {
                        for col in 0..num_cols {
                            let index = row * num_cols + col;
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
                                    self.exit_application(ui);
                                }
                            }
                        }
                        ui.end_row();
                    }
                })
        });
    }

    /// We've either picked a new directory or the user has requested a rescan.
    fn rescan_dir(&mut self) {
        self.scripts = Self::find_executables_in_dir(self.get_script_dir());
        self.num_rows = compute_n_rows(self.num_cols(), self.scripts.len());
    }

    /// Scans the given directory and returns a Vec of PathBufs for all executable files.
    pub fn find_executables_in_dir(dir: &Path) -> Vec<PathBuf> {
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
