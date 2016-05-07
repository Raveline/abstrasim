extern crate rand;

use std::vec;
use std::collections::BTreeMap;
use model::names::rand::Rng;

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

fn random_simple_name(from: &NameList) -> &String {
    pick_random(&from.names)
}

pub fn random_language_generic_first(sector: &str, language: &str, from: &NameList) -> String {
    let sector_langs = from.sectors.get(sector).expect("No such sector");
    let lang_last = sector_langs.words.get(language).expect("No such language");
    let generic_langs = from.sectors.get("generic").expect("No such sector");
    let lang_fst = generic_langs.words.get(language).expect("No such language");
    let n = format!("{} {}", pick_random(&lang_fst.first), pick_random(&lang_last.last));
    n.to_string()
}
