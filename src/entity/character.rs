extern crate tcod;
use self::tcod::{Console, BackgroundFlag, KeyCode};
use self::tcod::Key::Special;
use entity::traits::Updates;
use game::Game;
use geom::Point;
use geom::Contains::{DoesContain, DoesNotContain};

pub struct Character {
    pub position: Point,
    pub glyph: char
}

impl Character {
    pub fn new(x: i32, y: i32, glyph: char) -> Character {
        Character { position: Point { x: x, y: y }, glyph: glyph }
    }
}

impl Updates for Character {
    fn update(&mut self, keypress: &tcod::KeyState, game: &Game) {
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

    fn render(&self, console: &mut Console) {
        console.put_char(self.position.x, self.position.y, self.glyph, BackgroundFlag::Set);
    }
}
