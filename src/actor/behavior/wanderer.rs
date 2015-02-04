use std::cell::RefCell;
use std::rc::Rc;
use std::rand;
use std::rand::distributions::{IndependentSample, Range};
use actor::behavior::Behavior;
use game::MoveInfo;
use geom::{Bounds, Point};
use geom::Contains::{DoesContain, DoesNotContain};
use map::Map;

#[derive(Copy, Clone)]
pub struct Wanderer;

impl Behavior for Wanderer {
    fn update(&self, pos: Point, move_info: Rc<RefCell<MoveInfo>>, map: Rc<Map>) -> Point {
        let btwn = Range::new(0,3);
        let mut rng = rand::thread_rng();
        
        let mut offset = Point { x: pos.x, y: pos.y };
        let offset_x = btwn.ind_sample(&mut rng) - 1;
        match map.bounds.contains(offset.offset_x(offset_x)) {
            DoesContain => offset = offset.offset_x(offset_x),
            DoesNotContain => { return pos; }
        }

        let offset_y = btwn.ind_sample(&mut rng) - 1;
        match map.bounds.contains(offset.offset_y(offset_y)) {
            DoesContain => offset = offset.offset_y(offset_y),
            DoesNotContain => { return pos; }
        }

        if map.get_tile(offset).solid {
            pos
        } else {
            offset
        }

    }
}
