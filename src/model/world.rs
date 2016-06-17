use std::collections::BTreeMap;
use rand::thread_rng;
use rand::Rng;

use model::business::{Business, Ticker};
use model::sectors::Sector;
use model::stocks::Stocks;

pub type SectorBusinesses = BTreeMap<Sector, Vec<Business>>;

pub struct World {
    pub companies: Vec<Business>,
    stored_tickers: BTreeMap<Ticker, usize>,
    pub stocks: Stocks
}

impl World {
    pub fn new() -> World {
        World { companies: vec!(),
                stored_tickers: BTreeMap::new(),
                stocks: Stocks::new() }
    }

    pub fn tick(&mut self) {
        let sectorial_health = 0.5;
        for biz in self.companies.iter_mut() {
            let random_factor = thread_rng().gen_range(0., 1.);
            biz.performance = biz.compute_performance(sectorial_health);
            let change = biz.compute_capitalisation_change(random_factor);
            biz.capitalisation = (biz.capitalisation as f32 + change) as u32;
            biz.perception = biz.compute_perception(random_factor);
            self.stocks.push(&biz.name, biz.get_current_stock_value());
        }
    }

    pub fn get_by_ticker(&self, ticker: Ticker) -> Option<&Business> {
        match self.stored_tickers.get(&ticker) {
            Some (idx) => Some(&self.companies[*idx]),
            None => None
        }
    }

    fn define_ticker(&self, business: &Business) -> String {
        let mut cap_only = business.name.chars().filter(|c| c.is_uppercase()).collect();
        while self.ticker_exists(&cap_only) {
            let initial = cap_only.chars().nth(0).unwrap();
            cap_only.insert(0, initial);
        }
        cap_only
    }

    fn ticker_exists(&self, ticker: &Ticker) -> bool {
        self.companies.iter().find(|&b| b.ticker == *ticker).is_some()
    }

    pub fn add_business(&mut self, business: Business) {
        let ticker = self.define_ticker(&business);
        self.companies.push(business);
        let last_index = self.companies.len() - 1;
        let mut company = self.companies.get_mut(last_index).unwrap();
        company.ticker = ticker.clone();
        self.stored_tickers.insert(ticker, last_index);
    }
}
