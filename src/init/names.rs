
extern crate serde_json;

use utils::file::read_file;
use model::names::NameList;

pub fn load_names() -> NameList {
    let name_json = read_file("data/names.json");
    serde_json::from_str(&name_json).unwrap()
}
