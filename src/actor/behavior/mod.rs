use geom::{Point};
use input::keyboard::KeyboardInput;

pub mod player;
pub mod wanderer;

pub trait Behavior {
    fn update(&self, Point, &KeyboardInput) -> Point;
}
