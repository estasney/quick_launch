use std::path::{Path, PathBuf};
fn asset_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("assets")
}
pub const ICON_FONT: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/fonts/SymbolsNerdFont-Regular.ttf"
));
