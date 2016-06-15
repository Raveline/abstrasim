use std::fmt;
use rand::thread_rng;
use rand::Rng;
use model::sectors::Sector;

#[derive(Debug)]
pub struct Business {
    name: String,
    leadership: f32,
    size: f32,
    investement: f32,
    pr: f32,
    results: f32,
    perception: f32,
    performance: f32,
    sector: Sector
}

impl fmt::Display for Business {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Business {
    pub fn new(name: String, sector: Sector) -> Business {
        Business { name: name,
                   sector: sector,
                   leadership: thread_rng().gen_range(0., 1.),
                   size: thread_rng().gen_range(0., 1.),
                   investement: thread_rng().gen_range(0., 1.),
                   pr: thread_rng().gen_range(0., 1.),
                   results: thread_rng().gen_range(0., 1.),
                   perception: 0.,
                   performance: 0. }
    }

    pub fn compute_performance(&self, sector_health: f32) -> f32 {
        (self.leadership + self.size + self.investement + self.results + sector_health) / 5.0 // TODO: Add business event flags
    }

    pub fn compute_perception(&self, rnd_factor: f32) -> f32 {
        (self.performance + self.pr + rnd_factor) / 3.0
    }
}
