use std::path::PathBuf;
use raqote::*;

use crate::data::{LevelData, Object};

pub fn generate_image(map_grid: &Vec<Vec<i32>>, level_data: &LevelData, file_name: PathBuf, scale: u8, rotate: bool) {
    let height = map_grid.len() as f64;
    let width = map_grid[0].len() as f64;
    let scale = scale as usize;
    let mut dt: DrawTarget;
    if rotate {
        // there has to be a better way, I hate this
        let angle: f64 = 45. * (std::f64::consts::PI / 180.);
        let rotated_width: f64 = ((width as f64) * angle.cos()).abs() + ((height as f64) * angle.sin()).abs();
        let rotated_height: f64 = ((width as f64) * angle.sin()).abs() + ((height as f64) * angle.cos()).abs();
        let x_translation: f64 = ((height as f64) * angle.sin()).abs();
        // println!("{} {} rotated {} {}", width, height, rotated_width, rotated_height);
        dt = DrawTarget::new((rotated_width as i32) * scale as i32, (rotated_height as i32) * scale as i32);
        let translation = dt.get_transform()
            .then_rotate(euclid::Angle::degrees(45.0))
            .then_translate(euclid::vec2((x_translation * scale as f64) as f32, 0.));
        dt.set_transform(&translation);
    } else {
        dt = DrawTarget::new((width as i32) * scale as i32, (height as i32) * scale as i32);
    }
    
    let opts = &DrawOptions::new();
    let src = &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 128, 128, 128));
    //let src_trans = &Source::Solid(SolidSource::from_unpremultiplied_argb(96, 128, 128, 128));
    // dt.fill_rect(0.,0., (width as i32 * scale as i32) as f32,  (height as i32 * scale as i32) as f32, src_trans, opts);
    
    for (y, row) in map_grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell != &0 {
                dt.fill_rect((x * scale) as f32, (y * scale) as f32, scale as f32, scale as f32, src, opts);
            }
        }
    }
    draw_waypoints(&mut dt, &level_data, scale);
    draw_exits(&mut dt, &level_data, scale);
    draw_npcs(&mut dt, &level_data, scale);
    dt.write_png(file_name).unwrap();
}

fn draw_waypoints(dt: &mut DrawTarget, level_data: &LevelData, scale: usize) {
    let src_yellow = &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 255, 255, 0));
    
    let opts = &DrawOptions::new();

    for object in &level_data.objects {
        if object.name == "Waypoint" {
            let box_width = 12. * scale as f32;
            let box_height = 12. * scale as f32;
            let x = ((object.x * scale) as f32) - (box_width / 2.);
            let y = ((object.y * scale) as f32) - (box_height / 2.);
            dt.fill_rect(x as f32, y as f32, box_width, box_height, src_yellow, opts);
        }
    }
}

fn draw_exits(dt: &mut DrawTarget, level_data: &LevelData, scale: usize) {
    let src_purple = &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 255, 0, 255));
    let src_green = &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 0, 255, 0));
    let opts = &DrawOptions::new();

    for object in &level_data.objects {
        if object.object_type == "exit" {
            let box_width = 12. * scale as f32;
            let box_height = 12. * scale as f32;
            let x = ((object.x * scale) as f32) - (box_width / 2.);
            let y = ((object.y * scale) as f32) - (box_height / 2.);
            if object.is_good_exit == true && level_data.id == 46 {
                dt.fill_rect(x as f32, y as f32, box_width, box_height, src_green, opts);
            } else {
                dt.fill_rect(x as f32, y as f32, box_width, box_height, src_purple, opts);
            }
        }
    }
}

fn draw_npcs(dt: &mut DrawTarget, level_data: &LevelData, scale: usize) {
    let src_red = &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 255, 0, 0));
    let box_size = 8. * scale as f32;

    for object in &level_data.objects {
        // summoner
        if level_data.id == 74 && object.id == 250 {
            draw_dot(dt, object, box_size, scale, src_red);
        }
        // izual
        if level_data.id == 105 && object.object_type == "npc"  {
            draw_dot(dt, object, box_size, scale, src_red);
        }
        // maggot lair 3
        if level_data.id == 64 && object.object_type == "npc"  {
            draw_dot(dt, object, box_size, scale, src_red);
        }
        // radament
        if level_data.id == 49 && object.id == 744 {
            draw_dot(dt, object, box_size, scale, src_red);
        }
        // nihlithak
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
            let nihl_x = ((x * scale) as f32) - (box_size / 2.);
            let nihl_y = ((y * scale) as f32) - (box_size / 2.);
            
            let opts = &DrawOptions::new();
            dt.fill_rect(nihl_x as f32, nihl_y as f32, box_size, box_size, src_red, opts);
        }
    }
}

fn draw_dot(dt: &mut DrawTarget, object: &Object, box_size: f32, scale: usize, src: &Source) {
    let opts = &DrawOptions::new();
    let x = ((object.x * scale) as f32) - (box_size / 2.);
    let y = ((object.y * scale) as f32) - (box_size / 2.);
    dt.fill_rect(x as f32, y as f32, box_size, box_size, src, opts);
}