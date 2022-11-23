use std::path::PathBuf;

use dprint_plugin_typescript::format_text;
use dprint_plugin_typescript::configuration::resolve_config;

#[no_mangle]
pub fn format() -> u32 {
    let config = resolve_config(Default::default(), &Default::default()).config;
    let file_text = include_str!("../../data/large_file.ts");
    format_text(&PathBuf::from("file.ts"), file_text, &config).unwrap().unwrap().len() as u32
}
