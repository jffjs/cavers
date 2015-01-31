use core::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;
use actor::behavior::Behavior;
use game::MoveInfo;
use geom::{Bounds, Point};
use geom::Contains::{DoesContain, DoesNotContain};
use map::Map;

#[derive(Copy, Clone)]
pub struct Aggro;

// TODO: aggro radius
impl Behavior for Aggro {
    fn update(&self, pos: Point, move_info: Rc<RefCell<MoveInfo>>, map: &mut Box<Map> ) -> Point {
        let player_pos = move_info.borrow().deref().player_pos;
        let distance = pos.distance_to(player_pos);
        println!("{}", distance);
        if pos.distance_to(player_pos) < 2.0 {
            return pos;
        }
        let move_x = if player_pos.x < pos.x { -1 } else { 1 };
        let move_y = if player_pos.y < pos.y { -1 } else { 1 };
        let offset = Point { x: move_x, y: move_y };
        let new_pos = pos.offset(&offset);
        match map.bounds.contains(new_pos) {
            DoesContain => {
                if !map.get_tile(new_pos).solid {
                    new_pos
                } else {
                    pos
                }
            },
            DoesNotContain => { pos }
        }
    }
}
