use actor::behavior::Behavior;
use geom::{Bounds, Point};
use geom::Contains::{DoesContain, DoesNotContain};
use input::keyboard::{KeyboardInput, KeyCode};
use input ::keyboard::Key::{Special};
use map::Map;

#[derive(Copy, Clone)]
pub struct Player {
    pub bounds: Bounds
}

impl Player {
    pub fn new(bounds: Bounds) -> Player {
        Player { bounds: bounds }
    }
}

impl Behavior for Player {
    fn update(&self, pos: Point, keypress: &KeyboardInput, map: &Box<Map>) -> Point {
        let mut offset = Point { x: pos.x, y: pos.y };
        offset = match keypress.key {
            Special(KeyCode::Up) => { offset.offset_y(-1) },
            Special(KeyCode::Down) => { offset.offset_y(1) },
            Special(KeyCode::Left) => { offset.offset_x(-1) },
            Special(KeyCode::Right) => { offset.offset_x(1) },
            _ => { offset }
        };

        offset = match self.bounds.contains(offset) {
            DoesContain => { offset }
            DoesNotContain => { pos }
        };

        if map.get_tile(offset).solid {
            pos
        } else {
            offset
        }
    }
}
