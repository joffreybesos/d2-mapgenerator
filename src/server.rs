use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};
use colored::Colorize;
use serde::Deserialize;

use crate::{
    generate_single,
    image::{ImageRequest, MapImage}, cache,
};

#[derive(Debug, Deserialize)]
pub struct Params {
    rotate: Option<bool>,
    scale: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct SeedValues {
    seed: u32,
    difficulty: u32,
    mapid: u32,
}

#[get("/v1/map/{seed}/{difficulty}/{mapid}/image")]
pub async fn get_map_image(
    path_params: web::Path<SeedValues>,
    query: web::Query<Params>,
) -> impl Responder {
    let seed = path_params.seed;

    let difficulty = path_params.difficulty;
    if !(0..=2).contains(&difficulty) {
        println!(
            "Invalid difficulty, must be 0, 1, or 2, found {}",
            difficulty
        );
        return HttpResponse::BadRequest().body(format!(
            "Invalid difficulty, must be 0, 1, or 2, found {}",
            difficulty
        ));
    }

    let mapid = path_params.mapid;
    if !(1..=136).contains(&mapid) {
        println!("Invalid map number, must be 1-136, found {}", mapid);
        return HttpResponse::BadRequest().body(format!(
            "Invalid map number, must be 1-136, found {}",
            mapid
        ));
    }
    let rotate = match query.rotate {
        Some(r) => r == true,
        None => false,
    };
    let scale = match query.scale {
        Some(s) => s,
        None => 2,
    };

    let d2lod = std::path::PathBuf::from("./d2lod");
    if !std::path::Path::new(&d2lod).exists() {
        panic!("{} '{}'", "ERROR: Diablo 2 LoD path does not exist! Make sure you have the d2 lod 1.13c game files located in".red().bold(), &d2lod.to_string_lossy().red());
    }

    let blachaexe = std::path::PathBuf::from("./mapgen/d2-mapgen.exe");
    if !std::path::Path::new(&blachaexe).exists() {
        panic!("{} '{}'", "ERROR: d2-mapgen.exe not in configured location, you have missing files".red().bold(), blachaexe.to_string_lossy().red());
    }

    let image_request = ImageRequest { seed, difficulty, mapid, d2lod: d2lod.to_path_buf(), blachaexe: blachaexe.to_path_buf(), rotate, scale };

    let cached_image_file_name = cache::cached_image_file_name(&seed, &difficulty, &mapid);
    if std::path::Path::new(&cached_image_file_name).exists() {
        println!("Reading image from cache {}", cached_image_file_name.to_string_lossy());
        let image_content = web::block(|| std::fs::read(cached_image_file_name))
            .await
            .unwrap()
            .unwrap();

        HttpResponse::build(StatusCode::OK)
            .content_type("image/png")
            .body(image_content)
    } else {
        let map_image: Option<MapImage> = generate_single(image_request);
        match map_image {
            Some(p) => {
                // let response = format!("Generated {} {} {}", path_params.seed, path_params.difficulty, path_params.mapid);
                let pngdata = p.pixmap.encode_png().unwrap();
                HttpResponse::build(StatusCode::OK)
                    .content_type("image/png")
                    .body(pngdata)
            }
            None => {
                let response = format!(
                    "Error generating image {} {} {}",
                    path_params.seed, path_params.difficulty, path_params.mapid
                );
                HttpResponse::InternalServerError().body(response)
            }
        }
    }
}
