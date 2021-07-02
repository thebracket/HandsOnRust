use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn in_bounds(&self, point : Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn try_idx(&self, point : Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }

    pub fn can_enter_tile(&self, point : Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)]==TileType::Floor
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {// (1)
        let destination = loc + delta;// (2)
        if self.in_bounds(destination) {// (3)
            if self.can_enter_tile(destination) {// (4)
                let idx = self.point2d_to_index(destination);// (5)
                Some(idx)
            } else {
                None// (6)
            }
        } else {
            None
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize)
    -> SmallVec<[(usize, f32); 10]>// (7)
    {
        let mut exits = SmallVec::new();// (8)
        let location = self.index_to_point2d(idx);// (9)

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {// (10)
            exits.push((idx, 1.0))// (11)
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {// (12)
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }

        exits// (13)
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras
            .distance2d(
                self.index_to_point2d(idx1),
                self.index_to_point2d(idx2)
            )
    }
}
