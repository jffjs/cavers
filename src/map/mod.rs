use core::ops::{Deref, DerefMut};
use std::cmp;
use std::cell::RefCell;
use std::rc::Rc;
use game::MoveInfo;
use geom::{Bounds, Point};
use geom::Contains::{DoesContain, DoesNotContain};
use input::keyboard::{KeyCode, KeyboardInput};
use input ::keyboard::Key::{Special};
use rendering::renderer::RenderingComponent;
use terrain::{Tile, TileType};

pub struct Map<'a> {
    pub bounds: Bounds,
    pub view_origin: Point,
    pub tiles: Vec<Vec<Box<Tile>>>
}

impl<'a> Map<'a> {
    pub fn new(bounds: Bounds, tiles: Vec<Vec<Box<Tile>>>) -> Map<'a> {
        Map { bounds: bounds, view_origin: Point { x: 0, y: 0}, tiles: tiles }
    }

    pub fn scroll_x(&self, center: Point, window_bounds: &Bounds) -> i32 {
        let screen_width = window_bounds.width();
        let world_width = self.bounds.width();
        let center_x = center.x as i32;
        cmp::max(0, cmp::min(center_x - screen_width / 2, world_width - screen_width))
    }

    pub fn scroll_y(&self, center: Point, window_bounds: &Bounds) -> i32 {
        let screen_height = window_bounds.height();
        let world_height = self.bounds.height();
        let center_y = center.y as i32;
        cmp::max(0, cmp::min(center_y - screen_height / 2, world_height - screen_height))
    }

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

    pub fn render(&self, window_bounds: Bounds, move_info: Rc<RefCell<MoveInfo>>, renderer: &mut Box<RenderingComponent>) {
        let center = move_info.borrow().deref().player_pos;
        let start_x = self.scroll_x(center, &window_bounds);
        let start_y = self.scroll_y(center, &window_bounds);

        move_info.borrow_mut().deref_mut().view_origin = Point { x: start_x, y: start_y };
        for x in (0 .. window_bounds.width()) {
            for y in (0 .. window_bounds.height()) {
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
