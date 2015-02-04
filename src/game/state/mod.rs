use core::ops::Deref;
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
    fn update(&mut self, &mut Vec<Box<Actor>>, &mut Box<Actor>) -> State;
    fn render(&self, &mut Box<RenderingComponent>, &Vec<Box<Actor>>, &Box<Actor>, &mut Vec<&mut Box<Window>>);
}

pub struct MovementState<'a> {
    pub map: Rc<Map<'a>>,
    pub move_info: Rc<RefCell<MoveInfo>>
}


impl<'a> MovementState<'a> {
    pub fn new(map: Rc<Map>, move_info: Rc<RefCell<MoveInfo>>) -> MovementState {
        MovementState { map: map, move_info: move_info }
    }
}

impl<'a> GameState for MovementState<'a> {
    fn update(&mut self, mobs: &mut Vec<Box<Actor>>, player: &mut Box<Actor>) -> State {
        player.update(self.move_info.clone(), self.map.clone());
        for i in mobs.iter_mut() {
            i.update(self.move_info.clone(), self.map.clone());
        }
        State::Movement
    }

    fn render(&self, renderer: &mut Box<RenderingComponent>, mobs: &Vec<Box<Actor>>, player: &Box<Actor>, windows: &mut Vec<&mut Box<Window>>) {
        renderer.before_render_new_frame();
        self.map.render(self.move_info.clone(), renderer);
        for w in windows.iter_mut() {
            renderer.attach_window(*w);
        }
        for m in mobs.iter() {
            m.render(self.move_info.borrow().deref().view_origin, renderer);
        }
        player.render(self.move_info.borrow().deref().view_origin, renderer);
        renderer.after_render_new_frame();
    }
}
