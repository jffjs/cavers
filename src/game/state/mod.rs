use std::cell::RefCell;
use std::rc::Rc;
use actor::Actor;
use game::MoveInfo;
use map::Map;
use rendering::renderer::RenderingComponent;
use rendering::window::Window;

#[derive(Copy)]
pub enum State {
    Start,
    Movement
}

pub trait GameState {
    fn update(&mut self, &mut Vec<Box<Actor>>, &mut Box<Actor>, &mut Box<Map>, Rc<RefCell<MoveInfo>>) -> State;
    fn render(&mut self, &mut Box<RenderingComponent>, &Vec<Box<Actor>>, &Box<Actor>, &mut Vec<&mut Box<Window>>, &mut Box<Map>);
}

pub struct MovementState;

impl MovementState {
    pub fn new() -> MovementState {
        MovementState
    }
}

impl GameState for MovementState {
    fn update(&mut self, mobs: &mut Vec<Box<Actor>>, player: &mut Box<Actor>, map: &mut Box<Map>, move_info: Rc<RefCell<MoveInfo>>) -> State {
        player.update(move_info.clone(), map);
        for i in mobs.iter_mut() {
            i.update(move_info.clone(), map);
        }
        State::Movement
    }

    fn render(&mut self, renderer: &mut Box<RenderingComponent>, mobs: &Vec<Box<Actor>>, player: &Box<Actor>, windows: &mut Vec<&mut Box<Window>>, map: &mut Box<Map>) {
        renderer.before_render_new_frame();
        map.render(player.position, renderer);
        for w in windows.iter_mut() {
            renderer.attach_window(*w);
        }
        for m in mobs.iter() {
            m.render(map.view_origin, renderer);
        }
        player.render(map.view_origin, renderer);
        renderer.after_render_new_frame();
    }
}
