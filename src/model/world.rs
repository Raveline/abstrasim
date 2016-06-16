use std::collections::BTreeMap;
use rand::thread_rng;
use rand::Rng;

use model;
use init;
use model::names::NameList;
use model::business::Business;
use model::sectors::Sector;
use model::stocks::Stocks;
use init::names;

pub type SectorBusinesses = BTreeMap<Sector, Vec<Business>>;

pub struct World<'a> {
    pub sectors: SectorBusinesses,
    pub stocks: Stocks<'a>
}

impl <'a> World<'a> {
    pub fn new() -> World<'a> {
        World { sectors: BTreeMap::new(),
                stocks: Stocks::new() }
    }

    pub fn tick(&'a mut self) {
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
                self.stocks.push(&b, b.get_current_stock_value());
            }
        }
    }

    pub fn generate_world() -> World<'a> {
        let nl = init::names::build_name_list();
        let mut w = World::new();
        for sec in Sector::iterate() {
            w.sectors.insert(*sec, World::generate_sector(&nl, *sec));
        }
        w
    }

    fn generate_sector(nl: &NameList, sector : Sector) -> Vec<Business> {
        let mut businesses = vec!();
        for _ in 0..10 {
            let name = model::names::random_company_name(sector, "english", &nl);
            let b = Business::new(name, sector);
            businesses.push(b);
        }
        businesses
    }
}
