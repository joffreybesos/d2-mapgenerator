use pathfinding::prelude::astar;

use crate::{mapgrid::{MapGrid, Pos}, jsondata::{LevelData}};


pub fn get_path_data(level_data: &LevelData, map_grid: &MapGrid, path_start: &String, path_end: &String) -> Vec<Pos> {
    let start_pos = get_pos(&level_data, path_start);
    let end_pos = get_pos(&level_data, path_end);
    let path_data = get_path(map_grid, start_pos, end_pos);
    match path_data {
        Some(vec) => vec.0,
        None => vec![]
    }
}


pub fn get_pos(level_data: &LevelData, path_pos: &String) -> Pos {
    let mut actual_pos = Pos(0,0);
    if path_pos.contains(',') {
        let v: Vec<i16> = path_pos.split(',').map(|s| s.parse::<i16>().unwrap()).collect();
        actual_pos = Pos(v[0], v[1]);
    } else {
        let object_id = path_pos.parse::<u32>().unwrap();
        if object_id > 0 {
            let start_obj = level_data.objects.iter().find(|&obj| obj.id == object_id);
            match start_obj {
                Some(obj) => {
                    actual_pos = Pos(obj.x.try_into().unwrap(), obj.y.try_into().unwrap());
                },
                None => (),
            }
        }
    }
    actual_pos
}

pub fn get_path(map_grid: &MapGrid, start_pos: Pos, end_pos: Pos) -> Option<(Vec<Pos>, u32)> {
    astar(
        &start_pos,
        |p| map_grid.get_successors(p).iter().map(|s| (s.pos, s.cost)).collect::<Vec<_>>(),
        |p| ((p.0 - end_pos.0).abs() + (p.1 - end_pos.1).abs()) as u32,
        |p| *p==end_pos)
}
