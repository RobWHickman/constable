#[derive(Debug)]
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

pub enum Parties {
    Labour,
    Conservatives,
    LiberalDemocrats,
    Green,
    Reform,
    PlaidCymru,
    ScottishNational,
    Other
}

impl From<&str> for Parties{
    fn from(party_string: &str) -> Self {
        match party_string {
            "Lab" => Parties::Labour,
            "Con" => Parties::Conservatives,
            "Lib" => Parties::LiberalDemocrats,
            "Green" => Parties::Green,
            "RUK" => Parties::Reform,
            "PC" => Parties::PlaidCymru,
            "SNP" => Parties::ScottishNational,
            _ => Parties::Other
        }
    }
}

pub struct Constituency{
    pub name: String,
    pub id: String,
    pub country: Country
}

pub struct Party{
    pub name: Parties,
    pub colour: String,
    pub logo_file: String
}

pub struct PartyResult{
    pub party: Party,
    pub votes: i64
}

pub struct ConstituencyResult{
    pub constituency: Constituency,
    pub winner: Party,
    pub majority: i64,
    pub results: Vec<PartyResult>
}