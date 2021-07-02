#![warn(clippy::pedantic)]

use bracket_lib::prelude::*;

const SCREEN_WIDTH : i32 = 80;
const SCREEN_HEIGHT : i32 = 50;
const FRAME_DURATION : f32 = 75.0;

struct Player {
    x: i32, // (1)
    y: i32, // (2)
    velocity: f32, // (3)
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,// (4)
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set( // (5)
            0, // (6)
            self.y, // (7)
            YELLOW, // (8)
            BLACK,
            to_cp437('@') // (9)
        );
    }

    fn gravity_and_move(&mut self) {
         if self.velocity < 2.0 { // (10)
            self.velocity += 0.2; // (11)
        }
        self.y += self.velocity as i32; // (12)
        self.x += 1; // (13)
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }    
}

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY); // (14)
        self.frame_time += ctx.frame_time_ms; // (15)
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;

            self.player.gravity_and_move();           
        }
        if let Some(VirtualKeyCode::Space) = ctx.key { // (16)
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        if self.player.y > SCREEN_HEIGHT { // (17)
            self.mode = GameMode::End;
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
