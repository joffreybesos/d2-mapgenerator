use std::{env, path::{PathBuf}, fs};

use serde_json::Value;

pub fn cached_file_name(seed: &str, difficulty: &str) -> PathBuf {
    let temp_directory = env::temp_dir();
    let cached_seed_data_file_name = format!("{}_{}.json", seed, difficulty);
    temp_directory.join(cached_seed_data_file_name)
}

pub fn cached_image_file_name(seed: &str, difficulty: &str, level_id: &Value) -> PathBuf {
    let temp_directory = env::temp_dir();
    let cached_seed_data_file_name = format!("map_{}_{}_{}.png", seed, difficulty, level_id);
    temp_directory.join(cached_seed_data_file_name)
}


pub fn read_cached_file(file_name: &PathBuf) -> String {
    fs::read_to_string(file_name)
        .expect("Unable to read map data file")
}