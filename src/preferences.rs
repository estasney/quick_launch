use crate::utils::config::APP_ID;
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;
use std::path::PathBuf;

/// Store application preferences
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppPreferences {
    /// Directory currently in use for detecting executables
    pub script_dir: Option<PathBuf>,
    /// Our default directory
    pub default_script_dir: PathBuf,
    /// Number of columns in the UI grid
    pub num_cols: NonZeroUsize,
}

fn get_default_script_dir() -> PathBuf {
    dirs::executable_dir().unwrap().join(APP_ID)
}

impl AppPreferences {
    pub fn new(script_dir: Option<PathBuf>, num_cols: NonZeroUsize) -> Self {
        AppPreferences {
            script_dir,
            default_script_dir: get_default_script_dir(),
            num_cols,
        }
    }

    pub fn default() -> Self {
        let default_script_dir = get_default_script_dir();
        let default_script_copy = default_script_dir.clone();

        AppPreferences {
            script_dir: Some(default_script_dir),
            default_script_dir: default_script_copy,
            num_cols: NonZeroUsize::new(3).expect("Default number of columns must be non-zero"),
        }
    }

    pub fn save(&self) {
        confy::store(APP_ID, None, self).expect("Failed to save preferences")
    }

    pub fn load() -> Self {
        confy::load(APP_ID, None).unwrap_or_else(|_| AppPreferences::default())
    }
}

impl Default for AppPreferences {
    fn default() -> Self {
        Self::default()
    }
}
