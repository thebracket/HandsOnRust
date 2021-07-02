use crate::prelude::*;
use super::MapArchitect;

const NUM_MONSTERS : usize = 50;

pub struct CellularAutomataArchitect {
    map : Map
}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder{
            map : Map::new(),
            monster_spawns : Vec::new(),
            player_start : Point::zero(),
            amulet_start : Point::zero()
        };
        self.random_noise_map(rng);
        display("Random noise map - 55% floors", &self.map, &mb.player_start, &mb.amulet_start, &mb.monster_spawns);
        for i in 0..10 {
            self.iteration();
            display(&format!("Iteration {}", i+1), &self.map, &mb.player_start, &mb.amulet_start, &mb.monster_spawns);
        }
        self.add_boundaries();
        display("Boundary Check", &self.map, &mb.player_start, &mb.amulet_start, &mb.monster_spawns);
        mb.map.tiles = self.map.tiles.clone();
        let start = self.find_start();
        mb.monster_spawns = self.spawn_monsters(&start, rng);
        mb.player_start = start;
        mb.amulet_start = mb.find_most_distant();
        mb
    }
}

impl CellularAutomataArchitect {
    pub fn new() -> Self {
        Self{
            map: Map::new()
        }
    }

    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator) {
        self.map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0, 100);
            if roll > 55 {
                *t = TileType::Floor;
            } else {
                *t = TileType::Wall;
            }
        });
    }

    fn count_neighbors(&self, x: i32, y: i32) -> usize {
        let mut neighbors = 0;

        if self.map.tiles[map_idx(x-1, y)] == TileType::Wall { neighbors += 1; }
        if self.map.tiles[map_idx(x+1, y)] == TileType::Wall { neighbors += 1; }
        if self.map.tiles[map_idx(x, y-1)] == TileType::Wall { neighbors += 1; }
        if self.map.tiles[map_idx(x, y+1)] == TileType::Wall { neighbors += 1; }
        if self.map.tiles[map_idx(x-1, y-1)] == TileType::Wall { neighbors += 1; }
        if self.map.tiles[map_idx(x+1, y-1)] == TileType::Wall { neighbors += 1; }
        if self.map.tiles[map_idx(x-1, y+1)] == TileType::Wall { neighbors += 1; }
        if self.map.tiles[map_idx(x+1, y+1)] == TileType::Wall { neighbors += 1; }

        neighbors
    }

    fn iteration(&mut self) {
        let mut new_tiles = self.map.tiles.clone();
        for y in 1 .. SCREEN_HEIGHT -1 {
            for x in 1 .. SCREEN_WIDTH -1 {
                let neighbors = self.count_neighbors(x, y);
                let idx = map_idx(x, y);
                if neighbors > 4 || neighbors == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }
        self.map.tiles = new_tiles;
    }

    fn add_boundaries(&mut self) {
        for x in 1 .. SCREEN_WIDTH {
            self.map.tiles[map_idx(x, 1)] = TileType::Wall;
            self.map.tiles[map_idx(x, SCREEN_HEIGHT-1)] = TileType::Wall;
        }
        for y in 1 .. SCREEN_HEIGHT {
            self.map.tiles[map_idx(1, y)] = TileType::Wall;
            self.map.tiles[map_idx(SCREEN_WIDTH-1, y)] = TileType::Wall;
        }
    }

    fn find_start(&self) -> Point {
        let center = Point::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2);
        let closest_point = self
            .map.tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| (idx, DistanceAlg::Pythagoras.distance2d(
                center,
                self.map.index_to_point2d(idx)
            )))
            .min_by(|(_, distance), (_, distance2)| distance.partial_cmp(&distance2).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();
        self.map.index_to_point2d(closest_point)
    }

    fn spawn_monsters(&self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        let spawnable_tiles : Vec<Point> = self.map.tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| 
                **t == TileType::Floor &&
                DistanceAlg::Pythagoras.distance2d(*start, self.map.index_to_point2d(*idx)) > 10.0
            )
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();

        let mut spawns = Vec::new();
        for _ in 0 .. NUM_MONSTERS {
            spawns.push(rng.random_slice_entry(&spawnable_tiles).unwrap().clone());
        }
        spawns
    }
}