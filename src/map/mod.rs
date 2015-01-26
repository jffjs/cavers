extern crate tcod;

use std::rand;
use geom::{Bounds, Point, Contains};
use rendering::rendering_component::RenderingComponent;

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

impl<'a> Map<'a> {
    pub fn random_cave(bounds: Bounds) -> Map<'a> {
        let max_x = bounds.max.x + 1;
        let max_y = bounds.max.y + 1;
        let mut tiles: Vec<Vec<Box<Tile>>> = Vec::new();
        for _ in (0..max_x) {
            let mut col: Vec<Box<Tile>> = Vec::new();
            for _ in (0..max_y) {
                let mut tile: Box<Tile> = if rand::random() {
                    box Tile::cave_floor()
                } else {
                    box Tile::cave_wall()
                };
                col.push(tile);
            }
            tiles.push(col);
        }

        for _ in 0..3 {
            let mut tiles2: Vec<Vec<Box<Tile>>> = Vec::new();
            for x in (0..max_x) {
                let mut col: Vec<Box<Tile>> = Vec::new();
                for y in (0..max_y) {
                    let mut walls = 0;
                    let mut floors = 0;
                    let p = Point { x: x, y: y};
                    let adjacent_tiles = p.adjacent();
                    if tiles[x as usize][y as usize].kind == TileType::CaveFloor {
                        floors += 1;
                    } else {
                        walls += 1;
                    }

                    for t in adjacent_tiles.iter() {
                        match bounds.contains(*t) {
                            Contains::DoesContain => {
                                if tiles[t.x as usize][t.y as usize].kind == TileType::CaveFloor {
                                    floors += 1;
                                } else {
                                    walls += 1;
                                }
                            },
                            Contains::DoesNotContain => {}
                        }
                    }

                    if floors >= walls {
                        col.push(box Tile::cave_floor());
                    } else {
                        col.push(box Tile::cave_wall());
                    }
                } 
                tiles2.push(col);
            } 
            tiles = tiles2;
        }

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
