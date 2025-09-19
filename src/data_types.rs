pub enum Country {
    England,
    Scotland,
    Wales,
    NorthernIreland,
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