extern crate tcod;
use tcod::{Console, BackgroundFlag};
use tcod::Key::Special;
use tcod::KeyCode;

fn render(con: &mut Console) {
    con.clear();
    con.put_char(40, 25, '@', BackgroundFlag::Set);
    Console::flush();
}

fn main() {
    let mut con = Console::init_root(80, 50, "cavers", false);
    let mut exit = false;
    render(&mut con);
    while !(Console::window_closed() || exit) {
        // wait for user input
        let keypress = Console::wait_for_keypress(true);

        // update game state
        match keypress.key {
            Special(KeyCode::Escape) => exit = true,
            _ => {}
        }

        // render
        render(&mut con);
    }
}
