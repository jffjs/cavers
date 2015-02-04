use std::cell::RefCell;
use std::rc::Rc;
use actor::{Actor, ActorType};
use game::MoveInfo;
use geom::{Point};
use input::keyboard::KeyboardInput;
use map::Map;

pub mod player;
pub mod wanderer;
pub mod aggro;

pub trait Behavior {
    fn update(&self, Point, Rc<RefCell<MoveInfo>>, Rc<Map>) -> Point;
}

#[derive(Copy)]
pub enum Actions {
    Attack,
    Move,
    Spawn,
}
