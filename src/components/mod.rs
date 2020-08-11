mod player;
pub use player::*;

use specs::prelude::*;
use specs_derive::Component;
use tcod::Color;

#[derive(Component, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
pub struct Renderable {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
}
