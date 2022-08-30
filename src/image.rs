use std::{path::PathBuf, io::Write, fs::File};
use tiny_skia::*;

use crate::{data::{LevelData, Object, get_level_name}, cache};

pub struct ImageRequest {
    pub seed: u32,
    pub difficulty: u32,
    pub mapid: u32,
    pub d2lod: PathBuf,
    pub blachaexe: PathBuf,
    pub rotate: bool,
    pub scale: u8,
}

pub struct MapImage {
    pub offsetx: u32,
    pub offsety: u32,
    pub image_width: u32,
    pub image_height: u32,
    pub rotated: bool,
    pub map_width: u32,
    pub map_height: u32,
    pub scale: u32,
    pub waypoints: String,
    pub exits: String,
    pub bosses: String,
    pub super_chests: String,
    pub shrines: String,
    pub wells: String,
    pub pixmap: Pixmap
}

impl MapImage {
    pub fn get_headers(&self) -> String {
        let mut headers: Vec<String> = vec![];
        headers.push(format!("offsetx: {}", self.offsetx));
        headers.push(format!("offsety: {}", self.offsety));
        headers.push(format!("mapwidth: {}", self.map_width));
        headers.push(format!("mapheight: {}", self.map_height));
        headers.push(format!("originalwidth: {}", self.image_width));
        headers.push(format!("originalheight: {}", self.image_height));
        headers.push(format!("prerotated: {}", self.rotated));
        headers.push(format!("scale: {}", self.scale));
        headers.push(format!("waypoints: {}", self.waypoints));
        headers.push(format!("exits: {}", self.exits));
        headers.push(format!("bosses: {}", self.bosses));
        headers.push(format!("super_chests: {}", self.super_chests));
        headers.push(format!("shrines: {}", self.shrines));
        headers.push(format!("wells: {}", self.wells));
        headers.push(format!("version: {}", "0.1.2"));
        headers.join("\n")
    }
}


pub fn generate_image(map_grid: &Vec<Vec<i32>>, level_data: &LevelData, image_request: &ImageRequest) -> MapImage {
    let mut height = map_grid.len() as f64;
    let mut width = map_grid[0].len() as f64;
    let scale = image_request.scale as u32;
    let mut pixmap: Pixmap;
    let transform: Transform;
    if image_request.rotate {
        // there has to be a better way, I hate this
        let angle: f64 = 45. * (std::f64::consts::PI / 180.);
        let x_translation = ((height as f64) * angle.sin()).abs();
        width = ((width as f64) * angle.cos()).abs() + ((height as f64) * angle.sin()).abs();
        height = ((width as f64) * angle.sin()).abs() + ((height as f64) * angle.cos()).abs();
        
        // println!("{} {} rotated {} {}", width, height, rotated_width, rotated_height);
        pixmap = Pixmap::new((width as u32) * scale, (height as u32) * scale).unwrap();
        transform = Transform::from_rotate(45.0).post_scale(scale as f32, scale as f32).post_translate((x_translation * scale as f64) as f32, 0.);
    } else {
        pixmap = Pixmap::new((width as u32) * scale, (height as u32) * scale).unwrap();
        transform = Transform::from_scale(scale as f32, scale as f32);
    }
    
    // draw the tiles
    let mut paint = Paint::default();
    paint.set_color_rgba8(170, 170, 170, 255);
    for (y, row) in map_grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell != &0 {
                let rect = Rect::from_xywh(x as f32, y as f32, 1., 1.).unwrap();
                pixmap.fill_rect(rect, &paint, transform, None);
            }
        }
    }

    let waypoints = draw_waypoints(&mut pixmap, &level_data, transform);
    let exits = draw_exits(&mut pixmap, &level_data, transform);
    let bosses = draw_npcs(&mut pixmap, &level_data, transform);
    let (super_chests, shrines, wells) = draw_objects(&mut pixmap, &level_data, transform);
    

    // save to disk
    let cached_image_file_name = cache::cached_image_file_name(&image_request.seed, &image_request.difficulty, &level_data.id);
    pixmap.save_png(cached_image_file_name.as_path()).unwrap();
    println!("Saved to {}", cached_image_file_name.to_string_lossy());
    let map_image = MapImage {
        offsetx: level_data.offset.x,
        offsety: level_data.offset.y,
        image_width: width as u32,
        image_height: height as u32,
        rotated: image_request.rotate,
        map_width: level_data.size.width,
        map_height: level_data.size.height,
        waypoints,
        exits,
        bosses,
        super_chests,
        shrines,
        wells,
        scale,
        pixmap
    };
    let cached_headers_file_name = cache::cached_header_file_name(&image_request.seed, &image_request.difficulty, &level_data.id);
    let headers = map_image.get_headers();
    let mut file = File::create(cached_headers_file_name).unwrap();
    file.write_all(headers.as_bytes()).expect("Error writing header file");
    map_image
    
}


fn draw_objects(pixmap: &mut Pixmap, level_data: &LevelData, transform: Transform) -> (String, String, String) {
    let mut blue = Paint::default();
    blue.set_color_rgba8(0, 0, 255, 255);
    let mut super_chests: Vec<String> = vec![];
    let mut shrines: Vec<String> = vec![];
    let mut well: Vec<String> = vec![];
    let box_width = 12.;
    let box_height = 12.;
    for object in &level_data.objects {
        if object.name == "chest" {
            let x = (object.x as f32) - (box_width / 2.);
            let y = (object.y as f32) - (box_height / 2.);
            if level_data.id == 84 || level_data.id == 85 || level_data.id == 91 {
                let rect = Rect::from_xywh(x, y, box_width as f32, box_height as f32).unwrap();
                pixmap.fill_rect(rect, &blue, transform, None);
                super_chests.push(format!("{},{}", object.x, object.y));
            }
            if object.id == 580 || object.id == 581 {
                let rect = Rect::from_xywh(x, y, box_width as f32, box_height as f32).unwrap();
                pixmap.fill_rect(rect, &blue, transform, None);
                super_chests.push(format!("{},{}", object.x, object.y));
            }
        }
        if object.name == "Shrine" {
            let x = (object.x as f32) - (box_width / 2.);
            let y = (object.y as f32) - (box_height / 2.);
            let rect = Rect::from_xywh(x, y, box_width as f32, box_height as f32).unwrap();
            pixmap.fill_rect(rect, &blue, transform, None);
            shrines.push(format!("{},{}", object.x, object.y));
        }
        if object.name == "Well" {
            let x = (object.x as f32) - (box_width / 2.);
            let y = (object.y as f32) - (box_height / 2.);
            let rect = Rect::from_xywh(x, y, box_width as f32, box_height as f32).unwrap();
            pixmap.fill_rect(rect, &blue, transform, None);
            well.push(format!("{},{}", object.x, object.y));
        }
    }
    (super_chests.join("|"), shrines.join("|"), well.join("|"))
}

fn draw_waypoints(pixmap: &mut Pixmap, level_data: &LevelData, transform: Transform) -> String {
    let mut yellow = Paint::default();
    yellow.set_color_rgba8(255, 255, 0, 255);
    let box_width = 12.;
    let box_height = 12.;
    for object in &level_data.objects {
        if object.name == "Waypoint" {
            let x = (object.x as f32) - (box_width / 2.);
            let y = (object.y as f32) - (box_height / 2.);
            let rect = Rect::from_xywh(x, y, box_width as f32, box_height as f32).unwrap();
            pixmap.fill_rect(rect, &yellow, transform, None);
            return format!("{},{}", object.x, object.y)
        }
    }
    String::new()
}

fn draw_exits(pixmap: &mut Pixmap, level_data: &LevelData, transform: Transform) -> String {
    let mut purple = Paint::default();
    purple.set_color_rgba8(255, 0, 255, 255);
    let mut green = Paint::default();
    green.set_color_rgba8(0, 255, 0, 255);
    let mut exit_header: Vec<String> = vec![];
    let box_width = 12.;
    let box_height = 12.;
    for object in &level_data.objects {
        if object.object_type == "exit" {
            let x = (object.x as f32) - (box_width / 2.);
            let y = (object.y as f32) - (box_height / 2.);
            if object.is_good_exit == true && level_data.id == 46 {
                let rect = Rect::from_xywh(x, y, box_width as f32, box_height as f32).unwrap();
                pixmap.fill_rect(rect, &green, transform, None);
                exit_header.push(format!("{},{},{},{}", object.id, get_level_name(object.id), object.x, object.y));
            } else {
                let rect = Rect::from_xywh(x, y, box_width as f32, box_height as f32).unwrap();
                pixmap.fill_rect(rect, &purple, transform, None);
                exit_header.push(format!("{},{},{},{}", object.id, get_level_name(object.id), object.x, object.y));
            }
        }
    }
    exit_header.join("|")
}

fn draw_npcs(pixmap: &mut Pixmap, level_data: &LevelData, transform: Transform) -> String {
    let mut red = Paint::default();
    red.set_color_rgba8(255, 0, 0, 255);

    let box_size = 8.;
    let mut boss_header: Vec<String> = vec![];
    for object in &level_data.objects {
        // summoner
        if level_data.id == 74 && object.id == 250 {
            draw_dot(pixmap, object, box_size, transform, &red);
            boss_header.push(format!("Summoner,{},{}", object.x, object.y));
        }
        // izual
        if level_data.id == 105 && object.object_type == "npc"  {
            draw_dot(pixmap, object, box_size, transform, &red);
            boss_header.push(format!("Izual,{},{}", object.x, object.y));
        }
        // maggot lair 3
        if level_data.id == 64 && object.object_type == "npc"  {
            draw_dot(pixmap, object, box_size, transform, &red);
            boss_header.push(format!("Maggot King,{},{}", object.x, object.y));
        }
        // radament
        if level_data.id == 49 && object.id == 744 {
            draw_dot(pixmap, object, box_size, transform, &red);
            boss_header.push(format!("Radament,{},{}", object.x, object.y));
        }
        // nihlithak is calculated by the preset NPC on the _opposite_ side of the map
        if level_data.id == 124 && object.object_type == "npc" {
            let mut x = object.x;
            let mut y = object.y;
            if x == 30 && y == 208 { // bottom right
                x = 392;
                y = 201;
            }
            if x == 206 && y == 32 { // bottom left
                x = 207;
                y = 386;
            }
            if x == 207 && y == 393 { // top right
                x = 207;
                y = 16;
            }
            if x == 388 && y == 216 { //top left
                x = 22;
                y = 201;
            }
            let nihl_x = (x as f32) - (box_size / 2.);
            let nihl_y = (y as f32) - (box_size / 2.);
            
            let rect = Rect::from_xywh(nihl_x, nihl_y, box_size as f32, box_size as f32).unwrap();
            pixmap.fill_rect(rect, &red, transform, None);
            boss_header.push(format!("Nihlithak,{},{}", object.x, object.y));
        }
    }
    boss_header.join("|")
}

fn draw_dot(pixmap: &mut Pixmap, object: &Object, box_size: f32, transform: Transform, red: &Paint) {
    let x = (object.x as f32) - (box_size / 2.);
    let y = (object.y as f32) - (box_size / 2.);
    let rect = Rect::from_xywh(x, y, box_size as f32, box_size as f32).unwrap();
    pixmap.fill_rect(rect, &red, transform, None);
}