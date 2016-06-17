use model::business;

/// For the moment, we'll use integer as "counting the days".
/// TODO: replace by dates
pub type Date = u32;
/// For the moment, cash will be handled as a simple f64, but
/// we'll need some BigDecimal or likewise type.
pub type Cash = f64;

pub enum Option {
    Call(business::Ticker, f32),
    Put(business::Ticker, f32, Date)
}

pub struct Portfolio {
    cashflow: Cash,
    options: Vec<Option>
}

impl Portfolio {
    pub fn place_call(&mut self, ticker: business::Ticker, buying_value: f32) {
        let new_call = Option::Call(ticker, buying_value);
        self.options.push(new_call);
    }

    pub fn place_put(&mut self, ticker: business::Ticker, selling_value: f32, expiration: Date) {
        let new_put = Option::Put(ticker, selling_value, expiration);
        self.options.push(new_put);
    }
}
