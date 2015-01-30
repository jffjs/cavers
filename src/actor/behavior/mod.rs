use actor::{Actor, ActorType};
use geom::{Point};
use input::keyboard::KeyboardInput;
use map::Map;

pub mod player;
pub mod wanderer;

pub trait Behavior {
    fn update(&self, Point, &KeyboardInput, &mut Box<Map>) -> Point;
}

#[derive(Copy)]
pub enum Actions {
    Attack,
    Move,
    Spawn,
}

pub struct Action<'a> {
    pub actor: Box<Actor<'a>>,
    pub actor_type: ActorType,
    pub action_type: Actions,
    pub target: Point
}

impl<'a> Action<'a> {
    pub fn new(actor: Box<Actor<'a>>, actor_type: ActorType, action_type: Actions, target: Point) -> Action<'a> {
        Action{ actor: actor, actor_type: actor_type, action_type: action_type, target: target }
    }

    pub fn process_action(&mut self, actors: &mut Vec<Box<Actor>>, map: &mut Box<Map>) {
        match self.action_type {
            Actions::Move => {
                self.actor.position = self.target;
            },
            Actions::Spawn => {
                // let new_actor = box Actor::new_actor(self.actor_type, self.target, map);
                // actors.push(new_actor);
            }
            _ => {}
        }
        
    }
}
