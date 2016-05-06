#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
use std::collections::BTreeMap;


mod model;
mod utils;
mod init;

fn main() {
    let nl = init::names::load_names();
}
