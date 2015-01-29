use geom::{Point};
use input::keyboard::KeyboardInput;
use map::Map;

pub mod player;
pub mod wanderer;
pub mod spawner;

pub trait Behavior {
    fn update(&self, Point, &KeyboardInput, &mut Box<Map>) -> Point;
}
