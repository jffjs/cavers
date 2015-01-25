extern crate tcod;

use self::tcod::{KeyCode};
use self::tcod::Key::Special;
use game::Game;
use geom::Point;
use geom::Contains::{DoesContain, DoesNotContain};
use rendering::rendering_component::RenderingComponent;

#[derive(Copy)]
pub struct Character {
    pub position: Point,
    pub glyph: char
}

impl Character {
    pub fn new(x: i32, y: i32, glyph: char) -> Character {
        Character { position: Point { x: x, y: y }, glyph: glyph }
    }

    pub fn update(&mut self, keypress: &tcod::KeyState, game: &Game) {
        let mut offset = Point { x: 0, y: 0 };
        match keypress.key {
            Special(KeyCode::Up) => {
                offset.y = -1;
            },
            Special(KeyCode::Down) => {
                offset.y = 1;
            },
            Special(KeyCode::Left) => {
                offset.x = -1
            },
            Special(KeyCode::Right) => {
                offset.x = 1;
            },
            _ => {}
        }

        match game.window_bounds.contains(self.position.offset(&offset)) {
            DoesContain => self.position = self.position.offset(&offset),
            DoesNotContain => {}
        }
    }

    pub fn render(&self, renderer: &mut Box<RenderingComponent>) {
        renderer.render_object(&self.position, self.glyph);
    }
}
