use serde_json::{Value, Map, json};

// check if a level is stitched
fn valid_level(level_id: u64) -> bool {
    match level_id {
        2 | 3 | 4 | 5 | 6 | 7 | 17 | 28 => true,   // act 1
        41 | 42 | 43 | 44 | 45 => true,            // act 2
        79 | 80 | 81 | 82 => true,                 // act 3
        104 | 105 | 106 | 107 | 108 => true,       // act 4
        _ => false
    }
}

pub fn add_walkable_exits(seed_data_json: &Value) {
    for level_array in seed_data_json["levels"].as_array().unwrap() {
        let level_data: &Map<String, Value> = level_array.as_object().unwrap();
        let level_id = level_data["id"].as_u64().unwrap();
        if valid_level(level_id) {
            println!("Found level {}", level_id);
            top_exit(seed_data_json, level_data);
            bottom_exit(seed_data_json, level_data);
            right_exit(seed_data_json, level_data);
            left_exit(seed_data_json, level_data);
        }
    }

    
    //   seedData = updateUnlabelledExit(seedData, 42, 43)
    //   seedData = updateUnlabelledExit(seedData, 44, 43)
    
    //   seedData = updateUnlabelledExit(seedData, 80, 81)
    //   seedData = updateUnlabelledExit(seedData, 81, 82)
    
    //   return seedData;

}

fn top_exit(seed_data_json: &Value, level_data: &Map<String, Value>) {
    let level_id = level_data["id"].as_u64().unwrap();
    if let Some(tile_count) = level_data["map"][1][1].as_u64() {
        if valid_connector_width(level_id, tile_count)  {
            let exitx: u64 = level_data["map"][1][0].as_u64().unwrap() + (level_data["map"][1][1].as_u64().unwrap() / 2);
            let exity: u64 = 1;
            // println!("top {} {}", exitx, exity);
            add_new_exit(seed_data_json, level_data, exitx, exity);
        }
    }

    // for cases where there are 2 exits along the top of the map
    if let Some(tile_count) = level_data["map"][1][3].as_u64() {
        if valid_connector_width(level_id, tile_count) {
            let exitx: u64 = level_data["map"][1][0].as_u64().unwrap() + level_data["map"][1][1].as_u64().unwrap() + level_data["map"][1][2].as_u64().unwrap() + (level_data["map"][1][3].as_u64().unwrap() / 2);
            let exity: u64 = 1;
            // println!("top2 {} {}", exitx, exity);
            add_new_exit(seed_data_json, level_data, exitx, exity);
        }
    }
}

fn bottom_exit(seed_data_json: &Value, level_data: &Map<String, Value>) {
    let level_id = level_data["id"].as_u64().unwrap();
    let map_len = level_data["map"].as_array().unwrap().len().wrapping_sub(1);
    if let Some(tile_count) = level_data["map"][map_len][1].as_u64() {
        if valid_connector_width(level_id, tile_count)  {
            let exitx: u64 = level_data["map"][map_len][0].as_u64().unwrap() + (level_data["map"][map_len][1].as_u64().unwrap() / 2);
            let exity: u64 = level_data["size"]["height"].as_u64().unwrap();
            // println!("bottom {} {}", exitx, exity);
            add_new_exit(seed_data_json, level_data, exitx, exity);
        }
    }

    // for cases where there are 2 exits along the top of the map
    if let Some(tile_count) = level_data["map"][map_len][3].as_u64() {
        if valid_connector_width(level_id, tile_count) {
            let exitx: u64 = level_data["map"][map_len][0].as_u64().unwrap() + level_data["map"][map_len][1].as_u64().unwrap() + level_data["map"][map_len][2].as_u64().unwrap() + (level_data["map"][map_len][3].as_u64().unwrap() / 2);
            let exity: u64 = level_data["size"]["height"].as_u64().unwrap();
            // println!("bottom2 {} {}", exitx, exity);
            add_new_exit(seed_data_json, level_data, exitx, exity);
        }
    }
}

fn right_exit(seed_data_json: &Value, level_data: &Map<String, Value>) {
    let mut right_count = 0;
    let mut last_index = 0;
    let level_id = level_data["id"].as_u64().unwrap();
    
    for (index, map_array) in level_data["map"].as_array().unwrap().iter().enumerate() {
        let mut fill: bool = false;
        let map_cells = map_array.as_array().unwrap();
        for _ in map_cells.iter() {
            fill = !fill;
        }
        if fill {
            right_count += 1;
            last_index = index
        } else {
            if right_count > 0 {
                if valid_connector_width(level_id, right_count) {
                    let exitx: u64 = level_data["size"]["width"].as_u64().unwrap();
                    let exity: u64 = (last_index - (right_count as usize / 2)).try_into().unwrap();
                    // println!("right {} {}", exitx, exity);
                    add_new_exit(seed_data_json, level_data, exitx, exity);
                }
            }
            right_count = 0;
        }
    }
}

fn left_exit(seed_data_json: &Value, level_data: &Map<String, Value>) {
    let mut left_count = 0;
    let mut last_index = 0;
    let level_id = level_data["id"].as_u64().unwrap();
    
    for (index, map_array) in level_data["map"].as_array().unwrap().iter().enumerate() {
        if let Some(map_cells) = map_array.as_array() {
            if map_cells.len() > 0 && map_cells[0] == 0 {
                left_count += 1;
                last_index = index
            } else {
                if left_count > 0 {
                    if valid_connector_width(level_id, left_count) {
                        let exitx: u64 = 0;
                        let exity: u64 = (last_index - (left_count as usize / 2)).try_into().unwrap();
                        // println!("left {} {}", exitx, exity);
                        add_new_exit(seed_data_json, level_data, exitx, exity);
                    }
                }
                left_count = 0;
            }
        }
    }
}

fn add_new_exit(seed_data_json: &Value, level_data: &Map<String, Value>, exitx: u64, exity: u64) {
    // let new_exit = format!(r#"{{ "id": 0, "type": "exit", "x": {}, "y": {} }}"#, exitx, exity);
    let new_exit = json!({
        "id": 0,
        "type": "exit",
        "x": exitx,
        "y": exity
    });
    let level_id: u64 = level_data["id"].as_u64().unwrap();
    let new_exit_id: u64;
    match level_id {
        2 => new_exit_id = 3,
        4 => new_exit_id = 3,
        5 => new_exit_id = 6,
        7 => new_exit_id = 6,
        17 => new_exit_id = 3,
        28 => new_exit_id = 27,
        41 => new_exit_id = 42,
        45 => new_exit_id = 44,
        79 => new_exit_id = 80,
        104 => new_exit_id = 105,
        106 => new_exit_id = 105,
        107 => new_exit_id = 108,
        _ => new_exit_id = 0,
    };
    if new_exit_id > 0 {
        let other_exit = convert_exit(seed_data_json, level_data, level_id, exitx, exity);
        add_or_update_exit(seed_data_json, &new_exit_id, &other_exit);
    }
    add_or_update_exit(seed_data_json, &level_id, &new_exit);
}

fn convert_exit(seed_data_json: &Value, level_data: &Map<String, Value>, level_id: u64, exitx: u64, exity: u64) -> Value {
    let pointer = format!("/levels/{}", level_id - 1);
    let pointer_str: &str = pointer.as_str();
    let other_level_data = seed_data_json.pointer(pointer_str).unwrap();
    let absolute_x = level_data["offset"]["x"].as_u64().unwrap() + exitx;
    let absolute_y = level_data["offset"]["y"].as_u64().unwrap() + exity;
    let new_x = absolute_x - &other_level_data["offset"]["x"].as_u64().unwrap();
    let new_y = absolute_y - &other_level_data["offset"]["y"].as_u64().unwrap();
    let other_level_id: u64 = other_level_data["id"].as_u64().unwrap();
    json!({
        "id": other_level_id,
        "type": "exit",
        "x": new_x,
        "y": new_y
    })
    // format!(r#"{{"id": {}, "type": "exit", "x": {}, "y": {}}}"#, other_level_id, new_x, new_y)
}

fn add_or_update_exit(seed_data_json: &Value, level_id: &u64, new_exit_json: &Value) {
    
    let pointer = format!("/levels/{}/objects", level_id - 1);
    let pointer_str: &str = pointer.as_str();
    let objects = seed_data_json.pointer(pointer_str).unwrap();
    for object in objects.as_array().unwrap() {
        if object["type"] == "exit" {
            let x = object["x"].as_u64().unwrap();
            let y = object["y"].as_u64().unwrap();
            
            let new_x = new_exit_json["x"].as_u64().unwrap();
            let new_y = new_exit_json["y"].as_u64().unwrap();
            
            if x.abs_diff(new_x) < 3 && y.abs_diff(new_y) < 3 {
                if new_exit_json["id"].as_u64().unwrap() > 0 {
                    //object["id"] = new_exit_json["id"];
                }
            } else {
                // objects.as_array().unwrap().push(new_exit_json.clone());
            }
        }
    }
}

// function addOrUpdateExit(seedData: LevelList, levelid: number, newExit: Object) {
//     let foundExit = seedData.levels
//       .find((map) => map.id === levelid)
//       .objects.find(
//         (ojb) => ojb.type == "exit" && Math.abs(ojb.x - newExit.x) < 3 && Math.abs(ojb.y - newExit.y) < 3
//       );
//     if (foundExit) {
//         // if it's already added
//         if (newExit.id > 0) {
//           // console.log("updating exit " + newExit.id);
//           seedData.levels.find((map) => map.id === levelid).objects.find((ojb) => ojb.type == "exit" && Math.abs(ojb.x - newExit.x) < 3 && Math.abs(ojb.y - newExit.y) < 3).id = newExit.id;
//         }
//     } else {
//         seedData.levels.find((map) => map.id === levelid).objects.push(newExit);
//     }
//     // console.log(seedData.levels.find((map) => map.id === levelid).objects.filter(ojb => ojb.type == "exit"));
//     return seedData;
// }

// connector being tiles connecting to another level
fn valid_connector_width(level_id: u64, tile_count: u64) -> bool {
    match level_id {
        2 | 3 | 4 | 5 | 6 | 7 | 17 => {
            tile_count == 28 || tile_count == 19
        }
        28 => {
            tile_count == 5 || tile_count == 19 || tile_count == 39
        }
        41 | 42 | 43 | 44 | 45 => {
            tile_count == 8 || tile_count == 9 || tile_count == 11 || tile_count == 13 || tile_count == 19 || tile_count == 22
        }
        79 | 80 | 81 => {
            tile_count == 10 || tile_count == 15 || tile_count == 20
        }
        104 | 106 => {
            tile_count == 5 || tile_count == 6 || tile_count == 9
        }
        105 => {
            tile_count == 5
        }
        107 => {
            tile_count == 10
        }
        _ => false,
    }
}

