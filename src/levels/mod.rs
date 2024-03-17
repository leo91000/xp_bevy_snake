use std::fs::read_to_string;

use crate::components::point_list::PointList;
use serde::Deserialize;
use serde_json::from_str;

#[derive(Deserialize)]
pub struct Level {
    pub name: String,
    pub obstacles: Vec<PointList>,
}

#[allow(dead_code)]
pub fn load_level_at_runtime(level_name: &str) -> Level {
    let path = format!("assets/levels/{level_name}.json");
    let level = read_to_string(path).expect("Failed to read level file");
    from_str(&level).expect("Failed to parse level file")
}

#[macro_export]
macro_rules! load_level {
    ($level_name:literal) => {{
        let level_src = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/levels/",
            $level_name,
            ".json"
        ));
        let level: $crate::levels::Level =
            serde_json::from_str(level_src).expect("Failed to parse level file");
        level
    }};
}
