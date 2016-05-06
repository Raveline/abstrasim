use std::vec;
use std::collections::BTreeMap;

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
    pub sectors: BTreeMap<String, SectorNames>
}
