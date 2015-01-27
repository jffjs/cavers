use geom::{Bounds, Point};
use rendering::renderer::RenderingComponent;
use rendering::Color;
use terrain::tile::Tile;

// TODO: refactor this module to be more about keep track of objects in game world
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
                                                  Color::Black);
            }
        }
    }
}
