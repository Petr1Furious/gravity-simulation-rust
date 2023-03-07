extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::window::WindowSettings;
use piston::{Button, GenericEvent, MouseButton};

use crate::simulation::planet_system::PlanetSystem;
use crate::utils::pair::Pair;

use super::painter::Painter;

enum ShiftType {
    Ilde,
    AtCursor,
}

pub struct GUI {
    pub window: Window,
    painter: Painter,
    last_mouse_pos: Pair<f64>,
    last_shift_pos: Pair<f64>,
    last_tap_pos: Pair<f64>,
    shift: ShiftType,
}

impl GUI {
    pub fn new(opengl: OpenGL, title: &str, size: [u32; 2]) -> Self {
        GUI {
            window: WindowSettings::new(title, size)
                .graphics_api(opengl)
                .exit_on_esc(true)
                .build()
                .unwrap(),
            painter: Painter::new(opengl, size),
            last_mouse_pos: Pair::default(),
            last_shift_pos: Pair::default(),
            last_tap_pos: Pair::default(),
            shift: ShiftType::Ilde,
        }
    }

    pub fn event<E: GenericEvent>(&mut self, event: &E, planet_system: &PlanetSystem) {
        use std::ops::{Add, Sub};

        if let Some(args) = event.mouse_cursor_args() {
            self.last_mouse_pos = args.into();
        }

        if let ShiftType::AtCursor = self.shift {
            self.painter.shift_update(self
                .last_shift_pos
                .add(self.last_mouse_pos)
                .sub(self.last_tap_pos));
        }
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            dbg!(self.last_mouse_pos);
            self.shift = ShiftType::AtCursor;
            self.last_shift_pos = self.painter.get_shift();
            self.last_tap_pos = self.last_mouse_pos;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.release_args() {
            if let ShiftType::AtCursor = self.shift {
                self.shift = ShiftType::Ilde;
            }
            self.last_shift_pos = self.painter.get_shift();
        }

        if let Some(args) = event.render_args() {
            self.painter.render(&args, planet_system);
        }
    }
}
