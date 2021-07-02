#![warn(clippy::pedantic)]

mod map; // (1)

mod prelude { // (2)
    pub use bracket_lib::prelude::*; // (3)
    pub const SCREEN_WIDTH: i32 = 80; // (4)
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*; // (5)
}

use prelude::*; // (6)

struct State {
    map: Map,
}

impl State {
    fn new() -> Self {
        Self { map: Map::new() }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)// (7)
        .build()?;

    main_loop(context, State::new())
}
