use std::path::{Path, PathBuf};

use super::file_ops::is_executable;

pub struct RootFolder {
    pub entries: Vec<Entry>,
    pub folders: Vec<Folder>,
}

impl RootFolder {
    pub fn sort_by_usage(&mut self, score: &dyn Fn(&Path) -> u64) {
        self.entries
            .sort_by(|a, b| score(&b.executable_path).cmp(&score(&a.executable_path)));
        for folder in &mut self.folders {
            folder
                .flat_entries
                .sort_by(|a, b| score(&b.executable_path).cmp(&score(&a.executable_path)));
        }
    }
}

pub struct FlatEntry {
    pub display_name: String,
    pub executable_path: PathBuf,
}

pub struct Folder {
    pub name: String,
    pub entries: Vec<Entry>,
    pub folders: Vec<Folder>,
    pub flat_entries: Vec<FlatEntry>,
}

pub struct Entry {
    pub name: String,
    pub executable_path: PathBuf,
}

pub fn build_tree(root_path: &Path) -> RootFolder {
    let (entries, folders) = read_children(root_path);
    RootFolder { entries, folders }
}

fn read_children(dir: &Path) -> (Vec<Entry>, Vec<Folder>) {
    let dir_entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return (Vec::new(), Vec::new()),
    };

    let mut entries = Vec::new();
    let mut folders = Vec::new();

    for dir_entry in dir_entries.flatten() {
        let path = dir_entry.path();

        if path.is_dir() {
            let (child_entries, child_folders) = read_children(&path);
            let mut flat_entries = Vec::new();
            flatten_entries(
                &child_entries,
                &child_folders,
                PathBuf::new(),
                &mut flat_entries,
            );
            folders.push(Folder {
                name: file_name_string(&path),
                entries: child_entries,
                folders: child_folders,
                flat_entries,
            });
        } else if is_executable(&path) {
            entries.push(Entry {
                name: file_name_string(&path),
                executable_path: path,
            });
        }
    }

    (entries, folders)
}

fn flatten_entries(
    entries: &[Entry],
    folders: &[Folder],
    prefix: PathBuf,
    results: &mut Vec<FlatEntry>,
) {
    for entry in entries {
        let display_name = if prefix.as_os_str().is_empty() {
            entry.name.clone()
        } else {
            prefix.join(&entry.name).to_string_lossy().into_owned()
        };
        results.push(FlatEntry {
            display_name,
            executable_path: entry.executable_path.clone(),
        });
    }
    for folder in folders {
        flatten_entries(
            &folder.entries,
            &folder.folders,
            prefix.join(&folder.name),
            results,
        );
    }
}

fn file_name_string(path: &Path) -> String {
    path.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
}
