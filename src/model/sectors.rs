use std::fmt;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
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
