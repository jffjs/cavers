use geom::{Point};

pub trait RenderingComponent {
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, &Point, char);
    fn after_render_new_frame(&mut self);
}
