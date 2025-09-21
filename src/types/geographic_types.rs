pub enum Country {
    England,
    Scotland,
    Wales,
    NorthernIreland,
}

impl From<&str> for Country{
    fn from(country_string: &str) -> Self{
        match country_string {
            "England" => Country::England,
            "Scotland" => Country::Scotland,
            "Wales" => Country::Wales,
            "Northern Ireland" => Country::NorthernIreland,
            _ => panic!("Invalid Country {}", country_string)
        }
    }
}

pub struct Constituency{
    pub name: String,
    pub id: String,
    pub country: Country
}
