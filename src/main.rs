#![feature(box_syntax)]
extern crate tcod;
extern crate cavers;

use tcod::{Console, KeyCode};
use tcod::Key::Special;
use cavers::actor::Actor;
use cavers::actor::behavior::Behavior;
use cavers::actor::behavior::player::Player;
use cavers::actor::behavior::wanderer::Wanderer;
use cavers::game::Game;
use cavers::rendering::Color;


fn main() {
    let mut game = Game::new();
    let player_behavior = box Player::new(game.window_bounds) as Box<Behavior>;
    let c = box Actor::new(40, 25, '@', Color::White, player_behavior);
    // let mut c = Character::new(40, 25, '@');
    
    let wanderer_behavior = box Wanderer::new(game.window_bounds) as Box<Behavior>;
    let d = box Actor::new(10, 10, 'd', Color::White, wanderer_behavior);
    // let ct = box Actor::new(40, 25, 'c', Color::White, wanderer_behavior);
    // let d = box Mob::new(10, 10, 'd') as Box<Updates>;
    // let ct = box Mob::new(40, 25, 'c') as Box<Updates>;
    // let mut mobs: Vec<Box<Updates>> = vec![d, ct];
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
