extern crate tcod;

use geom::{Bounds, Point};

pub mod player;
pub mod wanderer;

pub trait Behavior {
    fn new(Bounds) -> Self;
    fn update(&self, Point, &tcod::KeyState) -> Point;
}
