extern crate tcod;

use self::tcod::{Console};
use actor::Actor;
use geom::{Bounds, Point};
use input::keyboard::KeyboardInput;
use map::Map;
use terrain;
use rendering::renderer::RenderingComponent;
use rendering::renderer::TcodRenderingComponent;

pub struct Game<'a> {
    pub exit: bool,
    pub window_bounds: Bounds,
    pub rendering_component: Box<RenderingComponent + 'a>,
    pub map: Box<Map<'a>>,
    pub player: Box<Actor<'a>>,
    pub actors: Vec<Box<Actor<'a>>>
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        let bounds = Bounds {
            min: Point { x: 0, y: 0 },
            max: Point { x: 79, y: 49 }
        };

        let console = Console::init_root(bounds.max.x + 1, bounds.max.y + 1, "cavers", false);
        let rc: Box<TcodRenderingComponent> = box TcodRenderingComponent::new(console);
        let map = box Map::new(bounds, terrain::random::cave(bounds, 4));
        let c = box Actor::player(40, 25, &map);
        let d = box Actor::dog(10, 10, &map);
        let actors = vec![d];

        Game {
            exit: false,
            window_bounds: bounds,
            rendering_component: rc,
            map: map,
            player: c,
            actors: actors
        }
    }

    pub fn render(&mut self) {
        self.rendering_component.before_render_new_frame();
        self.map.render(&mut self.rendering_component);
        for a in self.actors.iter() {
            a.render(&mut self.rendering_component);
        }

        self.player.render(&mut self.rendering_component);
        self.rendering_component.after_render_new_frame();
    }

    pub fn update(&mut self, keypress: &KeyboardInput) {
        self.player.update(keypress, &self.map);
        for i in self.actors.iter_mut() {
            i.update(keypress, &self.map);
        }
    }
}
