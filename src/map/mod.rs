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
        let x = p.x as usize;
        let y = p.y as usize;
        let ref tile = self.tiles[x][y];
        if tile.solid {
            // find closest non-solid tile
        }
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

// pub struct ActorMatrix<'a> {
//     pub matrix: Vec<Vec<Vec<Box<Actor<'a>>>>>,
//     pub x_len: i32,
//     pub y_len: i32
// }

// impl<'a> ActorMatrix<'a> {
//     pub fn new(bounds: Bounds) -> ActorMatrix<'a> {
//         let x_len = bounds.max.x + 1;
//         let y_len = bounds.max.y + 1;
        
//         let mut matrix: Vec<Vec<Vec<Box<Actor>>>> = vec![];
//         for _ in (0..x_len) {
//             let mut x_vec: Vec<Vec<Box<Actor>>> = vec![];
//             for _ in (0..y_len) {
//                 let y_vec: Vec<Box<Actor>> = vec![];
//                 x_vec.push(y_vec);
//             }
//             matrix.push(x_vec);
//         }

//         ActorMatrix { matrix: matrix, x_len: x_len, y_len: y_len }
//     }
// }
