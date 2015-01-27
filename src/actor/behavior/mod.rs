extern crate tcod;
use geom::{Point};

pub mod player;
pub mod wanderer;

pub trait Behavior {
    fn update(&self, Point, &tcod::KeyState) -> Point;
}
