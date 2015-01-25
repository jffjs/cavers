extern crate tcod;

#[derive(Copy)]
pub struct Tile {
    pub glyph: char,
    pub color: tcod::Color
}

impl Tile {
    pub fn cave_wall() -> Tile {
        Tile { glyph: 177u8 as char, color: tcod::colors::light_grey }
    }

    pub fn cave_floor() -> Tile {
        Tile { glyph: 250u8 as char, color: tcod::colors::grey }
    }

    pub fn out_of_bounds() -> Tile {
        Tile { glyph: 'x', color: tcod::colors::red }
    }
}

pub struct Map<'a> {
    pub tiles: Vec<&'a Tile>
}
