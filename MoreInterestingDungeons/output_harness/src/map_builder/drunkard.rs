use crate::prelude::*;
use super::MapArchitect;

const NUM_MONSTERS : usize = 50;
const STAGGER_DISTANCE: usize = 400;
const DESIRED_FLOOR : usize = NUM_TILES / 3;

pub struct DrunkardsWalkArchitect {
    map : Map
}

impl MapArchitect for DrunkardsWalkArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder{
            map : Map::new(),
            monster_spawns : Vec::new(),
            player_start : Point::zero(),
            amulet_start : Point::zero()
        };

        self.map.tiles.iter_mut().for_each(|t| *t = TileType::Wall);
        let center = Point::new(SCREEN_WIDTH /2, SCREEN_HEIGHT/2);

        let path = self.drunkard(&center, rng);
        display("First Drunken Dwarf", &self.map, &mb.player_start, &mb.amulet_start, &path);

        let mut i = 0;
        while self.map.tiles.iter().filter(|t| **t == TileType::Floor).count() < DESIRED_FLOOR
        {
            let path = self.drunkard(&Point::new(rng.range(0, SCREEN_WIDTH), rng.range(0, SCREEN_HEIGHT)), rng);

            let dijkstra_map = DijkstraMap::new(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &vec![self.map.point2d_to_index(center)],
                &self.map,
                1024.0
            );
            dijkstra_map.map
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance > &2000.0)
                .for_each(|(idx, _)| self.map.tiles[idx] = TileType::Wall);

            if i % 5 == 0 {
                display(&format!("Drunken Dwarf #{}", i+2), &self.map, &mb.player_start, &mb.amulet_start, &path);
            }
            i += 1;
        }
        self.add_boundaries();

        mb.map.tiles = self.map.tiles.clone();
        mb.monster_spawns = self.spawn_monsters(&center, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();
        mb
    }
}

impl DrunkardsWalkArchitect {
    pub fn new() -> Self {
        Self{
            map: Map::new()
        }
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
        if spawnable_tiles.is_empty() {
            return spawns;
        }

        for _ in 0 .. NUM_MONSTERS {
            spawns.push(rng.random_slice_entry(&spawnable_tiles).unwrap().clone());
        }
        spawns
    }

    fn drunkard(&mut self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        let mut path = Vec::new();
        let mut drunkard_pos = start.clone();
        let mut distance_staggered = 0;

        loop {
            let drunk_idx = self.map.point2d_to_index(drunkard_pos);
            self.map.tiles[drunk_idx] = TileType::Floor;

            match rng.range(0, 4) {
                0 => drunkard_pos.x -= 1,
                1 => drunkard_pos.x += 1,
                2 => drunkard_pos.y -= 1,
                _ => drunkard_pos.y += 1,
            }
            if !self.map.in_bounds(drunkard_pos) {
                break;
            }
            path.push(drunkard_pos);

            distance_staggered += 1;
            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }

        path
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
}