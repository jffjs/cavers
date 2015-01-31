use geom::Contains::{DoesContain, DoesNotContain};
use core::num::Float;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {x: x, y: y }
    }

    pub fn distance_to(&self, p: Point) -> f32 {
        let x1 = self.x as f32;
        let y1 = self.y as f32;
        let x2 = p.x as f32;
        let y2 = p.y as f32;
        let tmp = (x2-x1) * (x2-x1) + (y2-y1) * (y2-y1);
        tmp.sqrt()
    }
    pub fn offset_x(&self, offset: i32) -> Point {
        Point { x: self.x + offset, y: self.y }
    }

    pub fn offset_y(&self, offset: i32) -> Point {
        Point { x: self.x, y: self.y + offset }
    }

    pub fn offset(&self, offset: &Point) -> Point {
        Point { x: self.x + offset.x, y: self.y + offset.y }
    }

    pub fn adjacent(&self) -> Vec<Point> {
        let offsets = vec![Point::new(-1, -1), Point::new(0, -1), Point::new(1, -1),
                           Point::new(-1, 0), Point::new(1, 0),
                           Point::new(-1, 1), Point::new(0, 1), Point::new(1, 1)];
        
        offsets.iter().map(|o| self.offset(o)).collect::<Vec<Point>>()
    }

}

#[derive(Copy)]
pub enum Contains {
    DoesContain,
    DoesNotContain
}

#[derive(Copy, Clone)]
pub struct Bounds {
    pub min: Point,
    pub max: Point
}

impl Bounds {
    pub fn contains(&self, point: Point) -> Contains {
        if
            point.x >= self.min.x &&
            point.x <= self.max.x &&
            point.y >= self.min.y &&
            point.y <= self.max.y {
                DoesContain
            } else {
                DoesNotContain
            }
    }
}
