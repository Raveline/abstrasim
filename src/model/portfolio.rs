use model::business;
use model::world::World;

/// For the moment, we'll use integer as "counting the days".
/// TODO: replace by dates
pub type Date = u32;
/// For the moment, cash will be handled as a simple f64, but
/// we'll need some BigDecimal or likewise type.
pub type Cash = f64;

pub enum Investment {
    Call(business::Ticker, f32),
    Put(business::Ticker, f32, Date)
}

impl Investment {
    fn evaluate(&self, world: &World) -> f32 {
        match *self {
            Investment::Call(ref t, bought_at) => {
                let current = world.get_by_ticker(t).unwrap().get_current_stock_value();
                current - bought_at
            },
            Investment::Put(ref t, bought_at, _) => {
                let current = world.get_by_ticker(t).unwrap().get_current_stock_value();
                bought_at - current
            }
        }
    }
}

pub struct Portfolio {
    cashflow: Cash,
    options: Vec<Investment>
}

impl Portfolio {
    pub fn place_call(&mut self, ticker: business::Ticker, buying_value: f32) {
        let new_call = Investment::Call(ticker, buying_value);
        self.options.push(new_call);
    }

    pub fn place_put(&mut self, ticker: business::Ticker, selling_value: f32, expiration: Date) {
        let new_put = Investment::Put(ticker, selling_value, expiration);
        self.options.push(new_put);
    }

    pub fn evaluate(&self, world: &World) -> Cash {
        let mut balance = 0.0;
        for option in self.options.iter() {
            balance += option.evaluate(world) as f64;
        }
        balance
    }
}
