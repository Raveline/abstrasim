#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate rand;
extern crate serde;
extern crate serde_json;


mod model;
mod utils;
mod init;
use model::sectors::Sector;
use model::business::Business;
use model::world::World;

fn main() {
    let world = generate_world();
    for biz in world.businesses {
        println!("{:?}", biz);
    }
}

/// Temporary function to test our simulation
fn generate_world() -> World {
    let mut w = World { businesses: vec!() };
    let nl = init::names::build_name_list();
    for _ in 0..10 {
        let name = model::names::random_company_name(Sector::Software, "english", &nl);
        let b = Business::new(name, Sector::Software);
        w.businesses.push(b);
    }
    w
}
