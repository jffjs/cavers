use core::ops::{Deref, DerefMut};
use std::cell::RefCell;
use std::rc::Rc;
use actor::behavior::Behavior;
use game::MoveInfo;
use geom::{Bounds, Point};
use geom::Contains::{DoesContain, DoesNotContain};
use input::keyboard::{KeyboardInput, KeyCode};
use input ::keyboard::Key::{Special};
use map::Map;

#[derive(Copy, Clone)]
pub struct Player;

impl Behavior for Player {
    fn update(&self, pos: Point, move_info: Rc<RefCell<MoveInfo>>, map: &mut Box<Map>) -> Point {
        let keypress = match move_info.borrow().deref().last_keypress {
            Some(k) => { k },
            None => { KeyboardInput { key: Special(KeyCode::None) }  }
        };

        let mut offset = Point { x: pos.x, y: pos.y };
        offset = match keypress.key {
            Special(KeyCode::Up) => {
                offset.offset_y(-1)
            },
            Special(KeyCode::Down) => {
                offset.offset_y(1)
            },
            Special(KeyCode::Left) => {
                offset.offset_x(-1)
            },
            Special(KeyCode::Right) => {
                offset.offset_x(1)
            },
            _ => { offset }
        };

        offset = match map.bounds.contains(offset) {
            DoesContain => { offset }
            DoesNotContain => { pos }
        };

        if map.get_tile(offset).solid {
            // pos
            offset
        } else {
            move_info.borrow_mut().deref_mut().player_pos = offset;
            offset
        }
    }
}
