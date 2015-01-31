use core::ops::Deref;
use core::cmp::Ord;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;
use actor::behavior::Behavior;
use game::MoveInfo;
use geom::Point;
use map::Map;

#[derive(Copy, Clone)]
pub struct Aggro {
    pub radius: f32
}

impl Behavior for Aggro {
    fn update(&self, pos: Point, move_info: Rc<RefCell<MoveInfo>>, map: &mut Box<Map> ) -> Point {
        let player_pos = move_info.borrow().deref().player_pos;
        let distance = pos.distance_to(player_pos);
        if distance < 2f32 || distance > self.radius {
            return pos;
        }

        let mut offset = Point { x: pos.x, y: pos.y };
        let move_x = match player_pos.x.cmp(&pos.x) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => 0,
        };
        let move_y = match player_pos.y.cmp(&pos.y) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => 0,
        };

        let mut tile = map.get_tile(offset.offset_x(move_x));
        if !tile.solid {
            offset = offset.offset_x(move_x);
        }
        tile = map.get_tile(offset.offset_y(move_y));
        if !tile.solid {
            offset = offset.offset_y(move_y);
        }

        offset
    }
}
