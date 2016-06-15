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
}
