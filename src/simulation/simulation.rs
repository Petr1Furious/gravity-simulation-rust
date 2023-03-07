use crate::{
    simulation::planet::G,
    utils::pair::Pair,
};

use super::planet_system::PlanetSystem;

pub struct Simulation {
    pub planet_system: PlanetSystem,
}

impl Simulation {
    #[allow(dead_code)]
    pub fn default() -> Self {
        Self {
            planet_system: PlanetSystem::default(),
        }
    }

    pub fn new(planet_system: PlanetSystem) -> Self {
        Self { planet_system }
    }

    fn dist2(coords1: (f64, f64), coords2: (f64, f64)) -> f64 {
        (coords1.0 - coords2.0) * (coords1.0 - coords2.0)
            + (coords1.1 - coords2.1) * (coords1.1 - coords2.1)
    }

    pub fn update_velocity(&mut self, dt: f64) {
        let old_planet_system = self.planet_system.clone();

        let gdt = G * dt;

        for planet in self.planet_system.planets.iter_mut() {
            for other_planet in old_planet_system.planets.iter() {
                if planet.coords != other_planet.coords {
                    let d2 = Simulation::dist2(planet.coords, other_planet.coords);
                    let d3 = d2 * f64::sqrt(d2);
                    let gdt_d3 = gdt / d3;
                    planet.velocity.0 +=
                        gdt_d3 * (other_planet.coords.0 - planet.coords.0) * other_planet.m;
                    planet.velocity.1 +=
                        gdt_d3 * (other_planet.coords.1 - planet.coords.1) * other_planet.m;
                }
            }
        }
    }

    pub fn update_coords(&mut self, dt: f64) {
        for planet in self.planet_system.planets.iter_mut() {
            planet.update_coords(dt);
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.update_velocity(dt);
        self.update_coords(dt);
    }

    pub fn get_planet_system(&mut self) -> &mut PlanetSystem {
        &mut self.planet_system
    }

    pub fn get_planet_at_pos(&self, pos: Pair<f64>) -> Option<usize> {
        for (index, planet) in self.planet_system.planets.iter().enumerate() {
            let dist = pos - planet.coords.into();
            if dist.x * dist.x + dist.y * dist.y < planet.r * planet.r {
                return Some(index);
            }
        }
        None
    }
}
