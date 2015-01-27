#![feature(box_syntax)]
extern crate tcod;
extern crate cavers;

use tcod::{Console, KeyCode};
use tcod::Key::Special;
use cavers::actor::Actor;
use cavers::game::Game;

fn main() {
    let mut game = Game::new();
    let c = box Actor::player(40, 25, game.window_bounds);
    let d = box Actor::dog(10, 10, game.window_bounds);
    let mut mobs = vec![c, d];
    
    game.render(&mobs);
    while !(Console::window_closed() || game.exit) {
        // wait for user input
        let keypress = Console::wait_for_keypress(true);

        // update game state
        match keypress.key {
            Special(KeyCode::Escape) => game.exit = true,
            _ => {}
        }
        game.update(&mut mobs, &keypress);

        // render
        game.render(&mobs);
    }
}

// TODO: refactor input to remove tcod references from all over
// TODO: make map aware of actors
// TODO: make map scrollable
// TODO: composable behaviors
