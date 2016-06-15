use std::slice::Iter;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize, Copy, Clone)]
#[serde(deny_unknown_fields)]
pub enum Sector {
    /// Not a real sector, but we'll put names that are not specific in here
    Generic,
    OilEnergy,
    Software,
    Mining,
    Logistics,
    Pharmaceutics,
    Tourism,
    Luxury,
    Automobile,
    Food,
    Media,
    Telecom,
    Banking,
    Distribution,
    Insurance
}

impl Sector {
    pub fn iterate() -> Iter<'static, Sector> {
        static SECTORS: [Sector; 14] = [
            Sector::OilEnergy, Sector::Software, Sector::Mining,
            Sector::Logistics, Sector::Pharmaceutics, Sector::Tourism,
            Sector::Luxury, Sector::Automobile, Sector::Food, Sector::Media,
            Sector::Telecom, Sector::Banking, Sector::Distribution,
            Sector::Insurance];
        SECTORS.into_iter()
    }
}

impl fmt::Display for Sector {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Sector::Generic => "",
            Sector::OilEnergy => "Oil & Energy",
            Sector::Software => "Software",
            Sector::Mining => "Mining",
            Sector::Logistics => "Logistics",
            Sector::Pharmaceutics => "Pharmaceutics",
            Sector::Tourism => "Tourism",
            Sector::Luxury => "Luxury",
            Sector::Automobile => "Automobile",
            Sector::Food => "Food",
            Sector::Media => "Media",
            Sector::Telecom => "Telecom",
            Sector::Banking => "Banking",
            Sector::Distribution => "Distribution",
            Sector::Insurance => "Insurance" })
    }
}
