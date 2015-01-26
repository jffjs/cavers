use std::rand;
use geom::{Bounds, Point, Contains};
use terrain::tile::{Tile, TileType};

pub fn cave(bounds: Bounds, smooth: i32) -> Vec<Vec<Box<Tile>>> {
    let max_x = bounds.max.x + 1;
    let max_y = bounds.max.y + 1;
    let mut tiles: Vec<Vec<Box<Tile>>> = Vec::new();
    let cave_floor = Tile::new(TileType::CaveFloor);
    let cave_wall = Tile::new(TileType::CaveWall);
    for _ in (0..max_x) {
        let mut col: Vec<Box<Tile>> = Vec::new();
        for _ in (0..max_y) {
            let tile = if rand::random() {
                box cave_floor
            } else {
                box cave_wall
            };
            col.push(tile);
        }
        tiles.push(col);
    }

    for _ in (0..smooth) {
        let mut tiles2: Vec<Vec<Box<Tile>>> = Vec::new();
        for x in (0..max_x) {
            let mut col: Vec<Box<Tile>> = Vec::new();
            for y in (0..max_y) {
                let mut walls = 0;
                let mut floors = 0;
                let p = Point { x: x, y: y};
                let adjacent_tiles = p.adjacent();
                if tiles[x as usize][y as usize].tile_type == TileType::CaveFloor {
                    floors += 1;
                } else {
                    walls += 1;
                }

                for t in adjacent_tiles.iter() {
                    match bounds.contains(*t) {
                        Contains::DoesContain => {
                            if tiles[t.x as usize][t.y as usize].tile_type == TileType::CaveFloor {
                                floors += 1;
                            } else {
                                walls += 1;
                            }
                        },
                        Contains::DoesNotContain => {}
                    }
                }

                if floors >= walls {
                    col.push(box cave_floor);
                } else {
                    col.push(box cave_wall);
                }
            } 
            tiles2.push(col);
        } 
        tiles = tiles2;
    }

    tiles
}
