use std::cmp;
use geom::{Bounds, Point};
use geom::Contains::{DoesContain, DoesNotContain};
use input::keyboard::{KeyCode, KeyboardInput};
use input ::keyboard::Key::{Special};
use rendering::renderer::RenderingComponent;
use terrain::{Tile, TileType};

pub struct Map<'a> {
    pub bounds: Bounds,
    pub window_bounds: Bounds,
    pub view_origin: Point,
    pub tiles: Vec<Vec<Box<Tile>>>
}

impl<'a> Map<'a> {
    pub fn new(bounds: Bounds, window_bounds: Bounds, tiles: Vec<Vec<Box<Tile>>>) -> Map<'a> {
        Map { bounds: bounds, window_bounds: window_bounds, view_origin: Point { x: 0, y: 0}, tiles: tiles }
    }

    pub fn scroll_x(&self, center: Point) -> i32 {
        let screen_width = self.window_bounds.max.x + 1 as i32;
        let world_width = self.bounds.max.x + 1 as i32;
        let center_x = center.x as i32;
        cmp::max(0, cmp::min(center_x - screen_width / 2, world_width - screen_width))
    }

    pub fn scroll_y(&self, center: Point) -> i32 {
        let screen_height = self.window_bounds.max.y + 1 as i32;
        let world_height = self.bounds.max.y + 1 as i32;
        let center_y = center.y as i32;
        cmp::max(0, cmp::min(center_y - screen_height / 2, world_height - screen_height))
    }

    // pub fn scroll_by(&mut self, offset: Point) {
    //     let world_width = self.bounds.max.x + 1 as i32;
    //     let world_height = self.bounds.max.y + 1 as i32;
    //     self.view_center = Point {
    //         x: cmp::max(0, cmp::min(self.view_center.x + offset.x, world_width - 1)),
    //         y: cmp::max(0, cmp::min(self.view_center.y + offset.y, world_height -1))
    //     };
    // }

    pub fn get_tile(&self, p: Point) -> Tile {
        let x = p.x as usize;
        let y = p.y as usize;

        match self.bounds.contains(p) {
            DoesContain => { *self.tiles[x][y] },
            DoesNotContain => { Tile::new(TileType::OutOfBounds) }
        }
    }

    pub fn find_empty_tile(&self, pos: Point) -> Point {
        let x = pos.x as usize;
        let y = pos.y as usize;
        let ref tile = self.tiles[x][y];
        
        if !(tile.solid) {
            return Point { x: pos.x, y: pos.y };
        } else {
            let mut search_q = pos.adjacent();
            while search_q.len() > 0 {
                let p = search_q[0];
                let px = p.x as usize;
                let py = p.y as usize;
                let ref t = self.tiles[px][py];
                // println!("tile: {}, solid: {}, x: {}, y: {}", t.glyph, t.solid, px, py);
                
                if !(t.solid) {
                    // found it!
                    match self.bounds.contains(p) {
                        DoesContain => { return p; },
                        DoesNotContain => { }
                    }
                }
                search_q.remove(0);

                // not found, add this points adjecent points to search queue
                search_q.append(&mut p.adjacent());
            }
            // TODO: what if we can't find an empty tile??
            Point { x: pos.x, y: pos.y }
        }
    }

    pub fn render(&mut self, center: Point, renderer: &mut Box<RenderingComponent>) {
        let start_x = self.scroll_x(center);
        let start_y = self.scroll_y(center);
        self.view_origin = Point { x: start_x, y: start_y };
        for x in (0..renderer.screen_width()) {
            for y in (0..renderer.screen_height()) {
                let tx = x + start_x;
                let ty = y + start_y;
                let tile = self.get_tile(Point { x: tx, y: ty });
                let pos = Point { x: x, y: y};
                renderer.render_object_with_color(&pos,
                                                  tile.glyph,
                                                  tile.fore_color,
                                                  tile.back_color);
            }
        }
    }
}
