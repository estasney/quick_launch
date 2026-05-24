mod app;
mod assets;
mod icon_button;

use crate::gui::assets::setup_fonts;
use crate::gui::icon_button::{folder_button, folder_open_dialog};
use crate::preferences::AppPreferences;
use crate::utils::build_tree::{RootFolder, build_tree};
use crate::utils::launch::{open_native_file_viewer, pick_folder_async, spawn_script_in_terminal};
use crate::utils::task::Task;
use std::path::{Path, PathBuf};

pub(crate) struct QuickLaunchApp {
    root_folder: RootFolder,
    pick_folder_task: Option<Task<Option<PathBuf>>>,
    app_preferences: AppPreferences,
}

impl QuickLaunchApp {
    pub fn new(cc: &eframe::CreationContext<'_>, app_preferences: AppPreferences) -> Self {
        let target_directory = app_preferences
            .script_dir
            .clone()
            .unwrap_or_else(|| app_preferences.default_script_dir.clone());

        let root_folder = build_tree(&target_directory);

        cc.egui_ctx.set_fonts(setup_fonts());
        QuickLaunchApp {
            root_folder,
            pick_folder_task: None,
            app_preferences,
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
            if let Some(task) = &mut self.pick_folder_task
                && let Some(new_dir) = task.try_take()
            {
                if let Some(new_dir) = new_dir {
                    self.set_script_dir(new_dir);
                    self.rescan_dir();
                }
                self.pick_folder_task = None;
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

            let num_rows = self.root_folder.entries.len().div_ceil(num_cols);
            egui::Grid::new("root")
                .spacing(egui::vec2(spacing, spacing))
                .show(ui, |ui| {
                    for row in 0..num_rows {
                        for col in 0..num_cols {
                            let index = row * num_cols + col;
                            if let Some(entry) = self.root_folder.entries.get(index) {
                                let tooltip = entry.executable_path.to_string_lossy();
                                if ui
                                    .add_sized(button_size, egui::Button::new(&entry.name))
                                    .on_hover_text(tooltip.as_ref())
                                    .clicked()
                                {
                                    spawn_script_in_terminal(&entry.executable_path)
                                        .expect("Failed to spawn script in terminal");
                                    self.exit_application(ui);
                                }
                            }
                        }
                        ui.end_row();
                    }
                });

            for folder in &self.root_folder.folders {
                if folder.flat_entries.is_empty() {
                    continue;
                }

                ui.separator();
                let padding = egui::vec2(8.0, 4.0);
                let galley = ui.painter().layout_no_wrap(
                    folder.name.clone(),
                    egui::FontId::default(),
                    egui::Color32::WHITE,
                );
                let (rect, _) =
                    ui.allocate_exact_size(galley.size() + padding * 2.0, egui::Sense::hover());
                ui.painter()
                    .rect_filled(rect, 4.0, egui::Color32::from_gray(40));
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    &folder.name,
                    egui::FontId::default(),
                    egui::Color32::WHITE,
                );

                let folder_rows = folder.flat_entries.len().div_ceil(num_cols);
                egui::Grid::new(&folder.name)
                    .spacing(egui::vec2(spacing, spacing))
                    .show(ui, |ui| {
                        for row in 0..folder_rows {
                            for col in 0..num_cols {
                                let index = row * num_cols + col;
                                if let Some(entry) = folder.flat_entries.get(index) {
                                    let tooltip = entry.executable_path.to_string_lossy();
                                    if ui
                                        .add_sized(
                                            button_size,
                                            egui::Button::new(&entry.display_name),
                                        )
                                        .on_hover_text(tooltip.as_ref())
                                        .clicked()
                                    {
                                        spawn_script_in_terminal(&entry.executable_path)
                                            .expect("Failed to spawn script in terminal");
                                        self.exit_application(ui);
                                    }
                                }
                            }
                            ui.end_row();
                        }
                    });
            }
        });
    }

    fn rescan_dir(&mut self) {
        self.root_folder = build_tree(self.get_script_dir());
    }
}
