use crate::prelude::*;
use super::MapArchitect;

pub struct CellularAutomataArchitect {}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder{
            map : Map::new(),
            rooms: Vec::new(),
            monster_spawns : Vec::new(),
            player_start : Point::zero(),
            amulet_start : Point::zero()
        };
        self.random_noise_map(rng, &mut mb.map);
        for _ in 0..10 {
            self.iteration(&mut mb.map);
        }
        let start = self.find_start(&mb.map);
        mb.monster_spawns = mb.spawn_monsters(&start, rng);
        mb.player_start = start;
        mb.amulet_start = mb.find_most_distant();
        mb
    }
}

impl CellularAutomataArchitect {
    fn random_noise_map(
        &mut self,
        rng: &mut RandomNumberGenerator,
        map: &mut Map)
    {
        map.tiles.iter_mut().for_each(|t| {// (1)
            let roll = rng.range(0, 100);// (2)
            if roll > 55 {// (3)
                *t = TileType::Floor;// (4)
            } else {
                *t = TileType::Wall;
            }
        });
    }

    fn count_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbors = 0;
        for iy in -1 ..= 1 {
            for ix in -1 ..= 1 {
                if !(ix==0 && iy == 0) &&// (5)
                    map.tiles[map_idx(x+ix, y+iy)] == TileType::Wall
                {
                    neighbors += 1;
                }
            }
        }

        neighbors
    }

    fn iteration(&mut self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();// (6)
        for y in 1 .. SCREEN_HEIGHT -1 {// (7)
            for x in 1 .. SCREEN_WIDTH -1 {
                let neighbors = self.count_neighbors(x, y, map);// (8)
                let idx = map_idx(x, y);
                if neighbors > 4 || neighbors == 0 {// (9)
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }
        map.tiles = new_tiles;
    }

    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2);// (10)
        let closest_point = map.tiles
            .iter()// (11)
            .enumerate()// (12)
            .filter(|(_, t)| **t == TileType::Floor)// (13)
            .map(|(idx, _)| (idx, DistanceAlg::Pythagoras.distance2d(// (14)
                center,
                map.index_to_point2d(idx)
            )))
            .min_by(|(_, distance), (_, distance2)| 
                distance.partial_cmp(&distance2).unwrap()// (15)
            )
            .map(|(idx, _)| idx)// (16)
            .unwrap();// (17)
        map.index_to_point2d(closest_point)// (18)
    }
}