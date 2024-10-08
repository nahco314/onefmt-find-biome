mod finding;

use crate::finding::find_biome;
use foro_plugin_utils::data_json_utils::JsonGetter;
use foro_plugin_utils::foro_plugin_setup;
use serde_json::{json, Value};
use std::path::PathBuf;

pub fn main_with_json(input: Value) -> Value {
    let current_dir = String::get_value(&input, ["current-dir"]).unwrap();

    let result = match find_biome(&PathBuf::from(current_dir)) {
        Some(v) => {
            json!({
                "found": true,
                "biome": v.to_str().unwrap()
            })
        }
        None => {
            json!({
                "found": false
            })
        }
    };

    result
}

foro_plugin_setup!(main_with_json);
