use opengl_graphics::{GlGraphics, OpenGL};
use piston::{RenderArgs, UpdateArgs};

use crate::simulation::simulation::Simulation;

pub struct Painter {
    gl: GlGraphics,
    sim: Simulation,
}

impl Painter {
    pub fn new(opengl: OpenGL, sim: Simulation) -> Self {
        Painter {
            gl: GlGraphics::new(opengl),
            sim,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            for planet in self.sim.get_planets() {
                let square = rectangle::square(0.0, 0.0, 2.0 * planet.r);
                let transform = c
                    .transform
                    .trans(planet.coords.0, planet.coords.1)
                    .trans(-planet.r, -planet.r);
                ellipse(WHITE, square, transform, gl);
            }
        })
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.sim.update(args.dt);
    }
}
