use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickLaunchExecutable {
    pub display_name: String,
    pub executable_path: std::path::PathBuf,
    pub icon_path: Option<std::path::PathBuf>,
    pub n_launches: u32,
}

impl QuickLaunchExecutable {
    pub fn new(
        display_name: String,
        executable_path: std::path::PathBuf,
        icon_path: Option<std::path::PathBuf>,
        n_launches: u32,
    ) -> Self {
        Self {
            display_name,
            executable_path,
            icon_path,
            n_launches,
        }
    }

    pub fn from_path(executable_path: std::path::PathBuf) -> Self {
        let display_name = executable_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();
        Self {
            display_name,
            executable_path,
            icon_path: None,
            n_launches: 0,
        }
    }

    pub fn increment_launch_count(&mut self) {
        self.n_launches += 1;
    }
}
