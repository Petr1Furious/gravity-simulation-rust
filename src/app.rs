use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use opengl_graphics::OpenGL;
use piston::{Events, EventSettings};

use crate::gui::gui::GUI;
use crate::simulation::planet::Planet;
use crate::simulation::simulation::Simulation;
use crate::simulation::planet_system::PlanetSystem;

pub struct App {
    gui: GUI,
    sim: Simulation,
}

impl App {
    pub fn new() -> Self {
        Self {
            gui: GUI::new(OpenGL::V4_5, "Gravity simulation", [640, 480]),
            sim: Simulation::new(
                PlanetSystem {
                    planets: vec![
                        Planet::new((-150.0, 0.0), 20.0, (0.0, 200.0), 1e16, "".to_string()),
                        Planet::new((150.0, 0.0), 20.0, (-0.0, -200.0), 1e16, "".to_string()),
                        Planet::new((0.0, 0.0), 30.0, (0.0, 0.0), 2e17, "".to_string()),
                    ]
                }
            ),
        }
    }
    
    pub fn start(mut self) {
        let planet_system = Arc::new(Mutex::new(self.sim.get_planet_system().clone()));
        let planet_system_simulation = planet_system.clone();
        
        let _simulation_thread = thread::spawn(move || {
            let mut last_update_time = Instant::now();
            loop {
                self.sim.update(last_update_time.elapsed().as_secs_f64());
                last_update_time = Instant::now();

                let mut planet_system_simulation = planet_system_simulation.lock().unwrap();
                *planet_system_simulation = self.sim.get_planet_system().clone();
            }
        });

        let mut events = Events::new(EventSettings::new());

        while let Some(event) = events.next(&mut self.gui.window) {
            let planet_system = planet_system.lock().unwrap();
            self.gui.event(&event, &*planet_system);
        }
    }
}
