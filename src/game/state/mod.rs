use core::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;
use actor::Actor;
use game::MoveInfo;
use geom::Bounds;
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
    fn render(&mut self, &mut Box<RenderingComponent>, &Vec<Box<Actor>>, &Box<Actor>);
}

pub struct MovementState<'a> {
    pub map: Rc<Map<'a>>,
    pub move_info: Rc<RefCell<MoveInfo>>,
    pub menu_window: Box<Window>,
    pub map_window: Box<Window>,
    pub msg_window: Box<Window>,
}


impl<'a> MovementState<'a> {
    pub fn new(map: Rc<Map>, move_info: Rc<RefCell<MoveInfo>>) -> MovementState {
        let menu_window = box Window::new(Bounds::new(0, 49, 59, 49));
        let map_window = box Window::new(Bounds::new(0, 0, 59, 48));
        let msg_window = box Window::new(Bounds::new(60, 0, 79, 49));
        MovementState { map: map, move_info: move_info, menu_window: menu_window, map_window: map_window, msg_window: msg_window }
    }
}
// Create a Screen struct 
// screen will define text to display
// pass screen into attach_window
impl<'a> GameState for MovementState<'a> {
    fn update(&mut self, mobs: &mut Vec<Box<Actor>>, player: &mut Box<Actor>) -> State {
        player.update(self.move_info.clone(), self.map.clone());
        for i in mobs.iter_mut() {
            i.update(self.move_info.clone(), self.map.clone());
        }
        State::Movement
    }

    fn render(&mut self, renderer: &mut Box<RenderingComponent>, mobs: &Vec<Box<Actor>>, player: &Box<Actor>) {
        renderer.attach_window(&mut self.map_window);
        renderer.attach_window(&mut self.menu_window);
        renderer.attach_window(&mut self.msg_window);
        self.map.render(self.map_window.bounds, self.move_info.clone(), renderer);
        for m in mobs.iter() {
            m.render(self.move_info.borrow().deref().view_origin, renderer);
        }
        player.render(self.move_info.borrow().deref().view_origin, renderer);
        renderer.after_render_new_frame();
    }
}
