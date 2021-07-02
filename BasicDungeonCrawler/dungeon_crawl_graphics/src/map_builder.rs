use crate::prelude::*;
const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map : Map,
    pub rooms : Vec<Rect>,
    pub player_start : Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {

        let mut mb = MapBuilder{
            map : Map::new(),
            rooms : Vec::new(),
            player_start : Point::zero(),
        };
        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();// (1)
        mb
    }

    fn fill(&mut self, tile : TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng : &mut RandomNumberGenerator) {// (2)
        while self.rooms.len() < NUM_ROOMS {// (3)
            let room = Rect::with_size(// (4)
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;// (5)
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {// (6)
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 
                        && p.y < SCREEN_HEIGHT 
                    {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room)
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1:i32, x2:i32, y:i32) {
        use std::cmp::{min, max};
        for x in min(x1,x2) ..= max(x1,x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1:i32, y2:i32, x:i32) {
        use std::cmp::{min, max};
        for y in min(y1,y2) ..= max(y1,y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a,b| a.center().x.cmp(&b.center().x));// (7)

        for (i, room) in rooms.iter().enumerate().skip(1) {// (8)
            let prev = rooms[i-1].center();// (9)
            let new = room.center();

            if rng.range(0,2) == 1 {// (10)
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
}