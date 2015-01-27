extern crate tcod;
use actor::behavior::Behavior;
use actor::behavior::player::Player;
use actor::behavior::wanderer::Wanderer;
use geom::{Bounds, Point};
use rendering::Color;
use rendering::renderer::RenderingComponent;

pub mod behavior;

pub struct Actor<'a> {
    pub position: Point,
    pub glyph: char,
    pub color: Color,
    pub behavior: Box<Behavior + 'a>
}

impl<'a> Actor<'a> {
    pub fn player(x: i32, y: i32, bounds: Bounds) -> Actor<'a> {
        let behavior: Box<Behavior> = box Player::new(bounds);
        Actor::new(x, y, '@', Color::White, behavior)
    }

    pub fn dog(x: i32, y: i32, bounds: Bounds) -> Actor<'a> {
        let behavior: Box<Behavior> = box Wanderer::new(bounds);
        Actor::new(x, y, 'd', Color::DarkAmber, behavior)
    }

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
        renderer.render_object_with_color(&self.position, self.glyph, self.color, Color::Black);
    }
}
