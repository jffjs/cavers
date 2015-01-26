extern crate tcod;

#[derive(Copy)]
pub enum Color {
    Black,
    Grey,
    LightGrey,
    Red,
    White,
}

impl Color {
    pub fn value(self) -> tcod::Color {
        match self {
            Color::Black => { tcod::colors::black },
            Color::Grey => { tcod::colors::grey },
            Color::LightGrey => { tcod::colors::light_grey },
            Color::Red => { tcod::colors::red },
            Color::White => { tcod::colors::white }
        }
    }
}