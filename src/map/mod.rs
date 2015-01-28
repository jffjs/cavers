use actor::Actor;
use geom::{Bounds, Point};
use geom::Contains::{DoesContain, DoesNotContain};
use rendering::renderer::RenderingComponent;
use terrain::Tile;

pub struct Map<'a> {
    pub bounds: Bounds,
    pub tiles: Vec<Vec<Box<Tile>>>,
    pub actors: Vec<Box<Actor<'a>>>,
    pub player: Box<Actor<'a>>
}

impl<'a> Map<'a> {
    pub fn new(bounds: Bounds, tiles: Vec<Vec<Box<Tile>>>, player: Box<Actor<'a>>) -> Map<'a> {
        let mut map = Map { bounds: bounds, tiles: tiles, actors: vec![], player: player };
        map.place_player();
        map
    }

    fn find_empty_tile(&self, pos: &Point, tile: &Tile) -> Point {
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

    fn place_player(&mut self) {
        let p = self.player.position;
        let x = p.x as usize;
        let y = p.y as usize;
        let tile = *self.tiles[x][y];
        let player_pos = self.find_empty_tile(&p, &tile);
        self.player.position = player_pos;
    }

    pub fn place_actor(&mut self, mut actor: Box<Actor<'a>>) -> Point {
        let p = actor.position;
        let x = p.x as usize;
        let y = p.y as usize;
        let tile = *self.tiles[x][y];
        let new_actor_pos = self.find_empty_tile(&p, &tile);
        actor.position = new_actor_pos;
        self.actors.push(actor);
        new_actor_pos
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
