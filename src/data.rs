use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SeedData {
    pub seed: u32,
    pub difficulty: u32,
    pub levels: Vec<LevelData>

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
    pub height: u32
}

#[derive(Serialize, Deserialize, Debug)]
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
    pub is_good_exit: bool
}
