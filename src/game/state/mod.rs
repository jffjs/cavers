use actor::Actor;
use rendering::renderer::RenderingComponent;

pub mod messages_state;
pub mod movement_state;

#[derive(Copy)]
pub enum State {
    Start,
    Exit,
    Messages,
    Movement,
}

pub trait GameState {
    fn update(&mut self, &mut Vec<Box<Actor>>, &mut Box<Actor>) -> State;
    fn render(&mut self, &mut Box<RenderingComponent>, &Vec<Box<Actor>>, &Box<Actor>);
}


