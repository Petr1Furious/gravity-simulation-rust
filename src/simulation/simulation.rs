use crate::simulation::planet::{Planet, G};

pub struct Simulation {
    pub planets: Vec<Planet>,
}

impl Simulation {
    pub fn new() -> Self {
        Self { planets: Vec::new() }
    }

    fn dist2(coords1: (f64, f64), coords2: (f64, f64)) -> f64 {
        (coords1.0 - coords2.0) * (coords1.0 - coords2.0) + (coords1.1 - coords2.1) * (coords1.1 - coords2.1)
    }

    pub fn update_velocity(&mut self, dt: f64) {
        let old_planets = self.planets.clone();

        let gdt = G * dt;

        for planet in self.planets.iter_mut() {
            for other_planet in old_planets.iter() {
                if planet.coords != other_planet.coords {
                    let d2 = Simulation::dist2(planet.coords, other_planet.coords);
                    let d3 = d2 * f64::sqrt(d2);
                    let gdt_d3 = gdt / d3;
                    planet.velocity.0 += gdt_d3 * (other_planet.coords.0 - planet.coords.0) * other_planet.m;
                    planet.velocity.1 += gdt_d3 * (other_planet.coords.1 - planet.coords.1) * other_planet.m;
                }
            }
        }
    }
    
    pub fn update_coords(&mut self, dt: f64) {
        for planet in self.planets.iter_mut() {
            planet.update_coords(dt);
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.update_velocity(dt);
        self.update_coords(dt);
    }

    pub fn get_planets(&mut self) -> &mut Vec<Planet> {
        &mut self.planets
    }
}
