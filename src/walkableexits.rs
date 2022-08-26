use crate::data::{SeedData, Object, LevelData};


pub fn get_walkable_exits(seed_data_json: &mut SeedData) {
    // mark every exit
    for level_data in seed_data_json.levels.iter_mut() {
        if valid_level(level_data.id) {
            let mut exits: Vec<Object> = get_valid_exits(level_data);
            level_data.objects.append(&mut exits);
        }
    }

}

fn get_valid_exits(level_data: &mut LevelData) -> Vec<Object> {
    let mut exits: Vec<Object> = vec![];

    // top exits
    if level_data.map[1].len() > 1 {
        let exit1 = level_data.map[1][1];
        if exit1 > 2 && exit1 < 100 {
            let x = level_data.map[1][0] + (level_data.map[1][1] as f32 / 2.).round() as u64;
            let y = 0;
            exits.push(new_exit(x as u32, y));
            // println!("{} Top exit 1 {} x{} y{}", level_data.id, exit1, x, y);
        }
    }
    if level_data.map[1].len() > 3 {
        let exit2 = level_data.map[1][3];
        if exit2 > 2 && exit2 < 50 {
            let x = level_data.map[1][0] + level_data.map[1][1] + level_data.map[1][2] + (level_data.map[1][3] as f32 / 2.).round() as u64;
            let y = 0;
            exits.push(new_exit(x as u32, y));
            // println!("{} Top exit 2 {} x{} y{}", level_data.id, exit2, x, y);
        }
    }

    //bottom exits
    if level_data.map[level_data.map.len() - 1].len() > 1 {
        let exit1 = level_data.map[level_data.map.len() - 1][1];
        if exit1 > 2 && exit1 < 50 {
            let x = level_data.map[level_data.map.len() - 1][0] + (level_data.map[level_data.map.len() - 1][1] as f32 / 2.).round() as u64;
            let y = level_data.size.height;
            exits.push(new_exit(x as u32, y));
            // println!("{} Bottom exit 1 {} x{} y{}", level_data.id, exit1, x, y);
        }
    }
    if level_data.map[level_data.map.len() - 1].len() > 3 {
        let exit2 = level_data.map[level_data.map.len() - 1][3];
        if exit2 > 2 && exit2 < 50 {
            let x = level_data.map[level_data.map.len() - 1][0] + level_data.map[level_data.map.len() - 1][1] + level_data.map[level_data.map.len() - 1][2] + (level_data.map[level_data.map.len() - 1][3] as f32 / 2.).round() as u64;
            let y = level_data.size.height;
            exits.push(new_exit(x as u32, y));
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
        let mut fill: bool = false;
        for _ in map_cells.iter() {
            fill = !fill;
        }
        if fill {
            right_count += 1;
            last_index = index
        } else {
            if right_count > 1 && right_count < 50 {
                let x = level_data.size.width + 1;
                let y = last_index - (right_count as f32 / 2.).round() as usize;
                exits.push(new_exit(x as u32, y as u32));
                // println!("{} Right exit {} x{} y{}", level_data.id, right_count, x, y);
            }
            right_count = 0;
        }
    }

    // left exits
    let mut left_count = 0;
    let mut last_index = 0;
    for (index, map_cells) in level_data.map.iter().enumerate() {
        if map_cells.len() > 0 && map_cells[0] == 0 {
            left_count += 1;
            last_index = index
        } else {
            if left_count > 2 && left_count < 50 {
                let x = 0;
                let y = last_index - (left_count as f32 / 2.).round() as usize;
                exits.push(new_exit(x as u32, y as u32));
                // println!("{} Left exit {} x{} y{}", level_data.id, left_count, x, y);
            }
            left_count = 0;
        }
    }
    exits

}

fn new_exit(x: u32, y: u32) -> Object {
    Object {
        id: 0,
        object_type: "exit".to_owned(),
        x,
        y,
        name: "".to_owned(),
        op: 0,
        class: "".to_owned(),
        is_good_exit: false
    }
}

// check if a level is stitched
fn valid_level(level_id: u32) -> bool {
    match level_id {
        2 | 3 | 4 | 5 | 6 | 7 | 17 | 27 | 28 => true,   // act 1
        41 | 42 | 43 | 44 | 45 => true,                 // act 2
        79 | 80 | 81 | 82 | 83 => true,                 // act 3
        103 | 104 | 105 | 106 | 107 | 108 => true,      // act 4
        _ => false
    }
}