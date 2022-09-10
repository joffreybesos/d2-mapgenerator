use actix_web::{get, http::{StatusCode}, web, HttpResponse, Responder};
use colored::Colorize;
use serde::Deserialize;

use crate::{
    generate_single,
    image::{ImageRequest, MapImage},
};

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Params {
    rotate: Option<bool>,
    serverScale: Option<f32>,
    pathStart: Option<String>,
    pathEnd: Option<String>,
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
    let rotate = query.rotate.unwrap_or(false);
    let scale = query.serverScale.unwrap_or(3.0);
    
    let path_start = match &query.pathStart {
        Some(s) => s,
        None => "0",
    };
    let path_end = match &query.pathEnd {
        Some(s) => s,
        None => "0",
    };

    let d2lod = std::path::PathBuf::from("./game");
    if !std::path::Path::new(&d2lod).exists() {
        panic!("{} '{}'", "ERROR: Diablo 2 LoD path does not exist! Make sure you have the d2 lod 1.13c game files located in".red().bold(), &d2lod.to_string_lossy().red());
    }

    let blachaexe = std::path::PathBuf::from("./mapgen/d2-mapgen.exe");
    if !std::path::Path::new(&blachaexe).exists() {
        panic!("{} '{}'", "ERROR: d2-mapgen.exe not in configured location, you have missing files".red().bold(), blachaexe.to_string_lossy().red());
    }

    let image_request = ImageRequest { seed, difficulty, mapid, d2lod, blachaexe, rotate, scale, path_start: path_start.to_string(), path_end: path_end.to_string()};

    let map_image: Option<MapImage> = generate_single(image_request);
    match map_image {
        Some(p) => {
            // let response = format!("Generated {} {} {}", path_params.seed, path_params.difficulty, path_params.mapid);
            let pngdata = p.pixmap.encode_png().unwrap();
            
            HttpResponse::build(StatusCode::OK)
                .content_type("image/png")
                .insert_header(("offsetx", p.offsetx))
                .insert_header(("offsety", p.offsety))
                .insert_header(("mapwidth", p.map_width))
                .insert_header(("mapheight", p.map_height))
                .insert_header(("originalwidth", (p.map_width as f32 * p.scale) as u32))
                .insert_header(("originalheight", (p.map_height as f32 * p.scale) as u32 + 20))
                .insert_header(("prerotated", p.rotated.to_string()))
                .insert_header(("serverScale", p.scale.to_string()))
                .insert_header(("exits", p.exits))
                .insert_header(("bosses", p.bosses))
                .insert_header(("super_chests", p.super_chests))
                .insert_header(("shrines", p.shrines))
                .insert_header(("wells", p.wells))
                .insert_header(("version", "0.3.0"))
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
