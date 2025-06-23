use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    /// Profile name to load settings from
    #[arg(short, long, value_name = "PROFILE", default_value = "default")] 
    profile: String,
}

