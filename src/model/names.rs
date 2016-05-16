use std::collections::BTreeMap;
use rand;
use rand::Rng;
use model::sectors::Sector;


#[derive(Debug, Serialize, Deserialize)]
pub struct SectorWords {
    first: Vec<String>,
    last: Vec<String>
}

pub type SectorNames = BTreeMap<String, SectorWords>;

pub struct NameList {
    pub sectors: BTreeMap<Sector, SectorNames>,
    pub names: Vec<String>
}

fn pick_random(from: &Vec<String>) -> &String {
    rand::thread_rng().choose(from).expect("Tried to pick random from an empty list.")
}

pub fn random_company_name(sector: Sector, language: &str, from: &NameList) -> String {
    let sector_names = from.sectors.get(&sector).expect("No such sector");
    let sector_language = sector_names.get(language).expect("No such language");
    let generics = from.sectors.get(&Sector::Generic).expect("No such sector");
    let generics_language = generics.get(language).expect("No such language");
    match rand::thread_rng().gen_range(0, 5) {
        0 => random_in_two_lists(&from.names, &sector_language.last, " "),
        1 => random_in_two_lists(&sector_language.first, &from.names, " "), 
        2 => random_in_two_lists(&from.names, &from.names, "-"),
        3 => random_in_two_lists(&generics_language.first, &sector_language.last, " "),
        4 => random_in_two_lists(&sector_language.first, &generics_language.last, " "),
        _ => panic!("Should not happen !")
    }
}

fn random_in_two_lists(prefixes: &Vec<String>, postfixes: &Vec<String>, middle: &str) -> String {
    format!("{}{}{}", pick_random(prefixes), middle, pick_random(postfixes))
}
