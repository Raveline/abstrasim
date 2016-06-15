#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate rand;
extern crate serde;
extern crate serde_json;


mod model;
mod utils;
mod init;
use model::names::NameList;
use model::sectors::Sector;
use model::business::Business;
use model::world::World;

fn main() {
    let mut world = generate_world();
    // Tick once
    world.tick();
    for (sec, biz) in world.sectors {
        println!("Sector : {}", sec);
        for business in biz {
            println!("\t{} - perf. : {} - perception : {} - current : {}",
                     business,
                     business.performance,
                     business.perception,
                     business.get_current_stock_value());
        }
    }
}

fn generate_world() -> World {
    let nl = init::names::build_name_list();
    let mut w = World::new();
    for sec in Sector::iterate() {
        w.sectors.insert(*sec, generate_sector(&nl, *sec));
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
