use actix_web::{get, web, HttpResponse, Responder};
use colored::Colorize;
use serde::Deserialize;

use crate::{image::ImageRequest, generate};


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
pub async fn map_image(path_params: web::Path<SeedValues>, query: web::Query<Params>) -> impl Responder {
    let seed = path_params.seed;
    
    let difficulty = path_params.difficulty;
    if !(0..=2).contains(&difficulty) {
        println!("Invalid difficulty, must be 0, 1, or 2, found {}", difficulty);
        return HttpResponse::BadRequest().body(format!("Invalid difficulty, must be 0, 1, or 2, found {}", difficulty))
    }
    
    let mapid = path_params.mapid;
    if !(1..=136).contains(&mapid) {
        println!("Invalid map number, must be 1-136, found {}", mapid);
        return HttpResponse::BadRequest().body(format!("Invalid map number, must be 1-136, found {}", mapid))
    }
    let rotate = match query.rotate {
        Some(r) => r == true,
        None => false
    };
    let scale = match query.scale {
        Some(s) => s,
        None => 2
    };

    let d2lod = std::path::PathBuf::from("./d2lod");
    if !std::path::Path::new(&d2lod).exists() {
        panic!("{} '{}'", "ERROR: Diablo 2 LoD path does not exist! Make sure you have the d2 lod 1.13c game files located in".red().bold(), &d2lod.to_string_lossy().red());
    }

    let blachaexe = std::path::PathBuf::from("./mapgen/d2-mapgen.exe");
    if !std::path::Path::new(&blachaexe).exists() {
        panic!("{} '{}'", "ERROR: d2-mapgen.exe not in configured location, you have missing files".red().bold(), blachaexe.to_string_lossy().red());
    }
    
    let image_request = ImageRequest { seed, difficulty, mapid, d2lod: d2lod, blachaexe: blachaexe.to_path_buf(), rotate, scale };
    generate(image_request);
    let response = format!("Generated {} {} {}", path_params.seed, path_params.difficulty, path_params.mapid);
    HttpResponse::Ok().body(response)
}