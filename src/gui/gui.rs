extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

use crate::simulation::simulation::Simulation;

use super::painter::Painter;

pub struct GUI {
    window: Window,
    painter: Painter,
}

impl GUI {
    pub fn new(opengl: OpenGL, title: &str, size: [u32; 2], sim: Simulation) -> Self {
        GUI {
            window: WindowSettings::new(title, size)
                .graphics_api(opengl)
                .exit_on_esc(true)
                .build()
                .unwrap(),
            painter: Painter::new(opengl, sim),
        }
    }

    pub fn start(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.painter.render(&args);
            }

            if let Some(args) = e.update_args() {
                self.painter.update(&args);
            }
        }
    }
}
