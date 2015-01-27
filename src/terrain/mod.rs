use rendering::Color;

pub mod random;

#[derive(Copy, PartialEq)]
pub enum TileType {
    CaveFloor,
    CaveWall,
    OutOfBounds
}

#[derive(Copy)]
pub struct Tile {
    pub glyph: char,
    pub fore_color: Color,
    pub back_color: Color,
    pub tile_type: TileType
}

impl Tile {
    pub fn new(tile_type: TileType) -> Tile {
        match tile_type {
            TileType::CaveWall => { Tile { glyph: 177u8 as char, fore_color: Color::LightGrey, back_color: Color::Black, tile_type: tile_type } },
            TileType::CaveFloor => { Tile { glyph: 250u8 as char, fore_color: Color::Grey, back_color: Color::Black, tile_type: tile_type } },
            TileType::OutOfBounds => { Tile { glyph: 'x', fore_color: Color::Red, back_color: Color::Black, tile_type: tile_type } }
        }
    }
}
