use std::path::Path;

pub fn is_executable(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        path.metadata()
            .map(|m| m.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }

    #[cfg(windows)]
    {
        let executable_extensions = ["exe", "cmd", "bat", "com"];
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| executable_extensions.contains(&ext.to_lowercase().as_str()))
            .unwrap_or(false)
    }
}