extern crate tcod;

use std::cell::RefCell;
use std::rc::Rc;
use self::tcod::{Console};
use actor::{Actor, ActorType};
use geom::{Bounds, Point};
use input::keyboard::KeyboardInput;
use map::Map;
use terrain;
use rendering::renderer::RenderingComponent;
use rendering::renderer::TcodRenderingComponent;

#[derive(Copy)]
pub struct MoveInfo {
    pub last_keypress: Option<KeyboardInput>,
    pub player_pos: Point,
    pub bounds: Bounds
}

impl MoveInfo {
    pub fn new(bounds: Bounds, player_pos: Point) -> MoveInfo {
        MoveInfo{ last_keypress: None, bounds: bounds, player_pos: player_pos }
    }
}

pub struct Game<'a> {
    pub move_info: Rc<RefCell<MoveInfo>>,
    pub exit: bool,
    pub window_bounds: Bounds,
    pub rendering_component: Box<RenderingComponent + 'a>,
    pub map: Box<Map<'a>>,
    pub player: Box<Actor<'a>>,
    pub actors: Vec<Box<Actor<'a>>>
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        let window_bounds = Bounds {
            min: Point { x: 0, y: 0 },
            max: Point { x: 79, y: 49 }
        };

        let map_bounds = Bounds {
            min: Point {x: 0, y: 0},
            max: Point {x: 199, y: 199 }
        };

        let console = Console::init_root(window_bounds.max.x + 1, window_bounds.max.y + 1, "cavers", false);
        let rc: Box<TcodRenderingComponent> = box TcodRenderingComponent::new(console);

        // move this out of Game into main? Add actors to map?
        let map = box Map::new(map_bounds, window_bounds, terrain::random::cave(map_bounds, 4));
        let c = box Actor::player(40, 25, &map);
        let d = box Actor::dog(10, 10, &map);
        let k = box Actor::kobold(20, 20, &map);
        let actors = vec![d, k];
        // let actors: Vec<Box<Actor>> = vec![];

        let move_info = Rc::new(RefCell::new(MoveInfo::new(map_bounds, c.position)));
        Game {
            move_info: move_info,
            exit: false,
            window_bounds: window_bounds,
            rendering_component: rc,
            map: map,
            player: c,
            actors: actors
        }
    }

    pub fn render(&mut self) {
        self.rendering_component.before_render_new_frame();
        self.map.render(self.player.position, &mut self.rendering_component);
        for a in self.actors.iter() {
            a.render(self.map.view_origin, &mut self.rendering_component);
        }
        self.player.render(self.map.view_origin, &mut self.rendering_component);
        self.rendering_component.after_render_new_frame();
    }

    pub fn update(&mut self) {
        self.player.update(self.move_info.clone(), &mut self.map);
        for i in self.actors.iter_mut() {
            i.update(self.move_info.clone(), &mut self.map);
        }
    }
}
