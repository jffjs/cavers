#![feature(box_syntax)]
extern crate core;
extern crate tcod;
extern crate cavers;

use core::ops::DerefMut;
use tcod::{Console};
use cavers::game::Game;
use cavers::input::keyboard;
use cavers::input::keyboard::{KeyCode};
use cavers::input::keyboard::Key::{Special};

fn main() {
    let mut game = Game::new();
    
    game.render();
    while !(Console::window_closed() || game.exit) {
        // wait for user input
        let keypress = keyboard::wait_for_keypress();

        // update game state
        match keypress.key {
            Special(KeyCode::Escape) => game.exit = true,
            _ => {}
        }

        game.move_info.borrow_mut().deref_mut().last_keypress = Some(keypress);
        game.update();

        // render
        game.render();
    }
}


// TODO: macro for tcod key mappings
// TODO: combat
// TODO: make bounds and window sizes dynamic, ratio based
