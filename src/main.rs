use std::time::Instant;
use serde_json::{Value, Map};
use clap::ArgMatches;

mod generate;
mod cache;
mod mapdata;
mod image;
mod cli;

fn main() {
    let matches: ArgMatches  = cli::command_line_interface();
    
    
    let seed = matches.value_of_os("seed").unwrap().to_str().unwrap();
    let difficulty = matches.value_of_os("difficulty").unwrap().to_str().unwrap();
    let mapid = matches.value_of_os("mapid").unwrap().to_str().unwrap();


        // "E:/Dev/d2-mapserver-rust/mapgen/d2-mapgen.exe"
    // let seed: &str = "76546";
    // let difficulty: &str = "2";

    let start = Instant::now();
    // let blachaexe = "./mapgen/d2-mapgen.exe";

    let d2lod = matches.get_one::<std::path::PathBuf>("d2lod").unwrap();
    let blachaexe = matches.get_one::<std::path::PathBuf>("blachaexe").unwrap();
    println!("Using d2lod files stored in {}", d2lod.to_string_lossy());
    println!("Using blacha exe found in {}", blachaexe.to_string_lossy());

    let seed_data_json: Value = generate::get_seed_data(seed, difficulty, d2lod, blachaexe);
    
    for level_array in seed_data_json["levels"].as_array().unwrap() {
        
        let level_data: &Map<String, Value> = level_array.as_object().unwrap();
        if level_data["id"] == mapid {
            let map_grid = mapdata::level_data_to_edges(&level_data);
            // let elapsed = start.elapsed();
            // println!("Generate grid: {} ms", elapsed.as_millis());
            // mapdata::print_map_grid(&map_grid);
            // let start = Instant::now();
            let image_file_name = cache::cached_image_file_name(seed, difficulty, &level_data["id"]);
            image::generate_image(&map_grid, image_file_name);
        }
        // let elapsed = start.elapsed();
        // println!("Generated image for area {} in {}ms", level_data["id"], elapsed.as_millis());
    }
    let elapsed = start.elapsed();
    println!("Generated all images in {}ms", elapsed.as_millis());
}


