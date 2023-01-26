use opengl_graphics::{GlGraphics, OpenGL};
use piston::{RenderArgs, UpdateArgs};

use crate::simulation::simulation::Simulation;
use crate::utils::pair::Pair;

pub struct Painter {
    gl: GlGraphics,
    sim: Simulation,
    shift: Pair<f64>,
    planet_offset_index: Option<usize>,
}

impl Painter {
    pub fn new(opengl: OpenGL, sim: Simulation, size: [u32; 2]) -> Self {
        Painter {
            gl: GlGraphics::new(opengl),
            sim,
            shift: Into::<Pair<f64>>::into(size) * 0.5,
            planet_offset_index: None,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let planet_offset = self.planet_coords(self.planet_offset_index);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            for planet in self.sim.get_planets() {
                let square = rectangle::square(0.0, 0.0, 2.0 * planet.r);
                let pos = self.shift + planet.coords.into() + (-planet.r, -planet.r).into() - planet_offset;
                let transform = c
                    .transform
                    .trans(
                        pos.x, pos.y
                    );
                ellipse(WHITE, square, transform, gl);
            }
        })
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.sim.update(args.dt);
    }

    pub fn shift_update(&mut self, new_shift: Pair<f64>) {
        self.shift = new_shift.into();
    }

    pub fn get_shift(&self) -> Pair<f64> {
        self.shift
    }

    pub fn get_object_at_point(&mut self, point: Pair<f64>) -> Option<usize> {
        self.sim.get_planet_at_pos(point - self.shift + self.planet_coords(self.planet_offset_index))
    }

    fn planet_coords(&self, index: Option<usize>) -> Pair<f64> {
        match index {
            None => Pair::default(),
            Some(index) => self.sim.planets[index].coords.into()
        }
    }

    pub fn planet_offset_update(&mut self, index: usize) {
        let old_offset = self.planet_coords(self.planet_offset_index);
        self.planet_offset_index = Some(index);
        let new_offset = self.planet_coords(self.planet_offset_index);

        self.shift_update(self.get_shift() + new_offset - old_offset);
    }
}
