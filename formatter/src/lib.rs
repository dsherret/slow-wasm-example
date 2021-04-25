use dprint_core::configuration::resolve_global_config;
use dprint_plugin_json::format_text;
use dprint_plugin_json::configuration::{resolve_config};
use std::collections::HashMap;

#[no_mangle]
pub fn format() -> u32 {
    let global_config = resolve_global_config(HashMap::new()).config;
    let config = resolve_config(HashMap::new(), &global_config).config;
    let file_text = include_str!("../../data/single_line_800kb.json");
    format_text(file_text, &config).unwrap().len() as u32
}
