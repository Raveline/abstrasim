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
    for _ in 0..10
    {
        world.tick();
    }
    utils::file::write_stock("/home/raveline/stocks.csv", &world).unwrap();
}

fn generate_world() -> World {
    let nl = init::names::build_name_list();
    let mut w = World::new();
    for sec in Sector::iterate() {
        for _ in 0..10 {
            w.add_business(generate_business(&nl, *sec));
        }
    }
    w
}

fn generate_business(nl: &NameList, sector : Sector) -> Business {
    let name = model::names::random_company_name(sector, "english", &nl);
    Business::new(name, sector)
}
