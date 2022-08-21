use std::path::{PathBuf, Path};
use std::process::{Command, Output};
use std::fs;
use colored::*;
use serde_json::Error;

use crate::cache;
use crate::data::SeedData;

pub fn get_seed_data(seed: &u32, difficulty: &u32, d2lod: &PathBuf, blachaexe: &PathBuf) -> SeedData {
    let cached_seed_data_file = cache::cached_file_name(seed, difficulty);
    let seed_data_str: String = if Path::new(&cached_seed_data_file).exists() {
        println!("Reading cached map data from file {}", &cached_seed_data_file.to_str().unwrap());
        cache::read_cached_file(&cached_seed_data_file)
    } else {
        println!("Generating fresh data for seed {} and difficulty {}", seed, difficulty);
        generate_data(seed, difficulty, d2lod, blachaexe)
    };
    let json: Result<SeedData, Error> = serde_json::from_str(&seed_data_str);
    match json {
        Ok(json) => json,
        Err(e) => {
            delete_cached_file(&cached_seed_data_file);
            panic!("{} {}", "Failed to generate map data!".red().bold(), e);
        }
    }
}

fn delete_cached_file(cached_seed_data_file: &PathBuf) {
    fs::remove_file(cached_seed_data_file).unwrap();
}

fn execute(blachaexe: &PathBuf, d2lod: &PathBuf, seed: &u32, difficulty: &u32) -> std::io::Result<Output> {
    Command::new(blachaexe)
        .arg(d2lod)
        .arg("--seed")
        .arg(seed.to_string())
        .arg("--difficulty")
        .arg(difficulty.to_string())
        // .arg("--map")
        // .arg("1")
        .output()
}

pub fn generate_data(seed: &u32, difficulty: &u32, d2lod: &PathBuf, blachaexe: &PathBuf) -> String {
    // generate data
    let output = execute(
        blachaexe,
        d2lod,
        seed,
        difficulty
    )
    .unwrap();

    // parse stdout
    let start_of_seed_data = format!("{{\"seed\":{},\"difficulty\":{},\"levels\":[", seed, difficulty);
    let mut seed_data = String::from(&start_of_seed_data);
    let stdout = String::from_utf8(output.stdout).unwrap();
    for line in stdout.lines() {
        if line.starts_with("{\"type\":\"map\"") {
            seed_data.push_str(line);
            seed_data.push_str(",");
        }
    }
    seed_data.pop();
    seed_data.push_str("]}");

    // save to file
    let cached_seed_data_file = cache::cached_file_name(seed, difficulty);
    fs::write(cached_seed_data_file, &seed_data).expect("Unable to write map data file");
    return seed_data;

}
