pub const G: f64 = 6.6743e-11;

#[derive(Clone)]
pub struct Planet {
    pub coords: (f64, f64),
    pub r: f64,
    pub velocity: (f64, f64),
    pub m: f64,
    pub name: String,
}

impl Planet {
    pub fn new(coords: (f64, f64), r: f64, velocity: (f64, f64), m: f64, name: String) -> Self {
        Self {
            coords,
            r,
            velocity,
            m,
            name,
        }
    }

    pub fn update_coords(&mut self, dt: f64) {
        self.coords.0 += self.velocity.0 * dt;
        self.coords.1 += self.velocity.1 * dt;
    }
}
