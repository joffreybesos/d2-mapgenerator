use std::{path::Path, time::Instant};

use serde_json::{Value, Map};

mod generate;
mod cache;
mod mapdata;
mod image;

fn main() {
    
    let seed: &str = "76546";
    let difficulty: &str = "2";

    let start = Instant::now();

    let seed_data_json: Value = get_seed_data(seed, difficulty);
    
    for level_array in seed_data_json["levels"].as_array().unwrap() {
        
        let level_data: &Map<String, Value> = level_array.as_object().unwrap();
        let map_grid = mapdata::level_data_to_edges(&level_data);
        // let elapsed = start.elapsed();
        // println!("Generate grid: {} ms", elapsed.as_millis());
        // mapdata::print_map_grid(&map_grid);
        // let start = Instant::now();
        image::generate_image(&map_grid, &level_data);
        // let elapsed = start.elapsed();
        // println!("Generated image for area {} in {}ms", level_data["id"], elapsed.as_millis());
    }
    let elapsed = start.elapsed();
    println!("Generated all images in {}ms", elapsed.as_millis());
}


fn get_seed_data(seed: &str, difficulty: &str) -> Value {
    let cached_seed_data_file = cache::cached_file_name(seed, difficulty);
    let seed_data_str: String = if Path::new(&cached_seed_data_file).exists() {
        println!("Reading cached map data from file {}", &cached_seed_data_file.to_str().unwrap());
        cache::read_cached_file(cached_seed_data_file)
    } else {
        println!("Generating fresh data for seed {} and difficulty {}", seed, difficulty);
        generate::generate_data(seed, difficulty)
    };
    let json = serde_json::from_str(&seed_data_str).unwrap();
    json
}
