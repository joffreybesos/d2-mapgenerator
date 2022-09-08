use std::ops::Sub;

use crate::jsondata::LevelData;

pub struct MapGrid {
    tiles: Vec<Vec<i32>>,
    width: i16,
    height: i16
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub i16, pub i16);

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct Successor {
    pub pos: Pos,
    pub cost: u32,
}

impl MapGrid {
    pub fn get_successors(&self, position: &Pos) -> Vec<Successor> {
        let mut successors = Vec::new();
        for dx in -1i16..=1 {
            for dy in -1i16..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let new_position = Pos(position.0 + dx, position.1 + dy);
                if new_position.0 < 0 || new_position.0 >= self.width.into() || new_position.1 < 0 || new_position.1 >= self.height.into() {
                    continue;
                }
                let board_value = self.tiles[new_position.1 as usize][new_position.0 as usize];
                if board_value > 0 {
                    successors.push(Successor { pos: new_position, cost: board_value as u32});
                }
                
            }
        }
        successors
    }
}


pub fn level_data_to_walkable(level_data: &LevelData) -> MapGrid {
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
    MapGrid { tiles: map_grid, width: lvl_width as i16, height: lvl_height as i16 }
    
}


pub fn level_data_to_edges(map_grid: &MapGrid) -> Vec<Vec<i32>> {
    
    let lvl_width = map_grid.width as usize;
    let lvl_height = map_grid.height as usize;

    let mut edge_map_grid: Vec<Vec<i32>> = vec![vec![0; lvl_width]; lvl_height];

    for (row, row_vec) in map_grid.tiles.iter().enumerate() {
        for (col, cell) in row_vec.iter().enumerate() {
            if cell == &0 {
                let border = check_surrounding_pixels(&map_grid.tiles, row , col, lvl_width, lvl_height);
                if border {
                    edge_map_grid[row][col] = 1;
                }
            } else if col == 0 || row == 0 {
                edge_map_grid[row][col] = 1;
            } else if col == lvl_width.sub(1) || row == lvl_height.sub(1) {
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
