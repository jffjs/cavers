extern crate tcod;

use std::rand;
use geom::{Bounds, Point, Contains};
use rendering::rendering_component::RenderingComponent;
use terrain;

// TODO: refactor this module to be more about keep track of objects in game world
#[derive(Copy)]
pub struct Tile {
    pub glyph: char,
    pub color: tcod::Color,
    pub kind: TileType
}

impl Tile {
    pub fn cave_wall() -> Tile {
        Tile { glyph: 177u8 as char, color: tcod::colors::light_grey, kind: TileType::CaveWall }
    }

    pub fn cave_floor() -> Tile {
        Tile { glyph: 250u8 as char, color: tcod::colors::grey, kind: TileType::CaveFloor }
    }

    pub fn out_of_bounds() -> Tile {
        Tile { glyph: 'x', color: tcod::colors::red, kind: TileType::OutOfBounds }
    }
}

#[derive(Copy, PartialEq)]
pub enum TileType {
    CaveFloor,
    CaveWall,
    OutOfBounds
}

pub struct Map<'a> {
    pub bounds: Bounds,
    pub tiles: Vec<Vec<Box<Tile>>>
}

// TODO: move generation to terrain module

impl<'a> Map<'a> {
    pub fn new(bounds: Bounds, tiles: Vec<Vec<Box<Tile>>>) -> Map<'a> {
        Map { bounds: bounds, tiles: tiles }
    }

    pub fn render(&self, renderer: &mut Box<RenderingComponent>) {
        for x in 0..80 {
            for y in 0..50 {
                let ref tile = self.tiles[x][y];
                let pos = Point { x: x as i32, y: y as i32};
                renderer.render_object_with_color(&pos,
                                                  tile.glyph,
                                                  tile.color,
                                                  tcod::colors::black);
            }
        }
    }
}
