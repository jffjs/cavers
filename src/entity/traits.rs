extern crate tcod;
use game::Game;
use rendering::rendering_component::RenderingComponent;

pub trait Updates {
    fn update(&mut self, &Game);
    fn render(&self, &mut Box<RenderingComponent>);
}
