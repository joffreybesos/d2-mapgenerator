use clap::ArgMatches;
use colored::*;
use rayon::prelude::*;
use std::time::Instant;

use crate::data::SeedData;

mod cache;
mod cli;
mod data;
mod generate;
mod image;
mod mapdata;

fn main() {
    let matches: ArgMatches = cli::command_line_interface();

    // input validation happens in cli.rs
    let seed = matches.get_one::<u32>("seed").unwrap();
    let difficulty = matches.get_one::<u32>("difficulty").unwrap();
    let mapid = matches.get_one::<u32>("mapid").unwrap();
    let scale = *matches.get_one::<u8>("scale").unwrap();
    let rotate = matches.is_present("rotate");

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

    let seed_data_json: SeedData = generate::get_seed_data(seed, difficulty, d2lod, blachaexe);

    // generate levels in parallel
    seed_data_json.levels.par_iter().for_each(|level_data| {
        if level_data.id == *mapid || *mapid == 0 {
            let edge_start = Instant::now();
            let map_grid = mapdata::level_data_to_edges(&level_data);
            let edge_elapsed = edge_start.elapsed();

            // mapdata::print_map_grid(&map_grid);
            let image_start = Instant::now();
            let image_file_name = cache::cached_image_file_name(seed, difficulty, &level_data.id);
            image::generate_image(&map_grid, &level_data, image_file_name, scale, rotate);
            let image_elapsed = image_start.elapsed();
            println!("Generate {} grid in {}ms, image in {}ms", level_data.id, edge_elapsed.as_millis(), image_elapsed.as_millis());
        }
    });
    // let elapsed = start.elapsed();
    // println!("Generated image for area {} in {}ms", level_data["id"], elapsed.as_millis());

    let elapsed = start.elapsed();
    println!("{} {}{}", "Finished in".green(), elapsed.as_millis().to_string().bright_green(), "ms".green());
}
