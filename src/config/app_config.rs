use confy::ConfyError;
use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use crate::config::profile::QuickLaunchProfileSettings;

const APP_NAME: &str = "quick_launch";

#[derive(Debug)]
pub enum ProfileError {
    DuplicateProfileName(String),
    ProfileNotFound(String),
}

impl fmt::Display for ProfileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ProfileError::DuplicateProfileName(name) => {
                write!(f, "Profile with name '{}' already exists.", name)
            }
            ProfileError::ProfileNotFound(name) => {
                write!(f, "Profile with name '{}' not found.", name)
            }
        }
    }
}

impl std::error::Error for ProfileError {}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuickLaunchAppSettings {
    pub profiles: Vec<QuickLaunchProfileSettings>,
}

impl Default for QuickLaunchAppSettings {
    fn default() -> Self {
        Self {
            profiles: vec![QuickLaunchProfileSettings::default()],
        }
    }
}

impl QuickLaunchAppSettings {
    pub fn new(profiles: Vec<QuickLaunchProfileSettings>) -> Self {
        Self { profiles }
    }

    pub fn profile_names(&self) -> Vec<String> {
        self.iter_profiles().map(|p| p.name.clone()).collect()
    }

    pub fn profile_by_name(&self, name: &str) -> Option<&QuickLaunchProfileSettings> {
        self.iter_profiles().find(|p| p.name == name.to_owned())
    }

    pub fn add_profile(&mut self, profile: QuickLaunchProfileSettings) -> Result<(), ProfileError> {
        let profile_name = profile.name.clone();
        if self.profiles.iter().any(|p| p.name == profile_name) {
            return Err(ProfileError::ProfileNotFound(profile_name));
        }
        self.profiles.push(profile);
        self.flush().unwrap_or_else(|e| {
            eprintln!("Failed to save settings: {}", e);
        });
        Ok(())
    }

    pub fn remove_profile(&mut self, name: &str) -> Result<(), ProfileError> {
        if let Some(index) = self.profiles.iter().position(|p| p.name == name) {
            self.profiles.remove(index);
            self.flush().unwrap_or_else(|e| {
                eprintln!("Failed to save settings: {}", e);
            });
            Ok(())
        } else {
            Err(ProfileError::ProfileNotFound(name.to_owned()))
        }
    }
    
    pub fn has_profile(&self, name: &str) -> bool {
        self.iter_profiles()
            .any(|p| p.name == name)
    }
    
    pub fn iter_profiles(&self) -> impl Iterator<Item = &QuickLaunchProfileSettings> {
        self.profiles.iter()
    }

    pub fn flush(&self) -> Result<(), ConfyError> {
        confy::store(APP_NAME, None, self)
    }

    pub fn load() -> Result<Self, ConfyError> {
        confy::load(APP_NAME, None)
    }
}