use actor::Actor;
use map::Map;
use rendering::renderer::RenderingComponent;
use rendering::window::Window;

pub trait GameState {
    fn update(&mut self, &mut Vec<Box<Actor>>, &mut Actor);
    fn render(&mut self, &mut Box<RenderingComponent>, &Vec<Box<Actor>>, &Actor, &mut Vec<&mut Box<Window>>);
}

pub struct MovementState<'a>;

impl<'a> MovementState<'a> {
    pub fn new() -> MovementState<'a> {
        MovementState 
    }
}

impl<'a> GameState for MovementState<'a> {
    fn update(&mut self, mobs: &mut Vec<Box<Actor>>, player: &mut Actor) {
        // player.update();
    }

    fn render(&mut self, renderer: &mut Box<RenderingComponent>, mobs: &Vec<Box<Actor>>, player: &Actor, windows: &mut Vec<&mut Box<Window>>) {
        // renderer.before_render_new_frame();
        // map.render(player.position, renderer);
        // for window in windows.iter_mut() {
        //     renderer.attach_window(*window);
        // }
        // for mob in mobs.iter() {
        //     mob.render(map.view_origin, renderer);
        // }
        // player.render(map.view_origin, renderer);
        // renderer.after_render_new_frame();
    }
}
