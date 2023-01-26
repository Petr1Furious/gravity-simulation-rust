use opengl_graphics::OpenGL;

mod gui;
mod utils;
use gui::gui::GUI;

mod simulation;
use simulation::planet::Planet;
use simulation::simulation::Simulation;

pub fn main() {
    let sim = Simulation {
        planets: vec![
            Planet::new((-150.0, 0.0), 20.0, (0.0, 200.0), 1e16, "".to_string()),
            Planet::new((150.0, 0.0), 20.0, (-0.0, -200.0), 1e16, "".to_string()),
            Planet::new((0.0, 0.0), 30.0, (0.0, 0.0), 2e17, "".to_string()),
        ],
    };

    let mut gui = GUI::new(OpenGL::V4_5, "Gravity simulation", [640, 480], sim);
    gui.start();
}
