use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

use model::planet;

use crate::model::World;
use crate::physics::Universe;
use crate::render::Renderer;

mod model;
mod physics;
mod render;

fn main() {
    let opengl = OpenGL::V4_5;
    let mut window: Window = WindowSettings::new("n-Body Simulation", [1980, 1024])
        .graphics_api(opengl)
        .fullscreen(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // let mars = planet::Planet::new("Mars".to_string(), (0.0, 0.0));
    let mut deimos = planet::Planet::new("Deimos".to_string(), (1.0e6, 0.0));
    let mut phobos = planet::Planet::new("Phobos".to_string(), (-1.0e6, 0.0));
    phobos.motion.velocity = [0.0, 0.15e5];
    deimos.motion.velocity = [0.0, -0.15e5];

    let mut world = World {
        planets: vec![phobos, deimos],
    };
    let mut renderer = Renderer::new(GlGraphics::new(opengl));
    let mut universe = Universe::new();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            renderer.render(args, &mut world);
        }

        if let Some(args) = e.update_args() {
            universe.update(args, &mut world);
        }
    }
}
