use std::cell::RefCell;
use std::rc::Rc;
use actor::behavior::Behavior;
use actor::behavior::aggro::Aggro;
use actor::behavior::player::Player;
use actor::behavior::wanderer::Wanderer;
use game:: MoveInfo;
use geom::{Point};
use map::Map;
use rendering::Color;
use rendering::renderer::RenderingComponent;

pub mod behavior;

#[derive(Copy)]
pub enum ActorType {
    Dog,
    Player
}

pub struct Actor<'a> {
    pub position: Point,
    pub glyph: char,
    pub color: Color,
    pub behavior: Box<Behavior + 'a>
}

impl<'a> Actor<'a> {
    pub fn new_actor(t: ActorType, p: Point, map: &Rc<Map>) -> Actor<'a> {
        match t {
            ActorType::Dog => { Actor::dog(p.x, p.y, map) },
            ActorType::Player => { Actor::player(p.x, p.y, map) }
        }
    }

    pub fn player(x: i32, y: i32, map: &Rc<Map>) -> Actor<'a> {
        let behavior: Box<Behavior> = box Player;
        let position = map.find_empty_tile(Point{ x: x, y: y });
        Actor::new(position.x, position.y, '@', Color::White, behavior)
    }

    pub fn dog(x: i32, y: i32, map: &Rc<Map>) -> Actor<'a> {
        let behavior: Box<Behavior> = box Wanderer;
        let position = map.find_empty_tile(Point{ x: x, y: y });
        Actor::new(position.x, position.y, 'd', Color::DarkAmber, behavior)
    }

    pub fn kobold(x: i32, y: i32, map: &Rc<Map>) -> Actor<'a> {
        let behavior: Box<Behavior> = box Aggro { radius: 10f32 };
        let pos = map.find_empty_tile(Point { x: x, y: y });
        Actor::new(pos.x, pos.y, 'k', Color::DarkerGreen, behavior)
    }

    pub fn new(x: i32, y: i32, glyph: char, color: Color, behavior: Box<Behavior + 'a>) -> Actor<'a> {
        Actor {
            position: Point { x: x, y: y },
            glyph: glyph,
            color: color,
            behavior: behavior
        }
    }

    pub fn update(&mut self, move_info: Rc<RefCell<MoveInfo>>, map: Rc<Map>) {
        self.position = self.behavior.update(self.position, move_info, map);
    }

    pub fn render(&self, view_origin: Point, renderer: &mut Box<RenderingComponent>) {
        let position = Point { x: self.position.x - view_origin.x, y: self.position.y - view_origin.y };
        if position.x >= 0 && position.y >= 0 {
            renderer.render_object_with_color(&position, self.glyph, self.color, Color::Black);
        }
    }
}
