use geom::{Point};
use rendering::color::Color;

pub trait RenderingComponent {
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, &Point, char);
    fn render_object_with_color(&mut self, &Point, char, Color, Color);
    fn after_render_new_frame(&mut self);
}
