use std::path::PathBuf;

use raqote::*;
use serde_json::{Value, Map};

pub fn generate_image(map_grid: &Vec<Vec<i32>>, level_data: &Map<String, Value>, file_name: PathBuf, scale: u8) {
    let height = map_grid.len() as i32;
    let width = map_grid[0].len() as i32;
    let scale = scale as usize;
    let mut dt = DrawTarget::new(width * scale as i32, height * scale as i32);
    let src = &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 128, 128, 128));
    let opts = &DrawOptions::new();

    for (y, row) in map_grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell != &0 {
                dt.fill_rect((x * scale) as f32, (y * scale) as f32, scale as f32, scale as f32, src, opts);
            }
        }
    }
    draw_exits(&mut dt, &level_data, scale);
    draw_npcs(&mut dt, &level_data, scale);
    dt.write_png(file_name).unwrap();
}


fn draw_exits(dt: &mut DrawTarget, level_data: &Map<String, Value>, scale: usize) {
    let src_purple = &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 255, 0, 255));
    let src_green = &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 0, 255, 0));
    let opts = &DrawOptions::new();

    for object_array in level_data["objects"].as_array().unwrap() {
        if object_array["type"] == "exit" {
            let box_width = 15. * scale as f32;
            let box_height = 15. * scale as f32;
            let x = ((object_array["x"].as_u64().unwrap() * scale as u64) as f32) - (box_width / 2.);
            let y = ((object_array["y"].as_u64().unwrap() * scale as u64) as f32) - (box_height / 2.);
            if object_array["isGoodExit"] == true && level_data["id"] == 46 {
                dt.fill_rect(x as f32, y as f32, box_width, box_height, src_green, opts);
            } else {
                dt.fill_rect(x as f32, y as f32, box_width, box_height, src_purple, opts);
            }
        }
    }
}

fn draw_npcs(dt: &mut DrawTarget, level_data: &Map<String, Value>, scale: usize) {
    let src_red = &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 255, 0, 0));
    let box_size = 10. * scale as f32;

    for object in level_data["objects"].as_array().unwrap() {
        // summoner
        if level_data["id"] == 74 && object["id"] == 250 {
            draw_dot(dt, object, box_size, scale, src_red);
        }
        // izual
        if level_data["id"] == 105 && object["type"] == "npc"  {
            draw_dot(dt, object, box_size, scale, src_red);
        }
        // maggot lair 3
        if level_data["id"] == 64 && object["type"] == "npc"  {
            draw_dot(dt, object, box_size, scale, src_red);
        }
        // radament
        if level_data["id"] == 49 && object["id"] == 744 {
            draw_dot(dt, object, box_size, scale, src_red);
        }
        // nihlithak
        if level_data["id"] == 124 && object["type"] == "npc" {
            let mut x = object["x"].as_u64().unwrap();
            let mut y = object["y"].as_u64().unwrap();
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
            let nihl_x = ((x * scale as u64) as f32) - (box_size / 2.);
            let nihl_y = ((y * scale as u64) as f32) - (box_size / 2.);
            
            let opts = &DrawOptions::new();
            dt.fill_rect(nihl_x as f32, nihl_y as f32, box_size, box_size, src_red, opts);
        }
    }
}

fn draw_dot(dt: &mut DrawTarget, object: &Value, box_size: f32, scale: usize, src: &Source) {
    let opts = &DrawOptions::new();
    let x = ((object["x"].as_u64().unwrap() * scale as u64) as f32) - (box_size / 2.);
    let y = ((object["y"].as_u64().unwrap() * scale as u64) as f32) - (box_size / 2.);
    dt.fill_rect(x as f32, y as f32, box_size, box_size, src, opts);
}