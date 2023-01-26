extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, GenericEvent, MouseButton};

use crate::simulation::simulation::Simulation;
use crate::utils::pair::Pair;

use super::painter::Painter;

enum ShiftType {
    Ilde,
    AtCursor,
}

pub struct GUI {
    window: Window,
    painter: Painter,
    last_mouse_pos: Pair<f64>,
    last_shift_pos: Pair<f64>,
    last_tap_pos: Pair<f64>,
    shift: ShiftType,
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
            last_mouse_pos: Pair::default(),
            last_shift_pos: Pair::default(),
            last_tap_pos: Pair::default(),
            shift: ShiftType::Ilde,
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
        }
    }

    fn get_planet_at_point(&mut self, point: Pair<f64>) -> Option<usize> {
        self.painter.get_object_at_point(point)
    }

    pub fn event<E: GenericEvent>(&mut self, event: &E) {
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
            if let Some(index) = self.get_planet_at_point(self.last_mouse_pos) {
                self.painter.planet_offset_update(index);
            }
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
    }
}
