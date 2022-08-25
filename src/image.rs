use std::path::PathBuf;
use tiny_skia::*;

use crate::data::{LevelData, Object};

pub struct ImageRequest {
    pub seed: u32,
    pub difficulty: u32,
    pub mapid: u32,
    pub d2lod: PathBuf,
    pub blachaexe: PathBuf,
    pub rotate: bool,
    pub scale: u8
}


pub fn generate_image(map_grid: &Vec<Vec<i32>>, level_data: &LevelData, file_name: PathBuf, scale: u8, rotate: bool) -> Pixmap {
    let height = map_grid.len() as f64;
    let width = map_grid[0].len() as f64;
    let scale = scale as u32;
    let mut pixmap: Pixmap;
    let transform: Transform;
    if rotate {
        // there has to be a better way, I hate this
        let angle: f64 = 45. * (std::f64::consts::PI / 180.);
        let rotated_width: f64 = ((width as f64) * angle.cos()).abs() + ((height as f64) * angle.sin()).abs();
        let rotated_height: f64 = ((width as f64) * angle.sin()).abs() + ((height as f64) * angle.cos()).abs();
        let x_translation = ((height as f64) * angle.sin()).abs();
        // println!("{} {} rotated {} {}", width, height, rotated_width, rotated_height);
        pixmap = Pixmap::new((rotated_width as u32) * scale, (rotated_height as u32) * scale).unwrap();
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
    draw_waypoints(&mut pixmap, &level_data, transform);
    draw_exits(&mut pixmap, &level_data, transform);
    draw_npcs(&mut pixmap, &level_data, transform);

    // save to disk
    pixmap.save_png(file_name).unwrap();
    pixmap
    
}

fn draw_waypoints(pixmap: &mut Pixmap, level_data: &LevelData, transform: Transform) {
    let mut yellow = Paint::default();
    yellow.set_color_rgba8(255, 255, 0, 255);
    for object in &level_data.objects {
        if object.name == "Waypoint" {
            let box_width = 12.;
            let box_height = 12.;
            let x = (object.x as f32) - (box_width / 2.);
            let y = (object.y as f32) - (box_height / 2.);
            let rect = Rect::from_xywh(x, y, box_width as f32, box_height as f32).unwrap();
            pixmap.fill_rect(rect, &yellow, transform, None);
        }
    }
}

fn draw_exits(pixmap: &mut Pixmap, level_data: &LevelData, transform: Transform) {
    let mut purple = Paint::default();
    purple.set_color_rgba8(255, 0, 255, 255);
    let mut green = Paint::default();
    green.set_color_rgba8(0, 255, 0, 255);

    for object in &level_data.objects {
        if object.object_type == "exit" {
            let box_width = 12.;
            let box_height = 12.;
            let x = (object.x as f32) - (box_width / 2.);
            let y = (object.y as f32) - (box_height / 2.);
            if object.is_good_exit == true && level_data.id == 46 {
                let rect = Rect::from_xywh(x, y, box_width as f32, box_height as f32).unwrap();
                pixmap.fill_rect(rect, &green, transform, None);
            } else {
                let rect = Rect::from_xywh(x, y, box_width as f32, box_height as f32).unwrap();
                pixmap.fill_rect(rect, &purple, transform, None);
            }
        }
    }
}

fn draw_npcs(pixmap: &mut Pixmap, level_data: &LevelData, transform: Transform) {
    let mut red = Paint::default();
    red.set_color_rgba8(255, 0, 0, 255);

    let box_size = 8.;

    for object in &level_data.objects {
        // summoner
        if level_data.id == 74 && object.id == 250 {
            draw_dot(pixmap, object, box_size, transform, &red);
        }
        // izual
        if level_data.id == 105 && object.object_type == "npc"  {
            draw_dot(pixmap, object, box_size, transform, &red);
        }
        // maggot lair 3
        if level_data.id == 64 && object.object_type == "npc"  {
            draw_dot(pixmap, object, box_size, transform, &red);
        }
        // radament
        if level_data.id == 49 && object.id == 744 {
            draw_dot(pixmap, object, box_size, transform, &red);
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
        }
    }
}

fn draw_dot(pixmap: &mut Pixmap, object: &Object, box_size: f32, transform: Transform, red: &Paint) {
    let x = (object.x as f32) - (box_size / 2.);
    let y = (object.y as f32) - (box_size / 2.);
    let rect = Rect::from_xywh(x, y, box_size as f32, box_size as f32).unwrap();
    pixmap.fill_rect(rect, &red, transform, None);
}