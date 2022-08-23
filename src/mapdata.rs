use crate::data::LevelData;

pub fn level_data_to_walkable(level_data: &LevelData) -> Vec<Vec<i32>> {

    
    let lvl_width = level_data.size.width as usize;
    let lvl_height = level_data.size.height as usize;
    
    let mut map_grid: Vec<Vec<i32>> = vec![vec![0; lvl_width]; lvl_height];
    
    let mut y: usize = 0;
    for map_rows in &level_data.map {
        // each row
        let mut fill: bool = false;
        let mut x: usize = 0;

        let last_idx = map_rows.len().wrapping_sub(1);
        
        for (index, map_row) in map_rows.iter().enumerate() {
            fill = !fill;
            let width = *map_row as usize;
            if !fill {
                for num in 0..width {
                    map_grid[y][num + x] = 1;
                }
            }
            x += width;
            
            if index == last_idx && fill {
                // fix end of row
                let extra_width = lvl_width.wrapping_sub(x).wrapping_sub(1);
                for num in 0..extra_width {
                    map_grid[y][num + x] = 1;
                }
            }
        }
        y += 1;
    }

    map_grid
    
}


pub fn level_data_to_edges(level_data: &LevelData) -> Vec<Vec<i32>> {

    let map_grid = level_data_to_walkable(&level_data);
    
    let lvl_width = level_data.size.width as usize;
    let lvl_height = level_data.size.height as usize;

    let mut edge_map_grid: Vec<Vec<i32>> = vec![vec![0; lvl_width]; lvl_height];

    for (row, row_vec) in map_grid.iter().enumerate() {
        for (col, cell) in row_vec.iter().enumerate() {
            if cell == &0 {
                let border = check_surrounding_pixels(&map_grid, row , col, lvl_width, lvl_height);
                if border {
                    edge_map_grid[row][col] = 1;
                }
            } else if col == 0 || row == 0 {
                edge_map_grid[row][col] = 1;
            }
        }
    }
    edge_map_grid
    
}


fn check_surrounding_pixels(map_grid: &Vec<Vec<i32>>, irow: usize, icol: usize, width: usize, height: usize) -> bool {
    // above row
    if irow > 0 {
        if icol > 0 {
            if map_grid[irow-1][icol-1] == 1 {
                return true
            }
        }
        if map_grid[irow-1][icol] == 1 {
            return true
        }
        if icol < width.wrapping_sub(1) {
            if map_grid[irow-1][icol+1] == 1 {
                return true
            }
        }
    }
    //same row
    if icol > 0 {
        if map_grid[irow][icol-1] == 1 {
            return true
        }
    }
    if icol < width.wrapping_sub(1) {
        if map_grid[irow][icol+1] == 1 {
            return true
        }
    }

    // row beneath
    if irow < height.wrapping_sub(1) {
        if icol > 0 {
            if map_grid[irow + 1][icol-1] == 1 {
                return true
            }
        }
        if map_grid[irow][icol] == 1 {
            return true
        }
        if icol < width.wrapping_sub(1) {
            if map_grid[irow + 1][icol+1] == 1 {
                return true
            }
        }
    }

    false

}


#[allow(dead_code)]
pub fn print_map_grid(map_grid: &Vec<Vec<i32>>) {
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
