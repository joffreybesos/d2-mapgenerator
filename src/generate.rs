use std::io::Result;
use std::process::{Command, Output};
use std::fs;

use crate::cache;

fn execute(exe: &str, d2lodarg: &str, seed: &str, difficulty: &str) -> Result<Output> {
    Command::new(exe)
        .arg(d2lodarg)
        .arg("--seed")
        .arg(seed)
        .arg("--difficulty")
        .arg(difficulty)
        // .arg("--map")
        // .arg("1")
        .output()
}


pub fn generate_data(seed: &str, difficulty: &str) -> String {
    // generate data
    let output = execute(
        "E:/Dev/d2-mapserver-rust/mapgen/d2-mapgen.exe",
        "E:/Dev/d2-mapserver-rust/d2lod",
        seed,
        difficulty
    )
    .unwrap();

    // parse stdout
    let start_of_seed_data = format!("{{\"seed\":\"{}\",\"difficulty\":\"{}\",\"levels\":[", seed, difficulty);
    let mut seed_data = String::from(&start_of_seed_data);
    let stdout = String::from_utf8(output.stdout).unwrap();
    for line in stdout.lines() {
        if line.starts_with("{\"type\":\"map\"") {
            seed_data.push_str(line);
            
        }
    }
    seed_data.push_str("]}");

    // save to file
    let cached_seed_data_file = cache::cached_file_name(seed, difficulty);
    fs::write(cached_seed_data_file, &seed_data).expect("Unable to write map data file");
    return seed_data;

}
