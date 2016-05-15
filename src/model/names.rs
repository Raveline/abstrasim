use std::collections::BTreeMap;
use rand;
use rand::Rng;

#[derive(Serialize, Deserialize)]
pub struct SectorWords {
    first: Vec<String>,
    last: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct SectorNames {
    words: BTreeMap<String, SectorWords>
}

#[derive(Serialize, Deserialize)]
pub struct NameList {
    pub sectors: BTreeMap<String, SectorNames>,
    pub names: Vec<String>
}

fn pick_random(from: &Vec<String>) -> &String {
    rand::thread_rng().choose(from).expect("Tried to pick random from an empty list.")
}

pub fn random_company_name(sector: &str, language: &str, from: &NameList) -> String {
    let sector = from.sectors.get(sector).expect("No such sector");
    let sector_language = sector.words.get(language).expect("No such language");
    let generics = from.sectors.get("Generic").expect("No such sector");
    let generics_language = generics.words.get(language).expect("No such language");
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
