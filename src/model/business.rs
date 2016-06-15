use std::fmt;
use rand::thread_rng;
use rand::Rng;
use model::sectors::Sector;

#[derive(Debug)]
pub struct Business {
    /// Legal name of this business
    name: String,
    /// 0 to 1 estimate of the quality of management
    leadership: f32,
    /// 0 to 1 estimate of the global size and its markeshare
    size: f32,
    /// 0 to 1 estimate of the amount invested
    investement: f32,
    /// 0 to 1 estimate of the ability to communicate
    pr: f32,
    /// 0 to 1 estimate of the current performances
    results: f32,
    /// How market actors perceive the performances ?
    pub perception: f32,
    /// Real performances of the business
    pub performance: f32,
    /// Outstanding shares times the price, total value of
    /// this business in the stock exchange.
    /// Expressed as an integer, in million.
    capitalisation: u32,
    /// Number of shares of the business on the stock exchange.
    /// Divided by the capitalization, it gives the share value.
    /// Express as an integer, in million
    shares_outstanding: u32,
    sector: Sector
}

impl fmt::Display for Business {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Business {
    pub fn new(name: String, sector: Sector) -> Business {
        let random_size : f32 = thread_rng().gen_range(0., 1.);
        let random_cap_factor : u32 = thread_rng().gen_range(2000, 1000000);
        let random_so_factor : u32 = thread_rng().gen_range(100, 10000);
        let capitalisation = random_size as f64 * random_cap_factor as f64;
        let shares_outstanding = random_size as f64 * random_so_factor as f64;
        Business { name: name,
                   sector: sector,
                   leadership: thread_rng().gen_range(0., 1.),
                   size: random_size,
                   investement: thread_rng().gen_range(0., 1.),
                   pr: thread_rng().gen_range(0., 1.),
                   results: thread_rng().gen_range(0., 1.),
                   perception: 0.,
                   performance: 0.,
                   capitalisation: capitalisation as u32,
                   shares_outstanding: shares_outstanding as u32 }
    }

    pub fn compute_performance(&self, sector_health: f32) -> f32 {
        (self.leadership + self.size + self.investement + self.results + sector_health) / 5.0 // TODO: Add business event flags
    }

    pub fn compute_perception(&self, rnd_factor: f32) -> f32 {
        (self.performance + self.pr + rnd_factor) / 3.0
    }

    pub fn get_current_stock_value(&self) -> f64 {
        let real = (self.capitalisation as f64 / self.shares_outstanding as f64);
        (real * 100.0).round() / 100.0
    }
}
