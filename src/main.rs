use actix_web::{App, HttpServer};
use clap::ArgMatches;
use colored::*;
use image::ImageRequest;
use rayon::prelude::*;
use std::time::Instant;

use crate::{jsondata::SeedData, image::MapImage, server::get_map_image, mapgrid::{MapGrid, Pos}};

mod blacha;
mod cache;
mod cli;
mod jsondata;
mod image;
mod mapgrid;
mod server;
mod walkableexits;
mod pathfinding;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches: ArgMatches = cli::command_line_interface();

    // input validation happens in cli.rs
    match matches.subcommand_name() {
        Some("generate") => {
            let generate_args = matches.subcommand_matches("generate").unwrap();
            generate_cli(generate_args)
        }
        Some("server") => {
            let server_args = matches.subcommand_matches("server").unwrap();
            let port = *server_args.get_one::<u16>("port").unwrap();
            println!(
                "{}{}",
                "Started rust map server on http://localhost:".blue(),
                port.to_string().blue()
            );
            HttpServer::new(|| App::new().service(get_map_image))
                .bind(("127.0.0.1", port))?
                .run()
                .await
        }
        Some(_) => Ok(()),
        None => Ok(()),
    }
}

fn generate_cli(generate_args: &ArgMatches) -> std::io::Result<()> {
    let seed = *generate_args.get_one::<u32>("seed").unwrap();
    let difficulty = *generate_args.get_one::<u32>("difficulty").unwrap();
    let mapid = *generate_args.get_one::<u32>("mapid").unwrap();
    let scale = *generate_args.get_one::<u8>("scale").unwrap();
    let rotate = generate_args.is_present("rotate");

    let path_start = generate_args.get_one::<String>("pathstart").unwrap().clone();
    let path_end = generate_args.get_one::<String>("pathend").unwrap().clone();

    let d2lod = generate_args
        .get_one::<std::path::PathBuf>("d2lod")
        .unwrap();
    if !std::path::Path::new(&d2lod).exists() {
        panic!("{} '{}'", "ERROR: Diablo 2 LoD path does not exist! Make sure you have the d2 lod 1.13c game files located in".red().bold(), &d2lod.to_string_lossy().red());
    }

    let blachaexe = generate_args
        .get_one::<std::path::PathBuf>("blachaexe")
        .unwrap();
    if !std::path::Path::new(&blachaexe).exists() {
        panic!(
            "{} '{}'",
            "ERROR: d2-mapgen.exe not in configured location, you have missing files"
                .red()
                .bold(),
            &blachaexe.to_string_lossy().red()
        );
    }

    println!(
        "{} '{}'",
        "Using Diablo 2 1.13c files stored in".green(),
        d2lod.to_string_lossy().bright_green()
    );
    println!(
        "{} '{}'",
        "Using blacha exe found in".green(),
        blachaexe.to_string_lossy().bright_green()
    );

    let image_request = ImageRequest {
        seed,
        difficulty,
        mapid,
        d2lod: d2lod.to_path_buf(),
        blachaexe: blachaexe.to_path_buf(),
        rotate,
        scale,
        path_start,
        path_end
    };
    if mapid == 0 {
        generate_all(image_request);
    } else {
        generate_single(image_request);
    }
    Ok(())
}

pub fn generate_single(image_request: ImageRequest) -> Option<MapImage> {
    let start = Instant::now();
    let mut seed_data_json: SeedData = blacha::get_seed_data(
        &image_request.seed,
        &image_request.difficulty,
        &image_request.d2lod,
        &image_request.blachaexe,
    );
    walkableexits::get_walkable_exits(&mut seed_data_json);

    // generate levels in parallel
    if let Some(level_data) = seed_data_json
        .levels
        .iter()
        .find(|a| a.id == image_request.mapid)
    {
        let edge_start = Instant::now();
        let map_grid: MapGrid = mapgrid::level_data_to_walkable(&level_data);
        let path_data: Vec<Pos> = pathfinding::get_path_data(&level_data, &map_grid, &image_request.path_start, &image_request.path_end);
        let edge_grid = mapgrid::level_data_to_edges(&map_grid);
        let edge_elapsed = edge_start.elapsed();

        let image_start = Instant::now();
        let map_image: MapImage = image::generate_image(&edge_grid, &level_data, &image_request, path_data);
        let image_elapsed = image_start.elapsed();
        println!(
            "Generated single map {}, created grid in {}ms, image in {}ms",
            level_data.id,
            edge_elapsed.as_millis(),
            image_elapsed.as_millis()
        );

        let elapsed = start.elapsed();
        println!(
            "{} {}{}",
            "Finished in".green(),
            elapsed.as_millis().to_string().bright_green(),
            "ms".green()
        );

        Some(map_image)
    } else {
        println!(
            "{} {}",
            "Error generating map".red(),
            image_request.mapid.to_string().red()
        );
        None
    }
}

pub fn generate_all(image_request: ImageRequest) {
    let start = Instant::now();
    let mut seed_data_json: SeedData = blacha::get_seed_data(
        &image_request.seed,
        &image_request.difficulty,
        &image_request.d2lod,
        &image_request.blachaexe,
    );
    walkableexits::get_walkable_exits(&mut seed_data_json);

    // generate levels in parallel
    seed_data_json.levels.par_iter().for_each(|level_data| {
        if level_data.id == image_request.mapid || image_request.mapid == 0 {
            let edge_start = Instant::now();
            let map_grid: MapGrid = mapgrid::level_data_to_walkable(&level_data);
            let edge_grid = mapgrid::level_data_to_edges(&map_grid);
            let edge_elapsed = edge_start.elapsed();

            let image_start = Instant::now();
            let path_data = vec![];  // no pathing for 'all' maps
            image::generate_image(&edge_grid, &level_data, &image_request, path_data);
            let image_elapsed = image_start.elapsed();
            println!(
                "Generated map {}, created grid in {}ms, image in {}ms",
                level_data.id,
                edge_elapsed.as_millis(),
                image_elapsed.as_millis()
            );
        }
    });
    let elapsed = start.elapsed();
    println!(
        "{} {}{}",
        "Finished in".green(),
        elapsed.as_millis().to_string().bright_green(),
        "ms".green()
    );
}
