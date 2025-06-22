mod gui;


mod lib;

use lib::config::Settings;

use egui::ViewportBuilder;
use serde::{Deserialize, Serialize};

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([370.0, 600.0])
            .with_min_inner_size([250.0, 400.0]),
        ..Default::default()
    };
    let mut cfg: Settings = confy::load("quick_launch", None).unwrap();
    // cfg.set_config_dir("~/.config/quick_launch".into());
    
    print!("{:?}", cfg)
    
        
    
}
