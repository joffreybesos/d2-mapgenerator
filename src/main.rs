use actix_web::{HttpServer, App};
use clap::ArgMatches;
use colored::*;
use image::ImageRequest;
use rayon::prelude::*;
use std::{time::Instant};

use crate::{data::SeedData, server::{map_image}};

mod cache;
mod cli;
mod data;
mod blacha;
mod image;
mod mapdata;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches: ArgMatches = cli::command_line_interface();

    // input validation happens in cli.rs
    match matches.subcommand_name() {
        Some("generate") => {
            let generate_args = matches.subcommand_matches("generate").unwrap();
            generate_cli(generate_args)
        },
        Some("server") => {
            let server_args = matches.subcommand_matches("server").unwrap();
            let port = *server_args.get_one::<u16>("port").unwrap();
            println!("Running server on 127.0.0.1:{}", port);
            HttpServer::new(|| {
                App::new().service(map_image)
            })
            .bind(("127.0.0.1", port))?
            .run()
            .await
        }
        Some(_) => Ok(()),
        None => Ok(())
    }
}


fn generate_cli(generate_args: &ArgMatches) -> std::io::Result<()> {
    
    let seed = *generate_args.get_one::<u32>("seed").unwrap();
    let difficulty = *generate_args.get_one::<u32>("difficulty").unwrap();
    let mapid = *generate_args.get_one::<u32>("mapid").unwrap();
    let scale = *generate_args.get_one::<u8>("scale").unwrap();
    let rotate = generate_args.is_present("rotate");

    let d2lod = generate_args.get_one::<std::path::PathBuf>("d2lod").unwrap();
    if !std::path::Path::new(&d2lod).exists() {
        panic!("{} '{}'", "ERROR: Diablo 2 LoD path does not exist! Make sure you have the d2 lod 1.13c game files located in".red().bold(), &d2lod.to_string_lossy().red());
    }

    let blachaexe = generate_args.get_one::<std::path::PathBuf>("blachaexe").unwrap();
    if !std::path::Path::new(&blachaexe).exists() {
        panic!("{} '{}'", "ERROR: d2-mapgen.exe not in configured location, you have missing files".red().bold(), &blachaexe.to_string_lossy().red());
    }
    
    println!("{} '{}'", "Using Diablo 2 1.13c files stored in".green(), d2lod.to_string_lossy().bright_green());
    println!("{} '{}'", "Using blacha exe found in".green(), blachaexe.to_string_lossy().bright_green());

    let image_request = ImageRequest { seed, difficulty, mapid, d2lod: d2lod.to_path_buf(), blachaexe: blachaexe.to_path_buf(), rotate, scale };
    generate(image_request);
    Ok(())
}

pub fn generate(image_request: ImageRequest) {
    
    let start = Instant::now();
    let seed_data_json: SeedData = blacha::get_seed_data(&image_request.seed, &image_request.difficulty, &image_request.d2lod, &image_request.blachaexe);

    // generate levels in parallel
    seed_data_json.levels.par_iter().for_each(|level_data| {
        if level_data.id == image_request.mapid || image_request.mapid == 0 {
            let edge_start = Instant::now();
            let map_grid = mapdata::level_data_to_edges(&level_data);
            let edge_elapsed = edge_start.elapsed();

            // mapdata::print_map_grid(&map_grid);
            let image_start = Instant::now();
            let image_file_name = cache::cached_image_file_name(&image_request.seed, &image_request.difficulty, &level_data.id);
            image::generate_image(&map_grid, &level_data, image_file_name, image_request.scale, image_request.rotate);
            let image_elapsed = image_start.elapsed();
            println!("Generate {} grid in {}ms, image in {}ms", level_data.id, edge_elapsed.as_millis(), image_elapsed.as_millis());
        }
    });
    // let elapsed = start.elapsed();
    // println!("Generated image for area {} in {}ms", level_data["id"], elapsed.as_millis());

    let elapsed = start.elapsed();
    println!("{} {}{}", "Finished in".green(), elapsed.as_millis().to_string().bright_green(), "ms".green());
}