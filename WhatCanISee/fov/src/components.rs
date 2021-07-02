pub use crate::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color : ColorPair,
    pub glyph : FontCharType
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AmuletOfYala;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity : Entity,
    pub destination : Point
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker : Entity,
    pub victim : Entity
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, Debug, PartialEq)]// (1)
pub struct FieldOfView{
    pub visible_tiles : HashSet<Point>,// (2)
    pub radius: i32,// (3)
    pub is_dirty: bool// (4)
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self{
            visible_tiles: HashSet::new(),// (5)
            radius,
            is_dirty: true// (6)
        }
    }

    pub fn clone_dirty(&self) -> Self {// (7)
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}
