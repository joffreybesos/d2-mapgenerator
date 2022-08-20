use std::path::PathBuf;

use raqote::*;

pub fn generate_image(map_grid: &Vec<Vec<i32>>, file_name: PathBuf, scale: u8) {
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
    dt.write_png(file_name).unwrap();
}