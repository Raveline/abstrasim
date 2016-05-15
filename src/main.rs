#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate rand;


mod model;
mod utils;
mod init;

fn main() {
    let nl = init::names::load_names();
    println!("10 random names for english software companies...");
    for _ in 0..10 {
        println!("{}", model::names::random_company_name("Software", "english", &nl));
    }
}
