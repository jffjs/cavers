use actor::behavior::Behavior;
use actor::behavior::player::Player;
use actor::behavior::wanderer::Wanderer;
use geom::{Point};
use input::keyboard::KeyboardInput;
use map::Map;
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
    pub fn player(x: i32, y: i32, map: &Box<Map>) -> Actor<'a> {
        let behavior: Box<Behavior> = box Player::new(map.bounds);
        let position = map.find_empty_tile(Point{ x: x, y: y });
        Actor::new(position.x, position.y, '@', Color::White, behavior)
    }

    pub fn dog(x: i32, y: i32, map: &Box<Map>) -> Actor<'a> {
        let behavior: Box<Behavior> = box Wanderer::new(map.bounds);
        let position = map.find_empty_tile(Point{ x: x, y: y });
        Actor::new(position.x, position.y, 'd', Color::DarkAmber, behavior)
    }

    pub fn new(x: i32, y: i32, glyph: char, color: Color, behavior: Box<Behavior + 'a>) -> Actor<'a> {
        Actor {
            position: Point { x: x, y: y },
            glyph: glyph,
            color: color,
            behavior: behavior
        }
    }

    pub fn update(&mut self, keypress: &KeyboardInput, map: &Box<Map>) {
        self.position = self.behavior.update(self.position, keypress, map)
    }

    pub fn render(&self, renderer: &mut Box<RenderingComponent>) {
        renderer.render_object_with_color(&self.position, self.glyph, self.color, Color::Black);
    }
}
