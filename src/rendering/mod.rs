extern crate tcod;

pub mod renderer;

#[derive(Copy)]
pub enum Color {
    Amber,
    DarkAmber,
    Black,
    Grey,
    LightGrey,
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
            Color::Red => { tcod::colors::red },
            Color::White => { tcod::colors::white }
        }
    }
}
