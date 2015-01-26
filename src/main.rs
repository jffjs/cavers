#![feature(box_syntax)]
extern crate tcod;
extern crate cavers;

use tcod::{Console, KeyCode};
use tcod::Key::Special;
use cavers::entity::character::Character;
use cavers::entity::mob::Mob;
use cavers::entity::traits::Updates;
use cavers::game::Game;

fn main() {
    let mut game = Game::new();
    let mut c = Character::new(40, 25, '@');
    
    let d = box Mob::new(10, 10, 'd') as Box<Updates>;
    let ct = box Mob::new(40, 25, 'c') as Box<Updates>;
    let mut mobs: Vec<Box<Updates>> = vec![d, ct];
    
    game.render(&mobs, &c);
    while !(Console::window_closed() || game.exit) {
        // wait for user input
        let keypress = Console::wait_for_keypress(true);

        // update game state
        match keypress.key {
            Special(KeyCode::Escape) => game.exit = true,
            _ => {}
        }
        game.update(&mut mobs, &mut c, &keypress);

        // render
        game.render(&mobs, &c);
    }
}
