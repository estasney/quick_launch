use std::path::{Path, PathBuf};

use crate::lib::fs_scan::{default_quick_launch_dir, default_quick_launch_settings_dir};
use serde::{Deserialize, Serialize};

/// App-wide configuration persisted by `confy`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    /// Directory scanned for executables.
    pub exec_dir: PathBuf,
    pub config_dir: PathBuf,
}

impl Settings {
    pub fn new(exec_dir: PathBuf, config_dir: PathBuf) -> Self {
        Self {
            exec_dir,
            config_dir,
        }
    }

    pub fn set_exec_dir(&mut self, exec_dir: PathBuf) {
        self.exec_dir = exec_dir;
    }

    pub fn set_config_dir(&mut self, config_dir: PathBuf) {
        self.config_dir = config_dir;
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new(
            default_quick_launch_dir(),
            default_quick_launch_settings_dir(),
        )
    }
}
