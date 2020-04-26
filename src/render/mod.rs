use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use vecmath;
use vecmath::Vector2;

use crate::model::World;
use crate::physics::motion::Position;
use crate::render::camera::{Camera, CameraSystem};
use crate::render::circle::CircleSystem;
use crate::render::renderable::Renderable;

pub mod camera;
pub mod circle;
pub mod renderable;

pub struct Renderer {
    pub zoom: f64,
    pub gl: GlGraphics,
    camera_system: CameraSystem,
    circle_system: CircleSystem,
}

pub struct Projector {
    pub zoom: f64,
}

impl Projector {
    pub fn project(&self, coords: &Vector2<f64>) -> Vector2<f64> {
        vecmath::vec2_scale(*coords, self.zoom)
    }
}

const BACK: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

impl Renderer {
    pub fn new(gl: GlGraphics) -> Renderer {
        Renderer {
            zoom: 2.5e-4,
            gl,
            camera_system: CameraSystem::new(Camera::new(2.5e-4)),
            circle_system: CircleSystem::new(),
        }
    }

    fn projector(&self) -> Projector {
        Projector { zoom: self.zoom }
    }

    pub fn render(&mut self, args: RenderArgs, world: &mut World) {
        let projector = self.projector();
        let mut gl = &mut self.gl;

        let mut context = gl.draw_begin(args.viewport());
        context = self.camera_system.update(context, world, args);

        // clear the screen
        graphics::clear(BACK, gl);

        self.circle_system.update(world, context, gl);

        let mut center = GeoCenter::new(&world);
        center.render_all(&projector, &mut context, &mut gl);

        gl.draw_end();
    }
}

struct GeoCenter {
    position: Position,
}

impl GeoCenter {
    fn new(world: &World) -> GeoCenter {
        let mut position = world
            .planets
            .iter()
            .map(|p| p.motion.position)
            .fold([0.0, 0.0], |a, p| vecmath::vec2_add(a, p));
        position = vecmath::vec2_scale(position, 1.0 / world.planets.len() as f64);
        GeoCenter { position }
    }
}

impl Renderable for GeoCenter {
    fn render(&mut self, projector: &Projector, context: &mut Context, gl: &mut GlGraphics) {
        let position: Position = projector.project(&self.position);
        let bound = graphics::rectangle::centered_square(position[0], position[1], 10.0);
        graphics::ellipse([1.0, 0.0, 0.0, 1.0], bound, context.transform, gl);
    }
}
