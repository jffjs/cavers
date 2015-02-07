extern crate tcod;

use std::cell::RefCell;
use std::rc::Rc;
use self::tcod::{Console};
use actor::Actor;
use game::state::{State};//, GameState, MessagesState, MovementState};
use geom::{Bounds, Point};
use input::keyboard::KeyboardInput;
use map::Map;
use terrain;
use rendering::renderer::RenderingComponent;
use rendering::renderer::TcodRenderingComponent;

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
    pub map: Rc<Map<'a>>,
    pub player: Box<Actor<'a>>,
    pub actors: Vec<Box<Actor<'a>>>,
    pub game_state: Box<state::GameState + 'a>,
    pub game_state_type: State
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        let window_bounds = Bounds::new(0, 0, 79, 49);
        let map_bounds = Bounds::new(0, 0, 199, 199);
        let console = Console::init_root(window_bounds.max.x + 1, window_bounds.max.y + 1, "cavers", false);
        let rc: Box<TcodRenderingComponent> = box TcodRenderingComponent::new(console);
        let map = Rc::new(Map::new(map_bounds, terrain::random::cave(map_bounds, 4)));
        let c = box Actor::player(40, 25, &map);
        let d = box Actor::dog(10, 10, &map);
        let k = box Actor::kobold(20, 20, &map);
        let actors = vec![d, k];
        let move_info = Rc::new(RefCell::new(MoveInfo::new(map_bounds, c.position)));
        let gs: Box<state::GameState> = box state::movement_state::MovementState::new(map.clone(), move_info.clone());

        Game {
            move_info: move_info,
            exit: false,
            window_bounds: window_bounds,
            rendering_component: rc,
            map: map,
            player: c,
            actors: actors,
            game_state: gs,
            game_state_type: State::Movement
        }
    }

    pub fn render(&mut self) {
        self.game_state.render(&mut self.rendering_component, &self.actors, &self.player);
    }

    pub fn update(&mut self) {
        let next_state = self.game_state.update(&mut self.actors, &mut self.player);
        self.game_state_type = next_state;
        match next_state {
            State::Exit => self.exit = true,
            State::Attack => self.game_state = box state::attack_state::AttackState::new(self.map.clone(), self.move_info.clone()),
            State::Messages => self.game_state = box state::messages_state::MessagesState::new(self.move_info.clone()),
            State::Movement => self.game_state = box state::movement_state::MovementState::new(self.map.clone(), self.move_info.clone()),
            _ => self.game_state = box state::movement_state::MovementState::new(self.map.clone(), self.move_info.clone())
        };
    }
}
