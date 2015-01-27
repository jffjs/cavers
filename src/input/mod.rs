pub mod keyboard {
    extern crate tcod;
    use self::tcod::{Console, KeyState};

    pub fn wait_for_keypress() -> KeyState {
        Console::wait_for_keypress(true)
    }
}
