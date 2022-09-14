use crate::jsondata::{LevelData, Object, SeedData};

pub fn get_walkable_exits(seed_data_json: &mut SeedData) {
    // add the map offset to each exit x and y value
    // I hate that i have to do this
    seed_data_json.levels.iter_mut().for_each(|o| {
        o.objects.iter_mut().for_each(|e| {
            if e.object_type == "exit" {
                e.x += o.offset.x;
                e.y += o.offset.y;
            }
        });
    });

    // calculate each exit by the tiles at the edge of each map
    let mut exits: Vec<Object> = vec![];
    for level_data in seed_data_json.levels.iter_mut().rev() {
        if valid_level(level_data.id) {
            get_valid_exits(level_data, &mut exits);
        }
    }

    // put new exits into the seed data
    let new_exits = exits.clone();
    for exit in new_exits.iter() {
        seed_data_json.levels.iter_mut().for_each(|l| {
            if l.id == exit.owned_level_id {
                l.objects.push(exit.clone());
            }
        });
    }

    seed_data_json.levels.iter_mut().for_each(|o| {
        o.objects.iter_mut().for_each(|e| {
            if e.object_type == "exit" {
                e.x -= o.offset.x;
                e.y -= o.offset.y;
            }
        });
    });
}

// detect the exit by the number of tiles at the edge of the map
// all 4 sides of the map are checked
fn get_valid_exits<'a>(
    level_data: &'a mut LevelData,
    exits: &'a mut Vec<Object>,
) -> &'a mut Vec<Object> {
    // if there's a stretch of this many tiles at the edge of the map
    let connecting_tiles = 2..=50; 
    // top exits
    if level_data.map[1].len() > 1 {
        let exit1 = level_data.map[1][1];
        if connecting_tiles.contains(&exit1) {
            let x: u32 = level_data.offset.x
                + level_data.map[1][0] as u32
                + (level_data.map[1][1] as f32 / 2.) as u32;
            let y: u32 = level_data.offset.y;
            exits.push(Object::new_exit(x, y, level_data.id, exits));
            // println!("{} Top exit 1 {} x{} y{}", level_data.id, exit1, x, y);
        }
    }
    if level_data.map[1].len() > 3 {
        let exit2 = level_data.map[1][3];
        if connecting_tiles.contains(&exit2) {
            let x: u32 = level_data.offset.x
                + (level_data.map[1][0] + level_data.map[1][1] + level_data.map[1][2]) as u32
                + (level_data.map[1][3] as f32 / 2.) as u32;
            let y: u32 = level_data.offset.y;
            exits.push(Object::new_exit(x, y, level_data.id, exits));
            // println!("{} Top exit 2 {} x{} y{}", level_data.id, exit2, x, y);
        }
    }

    //bottom exits
    if level_data.map[level_data.map.len() - 1].len() > 1 {
        let exit1 = level_data.map[level_data.map.len() - 1][1];
        if connecting_tiles.contains(&exit1) {
            let x: u32 = level_data.offset.x
                + (level_data.map[level_data.map.len() - 1][0]) as u32
                + (level_data.map[level_data.map.len() - 1][1] as f32 / 2.) as u32;
            let y: u32 = level_data.offset.y + level_data.size.height;
            exits.push(Object::new_exit(x, y, level_data.id, exits));
            // println!("{} Bottom exit 1 {} x{} y{}", level_data.id, exit1, x, y);
        }
    }
    if level_data.map[level_data.map.len() - 1].len() > 3 {
        let exit2 = level_data.map[level_data.map.len() - 1][3];
        if connecting_tiles.contains(&exit2) {
            let x: u32 = level_data.offset.x
                + (level_data.map[level_data.map.len() - 1][0]
                    + level_data.map[level_data.map.len() - 1][1]
                    + level_data.map[level_data.map.len() - 1][2]) as u32
                + (level_data.map[level_data.map.len() - 1][3] as f32 / 2.) as u32;
            let y: u32 = level_data.offset.y + level_data.size.height;
            exits.push(Object::new_exit(x, y, level_data.id, exits));
            // println!("{} Bottom exit 2 {} x{} y{}", level_data.id, exit2, x, y);
        }
        // for certain act 4 levels, remove some invalid exits
        if exit2 == 15 {
            exits.truncate(exits.len() - 2);
        }
    }

    // right exits
    let mut right_count = 0;
    let mut last_index = 0;
    for (index, map_cells) in level_data.map.iter().enumerate() {
        let fill: bool = map_cells.len() % 2 == 1;
        if fill {
            right_count += 1;
            last_index = index
        } else {
            if connecting_tiles.contains(&right_count) {
                let x: u32 = level_data.offset.x + level_data.size.width + 1;
                let y: u32 = level_data.offset.y + last_index as u32 - (right_count as f32 / 2.) as u32;
                exits.push(Object::new_exit(x, y, level_data.id, exits));
                // println!("{} Right exit {} x{} y{}", level_data.id, right_count, x, y);
            }
            right_count = 0;
        }
    }

    // left exits
    let mut left_count = 0;
    let mut last_index = 0;
    for (index, map_cells) in level_data.map.iter().enumerate() {
        if !map_cells.is_empty() && map_cells[0] == 0 {
            left_count += 1;
            last_index = index
        } else {
            if connecting_tiles.contains(&left_count) {
                let x: u32 = level_data.offset.x;
                let y: u32 = level_data.offset.y + last_index as u32 - (left_count as f32 / 2.) as u32;
                exits.push(Object::new_exit(x, y, level_data.id, exits));
                // println!("{} Left exit {} x{} y{}", level_data.id, left_count, x, y);
            }
            left_count = 0;
        }
    }
    exits
}

// check if a level is stitched
fn valid_level(level_id: u32) -> bool {
    match level_id {
        2 | 3 | 4 | 5 | 6 | 7 | 17 | 27 | 28 => true, // act 1
        41 | 42 | 43 | 44 | 45 => true,               // act 2
        79 | 80 | 81 | 82 | 83 => true,               // act 3
        103 | 104 | 105 | 106 | 107 | 108 => true,    // act 4
        _ => false,
    }
}

// who cares about enums?
pub fn get_attached_levels(level_id: u32) -> Vec<u32> {
    match level_id {
        1 => vec![2],
        2 => vec![1, 3],
        3 => vec![2, 4, 17],
        4 => vec![3],
        5 => vec![6],
        6 => vec![5, 7],
        7 => vec![6, 26],
        17 => vec![2],
        26 => vec![7, 27],
        27 => vec![26, 28],
        28 => vec![27],
        32 => vec![33],
        33 => vec![32],

        // act 2
        40 => vec![41],
        41 => vec![40, 42],
        42 => vec![41, 43],
        43 => vec![42, 44],
        44 => vec![43, 45],
        45 => vec![44],

        // act 3
        75 => vec![76],
        76 => vec![75, 77, 78],
        77 => vec![76, 78],
        78 => vec![76, 77, 79],
        79 => vec![78, 80],
        80 => vec![79, 81],
        81 => vec![80, 82],
        82 => vec![81, 83],
        83 => vec![82],

        //act 4
        103 => vec![104],
        104 => vec![103, 105],
        105 => vec![104, 106],
        106 => vec![105],
        107 => vec![108],
        108 => vec![107],

        //act 5
        109 => vec![110],
        110 => vec![109, 111],
        111 => vec![110, 112],
        112 => vec![111],
        _ => vec![2],
    }
}
