mod map;
mod map_builder;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
}

fn main() {
    use crate::prelude::*;
    let mut rng = RandomNumberGenerator::new();
    let mb = MapBuilder::build(&mut rng, Algorithm::Drunkard);
    display("Final Map", &mb.map, &mb.player_start, &mb.amulet_start, &mb.monster_spawns);
}
