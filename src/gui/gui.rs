extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, GenericEvent, MouseButton, MouseCursorEvent};

use crate::simulation::simulation::Simulation;

use super::painter::Painter;

pub struct GUI {
    window: Window,
    painter: Painter,
    last_mouse_pos: (f64, f64),
}

impl GUI {
    pub fn new(opengl: OpenGL, title: &str, size: [u32; 2], sim: Simulation) -> Self {
        GUI {
            window: WindowSettings::new(title, size)
                .graphics_api(opengl)
                .exit_on_esc(true)
                .build()
                .unwrap(),
            painter: Painter::new(opengl, sim, size),
            last_mouse_pos: (0.0, 0.0),
        }
    }

    pub fn start(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(event) = events.next(&mut self.window) {
            self.event(&event);

            if let Some(args) = event.render_args() {
                self.painter.render(&args);
            }

            if let Some(args) = event.update_args() {
                self.painter.update(&args);
            }

            if let Some(args) = event.mouse_cursor_args() {
                self.last_mouse_pos = (args[0], args[1]);
            }
        }
    }

    pub fn event<E: GenericEvent>(&mut self, event: &E) {
        if let Some(args) = event.mouse_cursor_args() {
            self.last_mouse_pos = (args[0], args[1]);
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            self.painter.shift_update(self.last_mouse_pos);
        }
    }
}
