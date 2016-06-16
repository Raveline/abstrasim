use rand::thread_rng;
use rand::Rng;
use model::business::Business;
use model::sectors::Sector;
use std::collections::BTreeMap;

pub type SectorBusinesses = BTreeMap<Sector, Vec<Business>>;

pub struct World {
    pub sectors: SectorBusinesses
}

impl World {
    pub fn new() -> World {
        World { sectors : BTreeMap::new() }
    }

    pub fn tick(&mut self) {
        for (_, biz) in self.sectors.iter_mut() {
            let sectorial_health = 0.5;
            let random_factor = thread_rng().gen_range(0., 1.);
            for b in biz {
                if b.perception == 0.0 {
                    b.performance = b.compute_performance(sectorial_health);
                }
                b.performance = b.compute_performance(sectorial_health);
                let change = b.compute_capitalisation_change(random_factor);
                b.capitalisation = (b.capitalisation as f32 + change) as u32;
                b.perception = b.compute_perception(random_factor);
            }
        }
    }
}
