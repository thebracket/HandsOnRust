#![warn(clippy::pedantic)]

mod map;
mod map_builder;
mod player;
mod camera;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
    camera: Camera
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        Self {
            map : map_builder.map,
            player: Player::new(map_builder.player_start),
            camera: Camera::new(map_builder.player_start)
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.player.update(ctx, &self.map, &mut self.camera);
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()// (1)
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT) // (2)
        .with_tile_dimensions(32, 32) // (3)
        .with_resource_path("resources/") // (4)
        .with_font("dungeonfont.png", 32, 32) // (5)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // (6)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, 
            "dungeonfont.png") // (7)
        .build()?;

    main_loop(context, State::new())
}
