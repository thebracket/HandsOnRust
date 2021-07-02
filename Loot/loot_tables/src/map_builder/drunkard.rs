use crate::prelude::*;
use super::MapArchitect;

const STAGGER_DISTANCE: usize = 400;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const DESIRED_FLOOR : usize = NUM_TILES / 3;

pub struct DrunkardsWalkArchitect {}

impl MapArchitect for DrunkardsWalkArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder{
            map : Map::new(),
            rooms : Vec::new(),
            monster_spawns : Vec::new(),
            player_start : Point::zero(),
            amulet_start : Point::zero(),
            theme: super::themes::DungeonTheme::new()
        };

        mb.fill(TileType::Wall);
        let center = Point::new(SCREEN_WIDTH /2, SCREEN_HEIGHT/2);
        self.drunkard(&center, rng, &mut mb.map);
        while mb.map.tiles.iter()
            .filter(|t| **t == TileType::Floor).count() < DESIRED_FLOOR// (1)
        {
            self.drunkard(
                &Point::new(
                    rng.range(0, SCREEN_WIDTH),
                    rng.range(0, SCREEN_HEIGHT)
                ),
                rng,
                &mut mb.map
            );// (2)
            let dijkstra_map = DijkstraMap::new(// (3)
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &vec![mb.map.point2d_to_index(center)],
                &mb.map,
                1024.0
            );
            dijkstra_map.map// (4)
                .iter()
                .enumerate()// (5)
                .filter(|(_, distance)| *distance > &2000.0)
                .for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall);
        }
        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();
        mb
    }
}

impl DrunkardsWalkArchitect {
    fn drunkard(
        &mut self, 
        start: &Point, 
        rng: &mut RandomNumberGenerator,
        map: &mut Map
    ) {
        let mut drunkard_pos = start.clone();// (6)
        let mut distance_staggered = 0;// (7)

        loop {// (8)
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;// (9)

            match rng.range(0, 4) {// (10)
                0 => drunkard_pos.x -= 1,
                1 => drunkard_pos.x += 1,
                2 => drunkard_pos.y -= 1,
                _ => drunkard_pos.y += 1,
            }
            if !map.in_bounds(drunkard_pos) {// (11)
                break;
            }
 
            distance_staggered += 1;// (12)
            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
}