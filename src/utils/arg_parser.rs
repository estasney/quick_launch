use clap::Parser;
use clap::builder::ValueParserFactory;
use std::{
    path::{PathBuf},
};

#[derive(Parser, Debug)]
pub(crate) struct Args {
    /// Directory to scan for executables
    #[arg(short, long, value_name = "PATH", value_parser = parse_path, default_value = "./")]
    pub path: PathBuf,
}

fn parse_path(path: &str) -> Result<PathBuf, String> {
    let p = PathBuf::from(path);
    let exists = p.exists();
    let is_dir = p.is_dir();
    if exists && is_dir {
        Ok(p.canonicalize().unwrap())
    } else if is_dir {
        Err(format!("Directory does not exist: {}", path))
    } else {
        Err(format!("Path is not a directory: {}", path))
    }
}
