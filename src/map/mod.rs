use actor::Actor;
use geom::{Bounds, Point};
use rendering::renderer::RenderingComponent;
use terrain::Tile;

// TODO: refactor this module to be more about keep track of objects in game world
pub struct Map<'a> {
    pub bounds: Bounds,
    pub tiles: Vec<Vec<Box<Tile>>>,
    pub actors: Vec<Box<Actor<'a>>>,
    pub player: Box<Actor<'a>>
}

impl<'a> Map<'a> {
    pub fn new(bounds: Bounds, tiles: Vec<Vec<Box<Tile>>>, player: Box<Actor<'a>>) -> Map<'a> {
        Map { bounds: bounds, tiles: tiles, actors: vec![], player: player }
    }

    pub fn place_actor(&mut self, actor: Box<Actor<'a>>) -> Point {
        let p = actor.position;
        self.actors.push(actor);
        p
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

        for a in self.actors.iter() {
            a.render(renderer);
        }

        self.player.render(renderer);
    }
}
