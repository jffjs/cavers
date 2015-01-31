extern crate tcod;

use self::tcod::{Console, BackgroundFlag};
use geom::Bounds;
use rendering::Color;

pub struct Window {
    pub console: Console,
    pub background_color: Color,
    bounds: Bounds
}

impl Window {
    pub fn new(bounds: Bounds) -> Window {
        let height = bounds.height();
        let width = bounds.width();
        let console = Console::new(width, height);
        Window {
            console: console,
            background_color: Color::Black,
            bounds: bounds
        }
    }

    pub fn get_bounds(&self) -> Bounds { self.bounds }

    pub fn get_console(&mut self) -> &mut Console { &mut self.console }

    pub fn get_bg_color(&self) -> Color { self.background_color }

    pub fn clear(&mut self) {
        let color = self.get_bg_color();
        let mut console = self.get_console();
        console.set_default_background(color.value());
        console.clear();
    }

    pub fn print_message(&mut self, x: i32, y: i32, alignment: tcod::TextAlignment, text: &str) {
        let mut console = self.get_console();
        console.print_ex(x, y, BackgroundFlag::Set, alignment, text);
    }
}
