extern crate tcod;

use self::tcod::{KeyCode};
use self::tcod::Key::Special;
use actor::behavior::Behavior;
use geom::{Bounds, Point};
use geom::Contains::{DoesContain, DoesNotContain};

pub struct Player {
    pub bounds: Bounds
}

impl Player {
    pub fn new(bounds: Bounds) -> Player {
        Player { bounds: bounds }
    }
}

impl Behavior for Player {
    fn update(&self, pos: Point, keypress: &tcod::KeyState) -> Point {
        let mut offset = Point { x: pos.x, y: pos.y };
        offset = match keypress.key {
            Special(KeyCode::Up) => { offset.offset_y(-1) },
            Special(KeyCode::Down) => { offset.offset_y(1) },
            Special(KeyCode::Left) => { offset.offset_x(-1) },
            Special(KeyCode::Right) => { offset.offset_x(1) },
            _ => { offset }
        };

        match self.bounds.contains(offset) {
            DoesContain => { offset }
            DoesNotContain => { pos }
        }
    }
}
