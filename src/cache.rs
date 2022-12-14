use std::{env, path::{PathBuf}, fs};

pub fn cached_file_name(seed: &u32, difficulty: &u32) -> PathBuf {
    let temp_directory = env::temp_dir();
    let cached_seed_data_file_name = format!("{}_{}.json", seed, difficulty);
    temp_directory.join(cached_seed_data_file_name)
}

pub fn cached_image_file_name(seed: &u32, difficulty: &u32, level_id: &u32) -> PathBuf {
    let temp_directory = env::temp_dir();
    let cached_seed_data_file_name = format!("map_{}_{}_{}.png", seed, difficulty, level_id);
    temp_directory.join(cached_seed_data_file_name)
}

pub fn cached_header_file_name(seed: &u32, difficulty: &u32, level_id: &u32) -> PathBuf {
    let temp_directory = env::temp_dir();
    let cached_seed_data_file_name = format!("map_{}_{}_{}.txt", seed, difficulty, level_id);
    temp_directory.join(cached_seed_data_file_name)
}

pub fn read_cached_file(file_name: &PathBuf) -> String {
    fs::read_to_string(file_name)
        .expect("Unable to read map data file")
}
