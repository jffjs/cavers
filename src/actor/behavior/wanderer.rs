extern crate tcod;

use std::rand;
use std::rand::distributions::{IndependentSample, Range};
use actor::behavior::Behavior;
use geom::{Bounds, Point};
use geom::Contains::{DoesContain, DoesNotContain};
use rendering::rendering_component::RenderingComponent;

#[derive(Copy)]
pub struct Wanderer {
    pub bounds: Bounds
}

impl Behavior for Wanderer {
    fn new(bounds: Bounds) -> Wanderer {
        Wanderer { bounds: bounds }
    }

    fn update(&self, pos: Point, keypress: &tcod::KeyState) -> Point {
        let btwn = Range::new(0,3);
        let mut rng = rand::thread_rng();
        
        let mut offset = Point { x: pos.x, y: pos.y };
        let offset_x = btwn.ind_sample(&mut rng) - 1;
        match self.bounds.contains(offset.offset_x(offset_x)) {
            DoesContain => offset = offset.offset_x(offset_x),
            DoesNotContain => { return pos; }
        }

        let offset_y = btwn.ind_sample(&mut rng) - 1;
        match self.bounds.contains(offset.offset_y(offset_y)) {
            DoesContain => offset = offset.offset_y(offset_y),
            DoesNotContain => { return pos; }
        }

        offset
    }
}
