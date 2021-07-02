pub use crate::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color : ColorPair,
    pub glyph : FontCharType
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player{
    pub map_level: u32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AmuletOfYala;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesHealing{
    pub amount: i32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesDungeonMap;

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

#[derive(Clone, PartialEq)]
pub struct Carried(pub Entity);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActivateItem {
    pub used_by : Entity,
    pub item : Entity
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView{
    pub visible_tiles : HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self{
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}
