extern crate tcod;

pub mod renderer;
pub mod window;

#[derive(Copy)]
pub enum Color {
    Amber,
    DarkAmber,
    Black,
    Grey,
    LightGrey,
    Green,
    DarkGreen,
    DarkerGreen,
    Red,
    White,
}

impl Color {
    pub fn value(self) -> tcod::Color {
        match self {
            Color::Amber => { tcod::colors::amber },
            Color::DarkAmber => { tcod::colors::dark_amber },
            Color::Black => { tcod::colors::black },
            Color::Grey => { tcod::colors::grey },
            Color::LightGrey => { tcod::colors::light_grey },
            Color::Green => { tcod::colors::green },
            Color::DarkGreen => { tcod::colors::dark_green },
            Color::DarkerGreen => { tcod::colors::darker_green },
            Color::Red => { tcod::colors::red },
            Color::White => { tcod::colors::white }
        }
    }
}
