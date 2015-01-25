extern crate tcod;
use self::tcod::{Console, BackgroundFlag};
use geom::{Point};
use rendering::rendering_component::RenderingComponent;

pub struct TcodRenderingComponent {
    pub console: Console
}

impl TcodRenderingComponent {
    pub fn new(console: Console) -> TcodRenderingComponent {
        TcodRenderingComponent {
            console: console
        }
    }
}

impl RenderingComponent for TcodRenderingComponent {
    fn before_render_new_frame(&mut self) {
        self.console.clear();
    }

    fn render_object(&mut self, position: &Point, glyph: char) {
        self.console.put_char(position.x, position.y, glyph, BackgroundFlag::Set);
    }

    fn after_render_new_frame(&mut self) {
        Console::flush();
    }
}
