use serde::{Deserialize, Serialize};
use crate::config::executables::QuickLaunchExecutable;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuickLaunchProfileSettings {
    pub name: String,
    pub executables: Vec<QuickLaunchExecutable>,
}


impl QuickLaunchProfileSettings {
    pub fn add_executable_from_path(&mut self, executable_path: std::path::PathBuf) {
        let executable = QuickLaunchExecutable::from_path(executable_path);
        self.executables.push(executable);
    }
    
}

impl Default for QuickLaunchProfileSettings {
    fn default() -> Self {
        Self {
            name: "default".to_owned(),
            executables: vec![],
        }
    }
    
    
    
}