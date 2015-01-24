#![feature(box_syntax)]
extern crate tcod;
extern crate cavers;

use cavers::entity::character::Character;
use cavers::entity::mob::Mob;
use cavers::entity::traits::Updates;
use cavers::game::Game;
use cavers::geom::{Point, Bounds};
use tcod::{Console, KeyCode};
use tcod::Key::Special;

fn render(con: &mut Console, objs: &Vec<Box<Updates>>) {
    con.clear();
    for i in objs.iter() {
        i.render(con);
    }
    Console::flush();
}

fn update(objs: &mut Vec<Box<Updates>>, keypress: &tcod::KeyState, game: &Game) {
    for i in objs.iter_mut() {
        i.update(keypress, game);
    }
}

fn main() {
    let mut game = Game {
        exit: false,
        window_bounds: Bounds {
            min: Point { x: 0, y: 0 },
            max: Point { x: 79, y: 49 }
        }
    };
    let mut con = Console::init_root(game.window_bounds.max.x + 1, game.window_bounds.max.y + 1, "cavers", false);
    let c = box Character::new(40, 25, '@') as Box<Updates>;
    
    let d = box Mob::new(10, 10, 'd') as Box<Updates>;
    let mut objs: Vec<Box<Updates>> = vec![c, d];
    
    render(&mut con, &objs);
    while !(Console::window_closed() || game.exit) {
        // wait for user input
        let keypress = Console::wait_for_keypress(true);

        // update game state
        match keypress.key {
            Special(KeyCode::Escape) => game.exit = true,
            _ => {}
        }
        update(&mut objs, &keypress, &game);

        // render
        render(&mut con, &objs);
    }
}
