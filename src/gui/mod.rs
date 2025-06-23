mod app;

use crate::config::app_config::QuickLaunchAppSettings;
use crate::config::profile::QuickLaunchProfileSettings;
use std::slice::Iter;

pub(crate) struct QuickLaunchApp {
    settings: QuickLaunchAppSettings,
    current_profile: String,
}

impl QuickLaunchApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        QuickLaunchApp::default()
    }

    pub fn settings(&self) -> &QuickLaunchAppSettings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut QuickLaunchAppSettings {
        &mut self.settings
    }

    pub fn profiles(&self) -> Iter<QuickLaunchProfileSettings> {
        self.settings.profiles.iter()
    }

    pub fn iter_profiles(&self) -> impl Iterator<Item = &QuickLaunchProfileSettings> {
        self.settings.iter_profiles()
    }

    fn top_bar(&mut self, ctx: &egui::Context) -> egui::Response {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::Grid::new("profile-grid")
                    .num_columns(1)
                    .spacing([40.0, 4.0])
                    .show(ui, |ui| {
                        ui.label("Profile:");
                        egui::ComboBox::from_id_salt("Profile")
                            .selected_text(self.current_profile.clone())
                            .show_ui(ui, |ui| {
                                for profile in self.settings.profiles.iter() {
                                    let is_selected = self.current_profile == profile.name;
                                    if ui.selectable_label(is_selected, &profile.name).clicked() {
                                        self.current_profile = profile.name.clone();
                                    }
                                }
                            });
                    });
                if ui.button("Add Script").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .set_title("Select Script")
                        .pick_file()
                    {
                        let profile: &QuickLaunchProfileSettings = self.settings.profile_by_name(&self.current_profile)
                            .expect("Current profile should always exist");
                        profile.add_executable_from_path(path);
                        
                    }
                    
                }
            })
        }).response
    }

}

impl Default for QuickLaunchApp {
    fn default() -> Self {

        let mut settings = QuickLaunchAppSettings::load().expect("Failed to load launch settings");
        // Ensure at least one profile exists, if not, create a default one
        if settings.profiles.is_empty() {
            settings.profiles.push(QuickLaunchProfileSettings::default());
        }
        // Our current profile is just the first one by default

        let current_profile = settings.profiles.first().map(|p| p.name.clone()).expect("This should never happen, we just created a default profile");

        Self {
            settings: QuickLaunchAppSettings::load().expect("Failed to load config file"),
            current_profile,
        }
    }
}
