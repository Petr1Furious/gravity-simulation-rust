use super::planet::Planet;

#[derive(Clone)]
pub struct PlanetSystem {
    pub planets: Vec<Planet>,
}

impl PlanetSystem {
    pub fn default() -> Self {
        Self { planets: Vec::new() }
    }

    pub fn new(planets: Vec<Planet>) -> Self {
        Self { planets }
    }
}
