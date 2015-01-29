extern crate tcod;
use self::tcod::{Console, BackgroundFlag};
use geom::{Point};
use rendering::Color;

pub trait RenderingComponent {
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, &Point, char);
    fn render_object_with_color(&mut self, &Point, char, Color, Color);
    fn after_render_new_frame(&mut self);
    fn screen_width(&self) -> i32;
    fn screen_height(&self) -> i32;
}

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

    fn render_object_with_color(&mut self, position: &Point, glyph: char, fore_color: Color, back_color: Color) {
        self.console.put_char_ex(position.x, position.y, glyph, fore_color.value(), back_color.value());
    }

    fn after_render_new_frame(&mut self) {
        Console::flush();
    }

    fn screen_width(&self) -> i32 {
        self.console.width()
    }

    fn screen_height(&self) -> i32 {
        self.console.height()
    }
}