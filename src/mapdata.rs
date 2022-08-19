use serde_json::{Value, Map};

pub fn level_data_to_walkable(level_data: &Map<String, Value>) -> Vec<Vec<i32>> {

    
    let lvl_width = level_data["size"]["width"].as_u64().unwrap() as usize;
    let lvl_height = level_data["size"]["height"].as_u64().unwrap() as usize;
    
    let mut map_grid: Vec<Vec<i32>> = vec![vec![0; lvl_width]; lvl_height];
    
    let mut y: usize = 0;
    for map_array in level_data["map"].as_array().unwrap() {
        // each row
        let mut fill: bool = false;
        let mut x: usize = 0;
        let map_rows = map_array.as_array().unwrap();

        let last_idx = map_rows.len().wrapping_sub(1);
        
        for (index, map_row) in map_rows.iter().enumerate() {
            fill = !fill;
            let width = map_row.as_u64().unwrap() as usize;
            if !fill {
                for num in 0..width {
                    map_grid[y][num + x] = 1;
                }
            }
            x += width;
            
            if index == last_idx && fill {
                // fix end of row
                let extra_width = lvl_width.wrapping_sub(x).wrapping_sub(1);
                dbg!(extra_width);
                for num in 0..extra_width {
                    map_grid[y][num + x] = 1;
                }
            }
        }
        y += 1;
    }

    map_grid
    
}


pub fn print_map_grid(map_grid: Vec<Vec<i32>>) {
    // output text
    for row in map_grid.iter() {
        let mut row_str = String::new();
        for cell in row.iter() {
            if cell == &0 {
                row_str.push_str(" ");
            } else {
                row_str.push_str("X");
            }
        }
        println!("{}", row_str);
    }
}