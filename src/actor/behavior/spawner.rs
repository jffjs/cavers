use std::rand;
use std::rand::distributions::{IndependentSample, Range};
use actor::Actor;
use actor::ActorType;
use actor::behavior::Behavior;
use geom::{Bounds, Point};
use geom::Contains::{DoesContain, DoesNotContain};
use input::keyboard::KeyboardInput;
use map::Map;

pub struct Spawner {
    pub bounds: Bounds,
    pub actor_type: ActorType,
    pub spawn_rate: u32
}

impl Spawner {
    pub fn new(bounds: Bounds, ac: ActorType, spawn_rate: u32) -> Spawner {
        Spawner { bounds: bounds, spawn_rate: spawn_rate, actor_type: ac }
    }
}

impl Behavior for Spawner {
    fn update(&self, pos: Point, keypress: &KeyboardInput, map: &mut Box<Map>) -> Point {
        if rand::random::<u32>() % 100 <= self.spawn_rate {

            let adjacent_points = pos.adjacent();
            let btwn = Range::new(0,adjacent_points.len());
            let mut rng = rand::thread_rng();
            let i = btwn.ind_sample(&mut rng);

            // let spawn = match map.bounds.contains(adjacent_points[i]) {
            //     DoesContain => {
            //         Actor::new_actor(self.actor_type, adjacent_points[i], map)
            //     },
            //     DoesNotContain => {}
            // };
        }
        pos
    }
}
