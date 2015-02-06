pub mod keyboard {
    extern crate tcod;
    use self::tcod::{Console, KeyState};

    
    #[derive(Copy)]
    pub enum KeyCode {
        // Arrow Keys
        Up,
        Down,
        Left,
        Right,

        // Special
        Shift,
        Escape,

        // Default
        None
    }

    #[derive(Copy)]
    pub enum Key {
        Printable(char),
        Special(KeyCode)
    }

    #[derive(Copy)]
    pub struct KeyboardInput {
        pub key: Key
    }

    pub fn wait_for_keypress() -> KeyboardInput {
        translate_input(Console::wait_for_keypress(true))
    }

    // TODO: Try making a macro for this to avoid all the repetition
    fn translate_input(key_state: KeyState) -> KeyboardInput {
        let key: Key = if key_state.shift {
            match key_state.key {
                // example purposes for now
                self::tcod::Key::Special(tcod::KeyCode::Number5) => Key::Printable('%'),
                self::tcod::Key::Special(tcod::KeyCode::Number6) => Key::Printable('^'),
                self::tcod::Key::Special(tcod::KeyCode::Number8) => Key::Printable('*'),
                _                                                => Key::Special(KeyCode::None)
            }
        } else {
            match key_state.key {
                self::tcod::Key::Printable('/')                 => Key::Printable('/'),
                self::tcod::Key::Printable('a')                 => Key::Printable('a'),
                self::tcod::Key::Printable('b')                 => Key::Printable('b'),
                self::tcod::Key::Printable('m')                 => Key::Printable('m'),
                self::tcod::Key::Printable('q')                 => Key::Printable('q'),
                self::tcod::Key::Special(tcod::KeyCode::Up)     => Key::Special(KeyCode::Up),
                self::tcod::Key::Special(tcod::KeyCode::Down)   => Key::Special(KeyCode::Down),
                self::tcod::Key::Special(tcod::KeyCode::Left)   => Key::Special(KeyCode::Left),
                self::tcod::Key::Special(tcod::KeyCode::Right)  => Key::Special(KeyCode::Right),
                self::tcod::Key::Special(tcod::KeyCode::Shift)  => Key::Special(KeyCode::Shift),
                self::tcod::Key::Special(tcod::KeyCode::Escape) => Key::Special(KeyCode::Escape),
                _                                               => Key::Special(KeyCode::None)
            } 
        };

        KeyboardInput { key: key }
    }
}
