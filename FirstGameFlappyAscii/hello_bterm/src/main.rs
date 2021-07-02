use bracket_lib::prelude::*;

struct State {}

impl GameState for State {// (1)
    fn tick(&mut self, ctx: &mut BTerm) { // (2)
        ctx.cls(); // (3)
        ctx.print(1, 1, "Hello, Bracket Terminal!"); // (4)
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50() // (5)
        .with_title("Flappy Dragon") // (6)
        .build()?; // (7)

    main_loop(context, State{})
}
