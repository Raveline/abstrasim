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
