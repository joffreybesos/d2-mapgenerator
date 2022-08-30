use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SeedData {
    pub seed: u32,
    pub difficulty: u32,
    pub levels: Vec<LevelData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelData {
    #[serde(alias = "type")]
    pub level_type: String,
    pub id: u32,
    pub name: String,
    pub offset: Offset,
    pub size: Size,
    pub objects: Vec<Object>,
    pub map: Vec<Vec<u64>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Offset {
    pub x: u32,
    pub y: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Object {
    pub id: u32,
    #[serde(alias = "type")]
    pub object_type: String,
    pub x: u32,
    pub y: u32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub class: String,
    #[serde(default)]
    pub op: u32,
    #[serde(alias = "isGoodExit")]
    #[serde(default)]
    pub is_good_exit: bool,
    #[serde(default)]
    pub owned_level_id: u32,
}

impl Object {
    pub fn new_exit(x: u32, y: u32, owned_level_id: u32, exits: &Vec<Object>) -> Object {
        let mut new_exit_id = 0;
        let attached_levels = crate::walkableexits::get_attached_levels(owned_level_id);
        if attached_levels.len() > 0 {
            new_exit_id = attached_levels[0];
            // println!("1 level {} {}", owned_level_id, new_exit_id);
        }

        // if a neighbouring map has a matching exit then update the id of that exit as well
        let matching_exit: Vec<&Object> = exits
            .iter()
            .filter(|e| (e.x as i32 - x as i32).abs() < 3 && (e.y as i32 - y as i32).abs() < 3)
            .collect();
        if matching_exit.len() > 0 {
            if matching_exit[0].owned_level_id > 0 {
                new_exit_id = matching_exit[0].owned_level_id
            }
            // println!("2 level {} {} {}", owned_level_id, matching_exit[0].owned_level_id, matching_exit[0].id);
        }

        Object {
            id: new_exit_id,
            object_type: "exit".to_owned(),
            x,
            y,
            name: "".to_owned(),
            op: 0,
            class: "".to_owned(),
            is_good_exit: false,
            owned_level_id,
        }
    }
}

pub fn get_level_name(level_id: u32) -> &'static str {
    match level_id {
        1 => "Rogue Encampment",
        2 => "Blood Moor",
        3 => "Cold Plains",
        4 => "Stony Field",
        5 => "Dark Wood",
        6 => "Black Marsh",
        7 => "Tamoe Highland",
        8 => "Den of Evil",
        9 => "Cave Level 1",
        10 => "Underground Passage Level 1",
        11 => "Hole Level 1",
        12 => "Pit Level 1",
        13 => "Cave Level 2",
        14 => "Underground Passage Level 2",
        15 => "Hole Level 2",
        16 => "Pit Level 2",
        17 => "Burial Grounds",
        18 => "Crypt",
        19 => "Mausoleum",
        20 => "Forgotten Tower",
        21 => "Tower Cellar Level 1",
        22 => "Tower Cellar Level 2",
        23 => "Tower Cellar Level 3",
        24 => "Tower Cellar Level 4",
        25 => "Tower Cellar Level 5",
        26 => "Monastery Gate",
        27 => "Outer Cloister",
        28 => "Barracks",
        29 => "Jail Level 1",
        30 => "Jail Level 2",
        31 => "Jail Level 3",
        32 => "Inner Cloister",
        33 => "Cathedral",
        34 => "Catacombs Level 1",
        35 => "Catacombs Level 2",
        36 => "Catacombs Level 3",
        37 => "Catacombs Level 4",
        38 => "Tristram",
        39 => "Secret Cow Level",
        40 => "Lut Gholein",
        41 => "Rocky Waste",
        42 => "Dry Hills",
        43 => "Far Oasis",
        44 => "Lost City",
        45 => "Valley of Snakes",
        46 => "Canyon of the Magi",
        47 => "Sewers Level 1",
        48 => "Sewers Level 2",
        49 => "Sewers Level 3",
        50 => "Harem Level 1",
        51 => "Harem Level 2",
        52 => "Palace Cellar Level 1",
        53 => "Palace Cellar Level 2",
        54 => "Palace Cellar Level 3",
        55 => "Stony Tomb Level 1",
        56 => "Halls of the Dead Level 1",
        57 => "Halls of the Dead Level 2",
        58 => "Claw Viper Temple Level 1",
        59 => "Stony Tomb Level 2",
        60 => "Halls of the Dead Level 3",
        61 => "Claw Viper Temple Level 2",
        62 => "Maggot Lair Level 1",
        63 => "Maggot Lair Level 2",
        64 => "Maggot Lair Level 3",
        65 => "Ancient Tunnels",
        66 => "Tal Rasha's Tomb - Star",
        67 => "Tal Rasha's Tomb - Square",
        68 => "Tal Rasha's Tomb - Semi Circle",
        69 => "Tal Rasha's Tomb - Circle",
        70 => "Tal Rasha's Tomb - Two Chevrons",
        71 => "Tal Rasha's Tomb - Triangle",
        72 => "Tal Rasha's Tomb - Circle with line",
        73 => "Duriel's Lair",
        74 => "Arcane Sanctuary",
        75 => "Kurast Docks",
        76 => "Spider Forest",
        77 => "Great Marsh",
        78 => "Flayer Jungle",
        79 => "Lower Kurast",
        80 => "Kurast Bazaar",
        81 => "Upper Kurast",
        82 => "Kurast Causeway",
        83 => "Travincal",
        84 => "Arachnid Lair",
        85 => "Spider Cavern",
        86 => "Swampy Pit Level 1",
        87 => "Swampy Pit Level 2",
        88 => "Flayer Dungeon Level 1",
        89 => "Flayer Dungeon Level 2",
        90 => "Swampy Pit Level 3",
        91 => "Flayer Dungeon Level 3",
        92 => "Sewers Level 1",
        93 => "Sewers Level 2",
        94 => "Ruined Temple",
        95 => "Disused Fane",
        96 => "Forgotten Reliquary",
        97 => "Forgotten Temple",
        98 => "Ruined Fane",
        99 => "Disused Reliquary",
        100 => "Durance of Hate Level 1",
        101 => "Durance of Hate Level 2",
        102 => "Durance of Hate Level 3",
        103 => "Pandemonium Fortress",
        104 => "Outer Steppes",
        105 => "Plains of Despair",
        106 => "City of the Damned",
        107 => "River of Flame",
        108 => "Chaos Sanctuary",
        109 => "Harrogath",
        110 => "Bloody Foothills",
        111 => "Frigid Highlands",
        112 => "Arreat Plateau",
        113 => "Crystalline Passage",
        114 => "Frozen River",
        115 => "Glacial Trail",
        116 => "Drifter Cavern",
        117 => "Frozen Tundra",
        118 => "Ancients' Way",
        119 => "Icy Cellar",
        120 => "Arreat Summit",
        121 => "Nihlathak's Temple",
        122 => "Halls of Anguish",
        123 => "Halls of Pain",
        124 => "Halls of Vaught",
        125 => "Abaddon",
        126 => "Pit of Acheron",
        127 => "Infernal Pit",
        128 => "Worldstone Keep Level 1",
        129 => "Worldstone Keep Level 2",
        130 => "Worldstone Keep Level 3",
        131 => "Throne of Destruction",
        132 => "Worldstone Chamber",
        133 => "Matron's Den",
        134 => "Forgotten Sands",
        135 => "Furnace of Pain",
        136 => "Uber Tristram",
        _ => "All",
    }
}
