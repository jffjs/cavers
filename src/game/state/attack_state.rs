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

pub struct AttackState<'a> {
    pub map: Rc<Map<'a>>,
    pub move_info: Rc<RefCell<MoveInfo>>,
    pub combat_window: Box<Window>,
    pub map_window: Box<Window>,
    pub msg_window: Box<Window>,
}

impl<'a> AttackState<'a> {
    pub fn new(map: Rc<Map>, move_info: Rc<RefCell<MoveInfo>>) -> AttackState {
        let msg_window = box Window::new(Bounds::new(0, 49, 59, 49));
        let map_window = box Window::new(Bounds::new(0, 0, 59, 48));
        let combat_window = box Window::new(Bounds::new(60, 0, 79, 49));
        AttackState { map: map, move_info: move_info, combat_window: combat_window, map_window: map_window, msg_window: msg_window }
    }

    pub fn combat_screen() -> Screen<'a> {
        let screen = vec![
            "Select a target",
        ];

        Screen::new(screen)
    }
}

impl<'a> GameState for AttackState<'a> {
    fn update(&mut self, mobs: &mut Vec<Box<Actor>>, player: &mut Box<Actor>) -> State {
        let keypress = match self.move_info.borrow().deref().last_keypress {
            Some(k) => { k },
            None => { KeyboardInput { key: Special(KeyCode::None) }  }
        };

        match keypress.key {
            Printable('b') => State::Movement,
            _ => { State::Attack }
        }
    }

    fn render(&mut self, renderer: &mut Box<RenderingComponent>, mobs: &Vec<Box<Actor>>, player: &Box<Actor>) {
        renderer.before_render_new_frame();
        renderer.attach_window(&mut self.map_window, Screen::new(vec![]));
        renderer.attach_window(&mut self.msg_window, Screen::new(vec![]));
        renderer.attach_window(&mut self.combat_window, AttackState::combat_screen());
        self.map.render(self.map_window.bounds, self.move_info.clone(), renderer);
        for m in mobs.iter() {
            m.render(self.move_info.borrow().deref().view_origin, renderer);
        }
        player.render(self.move_info.borrow().deref().view_origin, renderer);
        renderer.after_render_new_frame();
    }
}
