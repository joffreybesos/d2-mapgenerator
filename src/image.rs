use raqote::*;
use serde_json::{Value, Map};

pub fn generate_image(map_grid: &Vec<Vec<i32>>, level_data: &Map<String, Value>) {
    let height = map_grid.len();
    let width = map_grid[0].len();
    let mut dt = DrawTarget::new(width.try_into().unwrap(), height.try_into().unwrap());
    let src = &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 128, 128, 128));
    let opts = &DrawOptions::new();

    for (y, row) in map_grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell != &0 {
                dt.fill_rect(x as f32, y as f32, 1., 1., src, opts);
            }
        }
    }
    let file_name = format!("./images/{}.png", level_data["id"]);
    dt.write_png(file_name).unwrap();
}