use opengl_graphics::{OpenGL};

mod gui;
use gui::gui::GUI;

mod simulation;
use simulation::planet::Planet;
use simulation::simulation::Simulation;

pub fn main() {
    let opengl = OpenGL::V4_5;

    let sim = Simulation {
        planets: vec![
            Planet::new((500.0, 500.0), 30.0, (0.0, 0.0), 5e17, "".to_string()),
            Planet::new((750.0, 500.0), 20.0, (0.0, 300.0), 1e16, "".to_string()),
            Planet::new((250.0, 500.0), 25.0, (0.0, -300.0), 2e16, "".to_string()),
        ],
    };

    let mut gui = GUI::new(opengl, "Gravity simulation", [1000, 1000], sim);
    gui.start();
}
