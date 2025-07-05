mod app;
use crate::utils::launch::spawn_script_in_terminal;
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub(crate) struct QuickLaunchApp {
    script_dir: PathBuf,   // Directory containing the scripts
    scripts: Vec<PathBuf>, // List of executable scripts in the directory
    num_cols: usize,
    num_rows: usize,
}

impl QuickLaunchApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, script_dir: PathBuf) -> Self {
        let script_copy = script_dir.clone();
        let scripts = Self::find_executables_in_dir(&script_copy);
        let num_cols = 3;
        let num_rows = (scripts.len() + num_cols - 1) / num_cols;

        QuickLaunchApp {
            scripts,
            script_dir,
            num_cols,
            num_rows,
        }
    }

    fn top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel")
            .show(ctx, |ui| {
                let script_dir_text = self.script_dir.display().to_string();
                ui.horizontal_centered(|ui| {
                    ui.label(script_dir_text);
                    if ui.button("\u{1F4C1}").clicked() {
                        // Open the script directory in the file manager
                        if let Err(e) = open::that(&self.script_dir) {
                            eprintln!("Failed to open directory {:?}: {}", self.script_dir, e);
                        }
                    }
                })
            })
            .inner;
    }

    fn action_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
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
                                    let tooltip = script_path
                                        .to_str()
                                        .expect("Failed to convert path to str");
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
                                        // if let Err(e) =
                                        //     std::process::Command::new(script_path).spawn()
                                        // {
                                        //     eprintln!(
                                        //         "Failed to execute script {:?}: {}",
                                        //         script_path, e
                                        //     );
                                        // }
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
        println!("Scanning directory: {:?}", dir);
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
                            println!("Skipping non-executable file: {:?}", path);
                        }
                    }
                }
            }
        } else {
            eprintln!("Failed to read directory: {:?}", dir);
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
            scripts,
            num_cols,
            num_rows,
            script_dir,
        }
    }
}
