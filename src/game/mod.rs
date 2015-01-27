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
    pub map: Map<'a>
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        let bounds = Bounds {
            min: Point { x: 0, y: 0 },
            max: Point { x: 79, y: 49 }
        };

        let console = Console::init_root(bounds.max.x + 1, bounds.max.y + 1, "cavers", false);
        let rc: Box<TcodRenderingComponent> = box TcodRenderingComponent::new(console);

        let c = box Actor::player(40, 25, bounds);
        let d = box Actor::dog(10, 10, bounds);

        let mut map = Map::new(bounds, terrain::random::cave(bounds, 4), c);
        map.place_actor(d);

        Game {
            exit: false,
            window_bounds: bounds,
            rendering_component: rc,
            map: map
        }
    }

    pub fn render(&mut self) {
        self.rendering_component.before_render_new_frame();
        self.map.render(&mut self.rendering_component);
        // for i in self.map.actors.iter() {
        //     i.render(&mut self.rendering_component);
        // }
        // c.render(&mut self.rendering_component);
        self.rendering_component.after_render_new_frame();
    }

    pub fn update(&mut self, keypress: &KeyboardInput) {
        // c.update(keypress, self);
        self.map.player.update(keypress);
        for i in self.map.actors.iter_mut() {
            // i.update(self);
            i.update(keypress)
        }
    }
}
