extern crate serde_json;
use std::collections::BTreeMap;
use utils::file::read_file;
use model::names::{NameList, SectorNames};
use model::sectors::Sector;

/// Type used only for deserialization
#[derive(Serialize, Deserialize)]
struct ConfigFile {
    sectors: Vec<SectorConfig>,
    names: Vec<String>
}

/// Type used only for deserialization
#[derive(Serialize, Deserialize)]
struct SectorConfig {
    name: Sector,
    words: SectorNames
}

pub fn build_name_list() -> NameList {
    let mut sectors_to_names = BTreeMap::new();
    let config_file = load_config();
    for sc in config_file.sectors {
        sectors_to_names.insert(sc.name, sc.words);
    }
    NameList { sectors: sectors_to_names, names: config_file.names }
}

fn load_config() -> ConfigFile {
    let name_json = read_file("data/names.json");
    serde_json::from_str(&name_json).unwrap()
}
