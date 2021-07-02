use crate::prelude::*;

pub struct DungeonTheme {}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {// (1)
        Box::new(Self{})// (2)
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('#')
        }
    }
}

pub struct ForestTheme {}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437(';'),
            TileType::Wall => to_cp437('"')
        }
    }
}

impl ForestTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}
