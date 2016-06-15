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
    let world = generate_world();
    for (sec, biz) in world.sectors {
        println!("Sector : {}", sec);
        for business in biz {
            println!("\t{:?}", business);
        }
    }
}

fn generate_world() -> World {
    let nl = init::names::build_name_list();
    let mut w = World::new();
    w.sectors.insert(Sector::Software, generate_sector(&nl, Sector::Software));
    w
}

/// Temporary function to test our simulation
fn generate_sector(nl: &NameList, sector : Sector) -> Vec<Business> {
    let mut businesses = vec!();
    for _ in 0..10 {
        let name = model::names::random_company_name(sector, "english", &nl);
        let b = Business::new(name, sector);
        businesses.push(b);
    }
    businesses
}
