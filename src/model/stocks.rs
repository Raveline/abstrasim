use std::collections::BTreeMap;
use model::business::Ticker;

/// Records of all stocks values
pub struct Stocks {
    values : BTreeMap<String, Vec<f32>>
}

impl Stocks {
    pub fn new() -> Stocks {
        Stocks { values: BTreeMap::new() }
    }

    pub fn push(&mut self, ticker: &Ticker, value: f32) {
        if let Some(v) = self.values.get_mut(ticker) {
            v.push(value);
            return
        }
        {
            self.values.insert(ticker.to_string(), vec!(value));
        }
    }

    pub fn get(&self, ticker: &Ticker, tick: usize) -> f32 {
        self.values.get(ticker).unwrap()[tick]
    }

    pub fn get_all(&self, ticker: &Ticker) -> Vec<f32> {
        self.values.get(ticker).unwrap().clone()
    }
}
