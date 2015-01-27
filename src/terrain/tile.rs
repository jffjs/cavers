use rendering::Color;

#[derive(Copy, PartialEq)]
pub enum TileType {
    CaveFloor,
    CaveWall,
    OutOfBounds
}

#[derive(Copy)]
pub struct Tile {
    pub glyph: char,
    pub color: Color,
    pub tile_type: TileType
}

impl Tile {
    pub fn new(tile_type: TileType) -> Tile {
        match tile_type {
            TileType::CaveWall => { Tile { glyph: 177u8 as char, color: Color::LightGrey, tile_type: tile_type } },
            TileType::CaveFloor => { Tile { glyph: 250u8 as char, color: Color::Grey, tile_type: tile_type } },
            TileType::OutOfBounds => { Tile { glyph: 'x', color: Color::Red, tile_type: tile_type } }
        }
    }
}
