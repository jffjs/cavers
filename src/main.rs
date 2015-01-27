#![feature(box_syntax)]
extern crate tcod;
extern crate cavers;

use tcod::{Console};
use cavers::actor::Actor;
use cavers::game::Game;
use cavers::input::keyboard;
use cavers::input::keyboard::{KeyCode};
use cavers::input::keyboard::Key::{Special};

fn main() {
    let mut game = Game::new();
    // let c = box Actor::player(40, 25, game.window_bounds);
    // let d = box Actor::dog(10, 10, game.window_bounds);
    // let mut mobs = vec![c, d];
    
    game.render();
    while !(Console::window_closed() || game.exit) {
        // wait for user input
        let keypress = keyboard::wait_for_keypress();

        // update game state
        match keypress.key {
            Special(KeyCode::Escape) => game.exit = true,
            _ => {}
        }
        game.update(&keypress);

        // render
        game.render();
    }
}

// TODO: make map aware of actors
// TODO: make map scrollable
// TODO: composable behaviors
