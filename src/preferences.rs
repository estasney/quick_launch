use crate::utils::config::APP_ID;
use serde::{Deserialize, Serialize};
use std::num::{NonZeroUsize};

/// Store application preferences
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppPreferences {
    /// Directory containing the scripts
    pub script_dir: String,
    /// Number of columns in the UI grid
    pub num_cols: NonZeroUsize,
}

impl AppPreferences {
    pub fn new(script_dir: String, num_cols: NonZeroUsize) -> Self {
        AppPreferences {
            script_dir,
            num_cols,
        }
    }

    pub fn default() -> Self {
        AppPreferences {
            script_dir: dirs::executable_dir()
                .unwrap()
                .join(APP_ID)
                .to_string_lossy()
                .parse()
                .unwrap(),
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