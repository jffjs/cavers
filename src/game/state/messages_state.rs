use core::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;
use actor::Actor;
use game::MoveInfo;
use game::state::{State, GameState};
use geom::Bounds;
use input::keyboard::{KeyboardInput, KeyCode};
use input ::keyboard::Key::{Printable, Special};
use map::Map;
use rendering::renderer::RenderingComponent;
use rendering::window::{Screen, Window};

pub struct MessagesState {
    pub move_info: Rc<RefCell<MoveInfo>>,
    pub msg_window: Box<Window>,
}

impl MessagesState {
    pub fn new(move_info: Rc<RefCell<MoveInfo>>) -> MessagesState {
        let msg_window = box Window::new(Bounds::new(0, 0, 79, 79));
        MessagesState { move_info: move_info, msg_window: msg_window }
    }
}

impl GameState for MessagesState {
    fn update(&mut self, mobs: &mut Vec<Box<Actor>>, player: &mut Box<Actor>) -> State {
        let keypress = match self.move_info.borrow().deref().last_keypress {
            Some(k) => { k },
            None => { KeyboardInput { key: Special(KeyCode::None) }  }
        };

        match keypress.key {
            Printable('b') => State::Movement,
            _ => State::Messages
        }
    }

    fn render(&mut self, renderer: &mut Box<RenderingComponent>, mobs: &Vec<Box<Actor>>, player: &Box<Actor>) {
        let screen = vec![
            "Kobold stabs your left leg for 3 dmg",
            "You punch kobold in the face for 2 dmg"
                ];
        renderer.before_render_new_frame();
        renderer.attach_window(&mut self.msg_window, Screen::new(screen));
        renderer.after_render_new_frame();
    }
}
