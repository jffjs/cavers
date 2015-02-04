extern crate tcod;

use std::cell::RefCell;
use std::rc::Rc;
use self::tcod::{Console};
use actor::Actor;
use game::state::{State, GameState, MovementState};
use geom::{Bounds, Point};
use input::keyboard::KeyboardInput;
use map::Map;
use terrain;
use rendering::renderer::RenderingComponent;
use rendering::renderer::TcodRenderingComponent;
use rendering::window::Window;

pub mod state;

#[derive(Copy)]
pub struct MoveInfo {
    pub last_keypress: Option<KeyboardInput>,
    pub player_pos: Point,
    pub view_origin: Point,
    pub bounds: Bounds
}

impl MoveInfo {
    pub fn new(bounds: Bounds, player_pos: Point) -> MoveInfo {
        MoveInfo{ last_keypress: None, bounds: bounds, player_pos: player_pos, view_origin: player_pos }
    }
}

pub struct Game<'a> {
    pub move_info: Rc<RefCell<MoveInfo>>,
    pub exit: bool,
    pub window_bounds: Bounds,
    pub rendering_component: Box<RenderingComponent + 'a>,
    pub menu_window: Box<Window>,
    pub map_window: Box<Window>,
    pub msg_window: Box<Window>,
    pub map: Rc<Map<'a>>,
    pub player: Box<Actor<'a>>,
    pub actors: Vec<Box<Actor<'a>>>,
    pub game_state: Box<GameState + 'a>
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        let window_bounds = Bounds::new(0, 0, 79, 49);
        let map_bounds = Bounds::new(0, 0, 199, 199);
        let map_window_bounds = Bounds::new(0, 0, 59, 48);
        let msg_window_bounds = Bounds::new(0, 49, 59, 49);
        let menu_window_bounds = Bounds::new(60, 0, 79, 49);
        let console = Console::init_root(window_bounds.max.x + 1, window_bounds.max.y + 1, "cavers", false);
        let menu_window = box Window::new(menu_window_bounds);
        let map_window = box Window::new(map_window_bounds);
        let msg_window = box Window::new(msg_window_bounds);
        let rc: Box<TcodRenderingComponent> = box TcodRenderingComponent::new(console);
        let map = Rc::new(Map::new(map_bounds, map_window_bounds, terrain::random::cave(map_bounds, 4)));
        let c = box Actor::player(40, 25, &map);
        let d = box Actor::dog(10, 10, &map);
        let k = box Actor::kobold(20, 20, &map);
        let actors = vec![d, k];
        let move_info = Rc::new(RefCell::new(MoveInfo::new(map_bounds, c.position)));
        let gs: Box<GameState> = box MovementState::new(map.clone(), move_info.clone());

        Game {
            move_info: move_info,
            exit: false,
            window_bounds: window_bounds,
            rendering_component: rc,
            menu_window: menu_window,
            map_window: map_window,
            msg_window: msg_window,
            map: map,
            player: c,
            actors: actors,
            game_state: gs
        }
    }

    pub fn render(&mut self) {
        let mut windows = vec![&mut self.menu_window, &mut self.msg_window];
        self.game_state.render(&mut self.rendering_component, &self.actors, &self.player, &mut windows);
    }

    pub fn update(&mut self) {
        let next_state = self.game_state.update(&mut self.actors, &mut self.player);
        match next_state {
            State::Movement => self.game_state = box MovementState::new(self.map.clone(), self.move_info.clone()),
            _ => self.game_state = box MovementState::new(self.map.clone(), self.move_info.clone())
        }
    }
}
