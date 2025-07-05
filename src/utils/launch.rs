use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn spawn_script_in_terminal(script_path: &Path) -> io::Result<()> {
    #[cfg(target_os = "linux")]
    {
        // Prefer whatever the user set; otherwise fall back.
        let term = std::env::var("TERMINAL").unwrap_or_else(|_| "x-terminal-emulator".into());

        // Most modern emulators accept -e <cmd …>.
        Command::new(term)
            .args([
                "-e",
                "bash",
                "-c",
                &format!("{path}; exec bash", path = script_path.to_string_lossy()),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?; // detached; don’t .wait()
    }

    #[cfg(target_os = "windows")]
    {
        // “start” opens a new console window.
        Command::new("cmd")
            .args([
                "/C",
                "start",
                "powershell",
                "-NoExit", // keep window up
                "-File",
                script_path,
            ])
            .spawn()?;
    }

    #[cfg(target_os = "macos")]
    {
        // Ask Terminal.app (or iTerm) via AppleScript.
        Command::new("osascript")
            .args([
                "-e",
                &format!(
                    "tell application \"Terminal\" to do script \"bash -c '{}; exec bash'\"",
                    script_path.replace('\'', "'\\''") // escape single quotes
                ),
            ])
            .spawn()?;
    }

    Ok(())
}

pub fn open_native_file_viewer(path: &PathBuf) -> io::Result<()> {
    open::that(path)
}
