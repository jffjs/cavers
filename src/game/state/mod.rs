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
        let msg_window = box Window::new(Bounds::new(0, 49, 59, 49));
        let map_window = box Window::new(Bounds::new(0, 0, 59, 48));
        let menu_window = box Window::new(Bounds::new(60, 0, 79, 49));
        MovementState { map: map, move_info: move_info, menu_window: menu_window, map_window: map_window, msg_window: msg_window }
    }

    pub fn menu_screen() -> Screen<'a> {
        let mut screen = Screen::new();
        screen.push_line("a - Attack");
        screen.push_line("m - Messages");
        screen
    }
}
// Create a Screen struct 
// screen will define text to display
// pass screen into attach_window
pub struct Screen<'a> {
    pub data: Vec<&'a str>
}

impl<'a> Screen<'a> {
    pub fn new() -> Screen<'a> {
        let data: Vec<&str> = vec![];
        Screen { data: data }
    }

    pub fn push_line(&mut self, line: &'a str) {
        self.data.push(line);
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

    fn render(&mut self, renderer: &mut Box<RenderingComponent>, mobs: &Vec<Box<Actor>>, player: &Box<Actor>) {
        renderer.attach_window(&mut self.map_window, Screen::new());
        renderer.attach_window(&mut self.msg_window, Screen::new());
        renderer.attach_window(&mut self.menu_window, MovementState::menu_screen());
        self.map.render(self.map_window.bounds, self.move_info.clone(), renderer);
        for m in mobs.iter() {
            m.render(self.move_info.borrow().deref().view_origin, renderer);
        }
        player.render(self.move_info.borrow().deref().view_origin, renderer);
        renderer.after_render_new_frame();
    }
}
