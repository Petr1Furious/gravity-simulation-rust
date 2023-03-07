use opengl_graphics::{GlGraphics, OpenGL};
use piston::RenderArgs;

use crate::simulation::planet_system::PlanetSystem;
use crate::utils::pair::Pair;

pub struct Painter {
    gl: GlGraphics,
    shift: Pair<f64>,
}

impl Painter {
    pub fn new(opengl: OpenGL, size: [u32; 2]) -> Self {
        Painter {
            gl: GlGraphics::new(opengl),
            shift: Into::<Pair<f64>>::into(size) * 0.5,
        }
    }

    pub fn render(&mut self, args: &RenderArgs, planet_system: &PlanetSystem) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            for planet in planet_system.planets.iter() {
                let square = rectangle::square(0.0, 0.0, 2.0 * planet.r);
                let pos = self.shift + planet.coords.into() + (-planet.r, -planet.r).into();
                let transform = c
                    .transform
                    .trans(
                        pos.x, pos.y
                    );
                ellipse(WHITE, square, transform, gl);
            }
        })
    }

    pub fn shift_update(&mut self, new_shift: Pair<f64>) {
        self.shift = new_shift.into();
    }

    pub fn get_shift(&self) -> Pair<f64> {
        self.shift
    }
}
