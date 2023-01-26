extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod simulation;
use simulation::planet::Planet;
use simulation::simulation::Simulation;

pub struct App {
    gl: GlGraphics,
    pub simulation: Simulation,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            for planet in self.simulation.get_planets() {
                let square = rectangle::square(0.0, 0.0, 2.0 * planet.r);
                let transform = c.transform.trans(planet.coords.0, planet.coords.1).trans(-planet.r, -planet.r);
                ellipse(WHITE, square, transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.simulation.update(args.dt);
    }
}

pub fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: Window = WindowSettings::new("Gravity simulation", [1000, 1000])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        simulation: Simulation::new(),
    };
    
    app.simulation.planets.push(Planet::new((500.0, 500.0), 100.0, (0.0, 0.0), 1e17, "".to_string()));
    app.simulation.planets.push(Planet::new((750.0, 500.0), 20.0, (0.0, 150.0), 3e15, "".to_string()));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
