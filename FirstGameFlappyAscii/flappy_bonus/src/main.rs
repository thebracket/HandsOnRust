#![warn(clippy::pedantic)]

use bracket_lib::prelude::*;

const SCREEN_WIDTH : i32 = 40;
const SCREEN_HEIGHT : i32 = 25;
const FRAME_DURATION : f32 = 75.0;

const DRAGON_FRAMES : [u16; 6] = [ 64, 1, 2, 3, 2, 1 ];

struct Player {
    x: i32,
    y: f32,
    velocity: f32,
    frame: usize // Usize to index arrays
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y: y as f32,
            velocity: 0.0,
            frame: 0
        }
    }

    fn gravity_and_move(&mut self) {
         if self.velocity < 2.0 {
            self.velocity += 0.1;
        }

        self.y += self.velocity;
        if self.y < 0.0 {
            self.y = 0.0;
        }

        self.x += 1;
        self.frame += 1;
        self.frame = self.frame % 6; // % is modulus - remainder
    }

    fn flap(&mut self) {
        self.velocity = -1.0;
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_fancy(
            PointF::new(0.0, self.y),
            1,
            Degrees::new(0.0),
            PointF::new(2.0, 2.0),
            WHITE,
            NAVY,
            DRAGON_FRAMES[self.frame]
        );
        ctx.set_active_console(0);
    }
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(5, 20),
            size: i32::max(2, 10 - score)
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x : i32) {
        // The ground
        for x in 0..SCREEN_WIDTH {
            ctx.set(x, SCREEN_HEIGHT-1, WHITE, WHITE, to_cp437('#'));
        }

        let screen_x = self.x - player_x;
        let half_size = self.size / 2;
        // Top wall
        for y in 0..self.gap_y - half_size {
            ctx.set(
                screen_x,
                y,
                WHITE,
                NAVY,
                179,
            );
        }

        // Bottom wall - now leaving room for the ground
        for y in self.gap_y + half_size..SCREEN_HEIGHT-1 {
            ctx.set(
                screen_x,
                y,
                WHITE,
                NAVY,
                179,
            );
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        player.x == self.x
            && ((player.y as i32) < self.gap_y - half_size || player.y as i32 > 
            self.gap_y + half_size)
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
    obstacle: Obstacle,
    mode: GameMode,
    score: i32,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            mode: GameMode::Menu,
            score: 0,
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, SCREEN_WIDTH/2);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.mode = GameMode::Playing;
        self.score = 0;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(5, YELLOW, BLACK, "Welcome to Flappy Dragon");
        ctx.print_color_centered(8, CYAN, BLACK, "(P) Play Game");
        ctx.print_color_centered(9, CYAN, BLACK, "(Q) Quit Game");

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
        ctx.print_color_centered(5, RED, BLACK, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_color_centered(8, CYAN, BLACK, "(P) Play Again");
        ctx.print_color_centered(9, CYAN, BLACK, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;

            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }
        if self.player.y as i32 > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) 
        {
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
    let context = BTermBuilder::new()
        .with_font("../resources/flappy32.png", 32, 32)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
        .with_title("Flappy Dragon Enhanced")
        .with_tile_dimensions(16, 16)
        .build()?;

    main_loop(context, State::new())
}
