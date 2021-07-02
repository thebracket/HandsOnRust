use crate::prelude::*;
use super::MapArchitect;//(1)

pub struct EmptyArchitect {}//(2)

impl MapArchitect for EmptyArchitect {//(3)
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {//(4)
        let mut mb = MapBuilder{//(5)
            map : Map::new(),
            rooms: Vec::new(),
            monster_spawns : Vec::new(),
            player_start : Point::zero(),
            amulet_start : Point::zero()
        };
        mb.fill(TileType::Floor);
        mb.player_start = Point::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2);
        mb.amulet_start = mb.find_most_distant();//(6)
        for _ in 0..50 {//(7)
            mb.monster_spawns.push(
                Point::new(
                    rng.range(1, SCREEN_WIDTH),
                    rng.range(1, SCREEN_WIDTH)
                )
            )
        }
        mb
    }
}