use geom::{Point};
use input::keyboard::KeyboardInput;
use map::Map;

pub mod player;
pub mod wanderer;

pub trait Behavior {
    fn update(&self, Point, &KeyboardInput, &Box<Map>) -> Point;
}
