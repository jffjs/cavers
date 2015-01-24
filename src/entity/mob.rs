extern crate tcod;
use std;
use std::rand::distributions::{IndependentSample, Range};
use self::tcod::{Console, BackgroundFlag};
use entity::traits::Updates;
use game::Game;
use geom::Point;
use geom::Contains::{DoesContain, DoesNotContain};

pub struct Mob {
    pub position: Point,
    pub glyph: char
}

impl Mob {
    pub fn new(x: i32, y: i32, glyph: char) -> Mob {
        Mob { position: Point { x: x, y: y }, glyph: glyph }
    }
}

impl Updates for Mob {
    fn update(&mut self, kepress: &tcod::KeyState, game: &Game) {
        let btwn = Range::new(0,3);
        let mut rng = std::rand::thread_rng();
        
        let offset_x = btwn.ind_sample(&mut rng) - 1;
        match game.window_bounds.contains(self.position.offset_x(offset_x)) {
            DoesContain => self.position = self.position.offset_x(offset_x),
            DoesNotContain => {}
        }

        let offset_y = btwn.ind_sample(&mut rng) - 1;
        match game.window_bounds.contains(self.position.offset_y(offset_y)) {
            DoesContain => self.position = self.position.offset_y(offset_y),
            DoesNotContain => {}
        }
    }

    fn render(&self, console: &mut Console) {
        console.put_char(self.position.x, self.position.y, self.glyph, BackgroundFlag::Set);
    }
}

