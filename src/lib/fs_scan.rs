pub fn default_quick_launch_settings_dir() -> std::path::PathBuf {
    let config_dir = dirs::config_dir().expect("Failed to get config dir");
    let quick_launch_settings_dir = config_dir.join("quick_launch");
    std::fs::create_dir_all(&quick_launch_settings_dir)
        .expect("Failed to create quick launch settings directory");
    quick_launch_settings_dir
}

pub fn default_quick_launch_dir() -> std::path::PathBuf {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let quick_launch_dir = home_dir.join(".quick_launch");
    std::fs::create_dir_all(&quick_launch_dir)
        .expect("Failed to create quick launch directory");
    quick_launch_dir
}
