pub(crate) struct QuickLaunchApp {
    quick_launch_dir: Option<std::path::PathBuf>, 
    quick_launch_files: Vec<std::path::PathBuf>,
}

impl QuickLaunchApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        QuickLaunchApp::default()
    }
    
}

impl Default for QuickLaunchApp {
    fn default() -> Self {
        QuickLaunchApp {
            quick_launch_dir: None,
            quick_launch_files: Vec::new(),
        }
    }
}