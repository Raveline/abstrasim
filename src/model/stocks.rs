use std::collections::BTreeMap;
use model::business::Business;

/// Records of all stocks values
pub struct Stocks<'a> {
    values : BTreeMap<&'a Business, Vec<f32>>
}

impl <'a> Stocks<'a> {
    pub fn new() -> Stocks<'a> {
        Stocks { values: BTreeMap::new() }
    }

    pub fn push(&mut self, b: &'a Business, value: f32) {
        if let Some(v) = self.values.get_mut(b) {
            v.push(value);
            return
        }
        {
            self.values.insert(b, vec!(value));
        }
    }
}
