use actor::Actor;
use geom::{Bounds, Point};
use geom::Contains::{DoesContain, DoesNotContain};
use input::keyboard::KeyboardInput;
use rendering::renderer::RenderingComponent;
use terrain::Tile;

pub struct Map<'a> {
    pub bounds: Bounds,
    pub tiles: Vec<Vec<Box<Tile>>>
}

impl<'a> Map<'a> {
    pub fn new(bounds: Bounds, tiles: Vec<Vec<Box<Tile>>>) -> Map<'a> {
        Map { bounds: bounds, tiles: tiles }
    }

    pub fn get_tile(&self, p: Point) -> Tile {
        let x = p.x as usize;
        let y = p.y as usize;

        *self.tiles[x][y]
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

    pub fn render(&self, renderer: &mut Box<RenderingComponent>) {
        for x in (0..self.bounds.max.x + 1) {
            for y in (0..self.bounds.max.y + 1) {
                let ref tile = self.tiles[x as usize][y as usize];
                let pos = Point { x: x as i32, y: y as i32};
                renderer.render_object_with_color(&pos,
                                                  tile.glyph,
                                                  tile.fore_color,
                                                  tile.back_color);
            }
        }
    }
}
