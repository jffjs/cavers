extern crate tcod;
use actor::behavior::Behavior;
use geom::{Point};
use rendering::color::Color;
use rendering::rendering_component::RenderingComponent;

pub struct Actor<'a> {
    pub position: Point,
    pub glyph: char,
    pub color: Color,
    pub behavior: Box<Behavior + 'a>
}

impl<'a> Actor<'a> {
    pub fn new(x: i32, y: i32, glyph: char, color: Color, behavior: Box<Behavior + 'a>) -> Actor<'a> {
        Actor {
            position: Point { x: x, y: y },
            glyph: glyph,
            color: color,
            behavior: behavior
        }
    }

    pub fn update(&mut self, keypress: &tcod::KeyState) {
        self.position = self.behavior.update(self.position, keypress)
    }

    pub fn render(&self, renderer: &mut Box<RenderingComponent>) {
        renderer.render_object(&self.position, self.glyph);
    }
}
