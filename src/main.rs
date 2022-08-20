use std::time::Instant;
use serde_json::{Value, Map};
use clap::ArgMatches;
use colored::*;

mod generate;
mod cache;
mod mapdata;
mod image;
mod cli;

fn main() {
    let matches: ArgMatches  = cli::command_line_interface();
    
    // input validation happens in cli.rs
    let seed = matches.get_one::<u32>("seed").unwrap();
    let difficulty = matches.get_one::<u32>("difficulty").unwrap();
    let mapid = matches.get_one::<u32>("mapid").unwrap();
    let scale = *matches.get_one::<u8>("scale").unwrap();


    let d2lod = matches.get_one::<std::path::PathBuf>("d2lod").unwrap();
    if !std::path::Path::new(&d2lod).exists() {
        panic!("{} '{}'", "ERROR: Diablo 2 LoD path does not exist! Make sure you have the d2 lod 1.13c game files located in".red().bold(), &d2lod.to_string_lossy().red());
    }

    let blachaexe = matches.get_one::<std::path::PathBuf>("blachaexe").unwrap();
    if !std::path::Path::new(&blachaexe).exists() {
        panic!("{} '{}'", "ERROR: d2-mapgen.exe not in configured location, you have missing files".red().bold(), &blachaexe.to_string_lossy().red());
    }

    let start = Instant::now();
    println!("{} '{}'", "Using Diablo 2 1.13c files stored in".green(), d2lod.to_string_lossy().bright_green());
    println!("{} '{}'", "Using blacha exe found in".green(), blachaexe.to_string_lossy().bright_green());

    let seed_data_json: Value = generate::get_seed_data(seed, difficulty, d2lod, blachaexe);
    
    for level_array in seed_data_json["levels"].as_array().unwrap() {
        
        let level_data: &Map<String, Value> = level_array.as_object().unwrap();

        if level_data["id"].as_u64().unwrap() == *mapid as u64 || *mapid == 0 {
            let map_grid = mapdata::level_data_to_edges(&level_data);
            // let elapsed = start.elapsed();
            // println!("Generate grid: {} ms", elapsed.as_millis());
            // mapdata::print_map_grid(&map_grid);
            // let start = Instant::now();
            let image_file_name = cache::cached_image_file_name(seed, difficulty, &level_data["id"]);
            image::generate_image(&map_grid, image_file_name, scale);
        }
        // let elapsed = start.elapsed();
        // println!("Generated image for area {} in {}ms", level_data["id"], elapsed.as_millis());
    }
    let elapsed = start.elapsed();
    println!("{} {}{}", "Finished in".green(), elapsed.as_millis().to_string().bright_green(), "ms".green());
}


