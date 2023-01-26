use opengl_graphics::{GlGraphics, OpenGL};
use piston::{RenderArgs, UpdateArgs};

use crate::simulation::simulation::Simulation;
use crate::utils::pair::Pair;

struct Shift {
    current: Pair<f64>,
    old: Pair<f64>,
    future: Pair<f64>,
    t: f64,
}

impl Shift {

    fn update(&mut self, dt: f64) {

        const SPEED: f64 = 5.0;

        self.current = self.old * (1.0 - self.t) + self.future * self.t;
        self.t = (self.t + dt * SPEED).min(1.0);
    }

    fn new(base_shift: Pair<f64>) -> Self {
        Self {
            current: base_shift,
            old: base_shift,
            future: base_shift,
            t: 1.0,
        }
    }
}

pub struct Painter {
    gl: GlGraphics,
    sim: Simulation,
    shift: Shift,
}

impl Painter {
    pub fn new(opengl: OpenGL, sim: Simulation, size: [u32; 2]) -> Self {
        Painter {
            gl: GlGraphics::new(opengl),
            sim,
            shift: Shift::new(Into::<Pair<f64>>::into(size) * 0.5),
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
                    .trans(
                        self.shift.current.x + planet.coords.0,
                        self.shift.current.y + planet.coords.1,
                    )
                    .trans(-planet.r, -planet.r);
                ellipse(WHITE, square, transform, gl);
            }
        })
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.sim.update(args.dt);
        self.shift.update(args.dt);
    }

    pub fn shift_update(&mut self, new_shift: (f64, f64)) {
        self.shift.old = self.shift.current;
        self.shift.future = new_shift.into();
        self.shift.t = 0.0;
    }
}
