pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color : ColorPair, // (1)
    pub glyph : FontCharType // (2)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player; // (3)


