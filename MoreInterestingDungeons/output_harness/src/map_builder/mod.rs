use crate::prelude::*;
mod empty;
use empty::EmptyArchitect;
mod rooms;
use rooms::RoomsArchitect;
mod automata;
use automata::CellularAutomataArchitect;
mod drunkard;
use drunkard::DrunkardsWalkArchitect;
mod prefab;
use prefab::apply_prefab;

trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub struct MapBuilder {
    pub map : Map,
    pub monster_spawns : Vec<Point>,
    pub player_start : Point,
    pub amulet_start : Point
}

#[allow(dead_code)]
pub enum Algorithm{
    Empty, Cellular, Rooms, Drunkard
}

impl MapBuilder {
    pub fn build(rng: &mut RandomNumberGenerator, algo: Algorithm) -> Self {
        let mut architect : Box<dyn MapArchitect> = match algo {
            Algorithm::Empty => Box::new(EmptyArchitect{}),
            Algorithm::Cellular => Box::new(CellularAutomataArchitect::new()),
            Algorithm::Rooms => Box::new(RoomsArchitect::new()),
            Algorithm::Drunkard => Box::new(DrunkardsWalkArchitect::new())
        };
        let mut mb = architect.new(rng);
        apply_prefab(&mut mb, rng);
        mb
    }

    fn fill(&mut self, tile : TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_most_distant(&self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0
        );

        const UNREACHABLE : &f32 = &f32::MAX;
        self.map.index_to_point2d
        (
            dijkstra_map.map
                .iter()
                .enumerate()
                .filter(|(_,dist)| *dist < UNREACHABLE)
                .max_by(|a,b| a.1.partial_cmp(b.1).unwrap())
                .unwrap().0
        )
    }
}

pub fn display(title: &str, map: &Map, player_start: &Point, amulet_start: &Point, monster_spawns: &[Point]) {
    use colored::*;
    use std::io::stdin;
    let mut output = vec!['.'; NUM_TILES];

    map.tiles.iter().enumerate().for_each(|(idx, t)| {
        match *t {
            TileType::Floor => output[idx] = '.',
            TileType::Wall => output[idx] = '#'
        }
    });

    output[map.point2d_to_index(*player_start)] = '@';
    output[map.point2d_to_index(*amulet_start)] = 'A';
    monster_spawns.iter().for_each(|p| {
        output[map.point2d_to_index(*p)] = 'M';
    });

    print!("\x1B[2J"); // CLS!
    println!("----------------------\n{}\n----------------------", title.bright_yellow());
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            match output[map_idx(x,y)] {
                '#' => print!("{}", "#".bright_green()),
                '@' => print!("{}", "@".bright_yellow()),
                'M' => print!("{}", "M".bright_red()),
                'A' => print!("{}", "A".bright_magenta()),
                _ => print!("{}", ".".truecolor(64, 64, 64))
            }
        }
        println!("");
    }

    let mut ignore_me = String::new();
    stdin()
        .read_line(&mut ignore_me)
        .expect("Failed to read line");
}