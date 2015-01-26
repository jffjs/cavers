extern crate tcod;

use self::tcod::{Console};
use entity::character::Character;
use entity::traits::Updates;
use geom::{Bounds, Point};
use map::Map;
use rendering::rendering_component::RenderingComponent;
use rendering::tcod_rendering_component::TcodRenderingComponent;

pub struct Game<'a> {
    pub exit: bool,
    pub window_bounds: Bounds,
    pub rendering_component: Box<RenderingComponent + 'a>
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        let bounds = Bounds {
            min: Point { x: 0, y: 0 },
            max: Point { x: 79, y: 49 }
        };

        let console = Console::init_root(bounds.max.x + 1, bounds.max.y + 1, "cavers", false);
        let rc: Box<TcodRenderingComponent> = box TcodRenderingComponent::new(console);
        Game {
            exit: false,
            window_bounds: bounds,
            rendering_component: rc
        }
    }

    pub fn render(&mut self, map: &Map, mobs: &Vec<Box<Updates>>, c: &Character) {
        self.rendering_component.before_render_new_frame();
        map.render(&mut self.rendering_component);
        for i in mobs.iter() {
            i.render(&mut self.rendering_component);
        }
        c.render(&mut self.rendering_component);
        self.rendering_component.after_render_new_frame();
    }

    pub fn update(&mut self, mobs: &mut Vec<Box<Updates>>, c: &mut Character, keypress: &tcod::KeyState) {
        c.update(keypress, self);
        for i in mobs.iter_mut() {
            i.update(self);
        }
    }
}
