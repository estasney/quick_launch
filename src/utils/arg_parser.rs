use clap::{Arg, Command, value_parser, ValueHint};
use std::path::{PathBuf};

pub fn build_cli() -> Command {
    let def_dir = dirs::executable_dir().unwrap().join("quick_launch").into_os_string();
        
        

    Command::new("cli")
        .arg(
            Arg::new("dir")
                .long("dir")
                .value_name("DIR")
                .value_hint(ValueHint::DirPath)
                .value_parser(value_parser!(PathBuf))
                .default_value_os(def_dir.clone())
        )
}


