
#[derive(Debug, Clone, Copy)]
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
            "LD" => Parties::LiberalDemocrats,
            "Green" => Parties::Green,
            "RUK" => Parties::Reform,
            "PC" => Parties::PlaidCymru,
            "SNP" => Parties::ScottishNational,
            _ => Parties::Other
        }
    }
}


impl Parties {
    pub fn colour(&self) -> &'static str{
        match self {
            Parties::Labour => "red",
            Parties::Conservatives => "blue",
            Parties::LiberalDemocrats => "orange",
            Parties::Green => "green",
            Parties::Reform => "turquoise",
            Parties::PlaidCymru => "yellow",
            Parties::ScottishNational => "yellow",
            Parties::Other => "grey",
        }
    }

    pub fn logo_file(&self) -> Option<&'static str> {
        let path = match self {
            Parties::Labour => "./data/logos/labour.png",
            Parties::Conservatives => "./data/logos/conservatives.png",
            Parties::LiberalDemocrats => "./data/logos/libdems.png",
            Parties::Green => "./data/logos/green.png",
            Parties::Reform => "./data/logos/reform.png",
            Parties::PlaidCymru => "./data/logos/plaidcymru.png",
            Parties::ScottishNational => "./data/logos/snp.png",
            Parties::Other => return None,
        };
        Some(path)
    }
}

#[derive(Debug)]
pub struct Party{
    pub name: Parties,
    pub colour: String,
    pub logo_file: Option<String>
}


impl Party {
    pub fn new(party_string: &str) -> Self {
        let party_enum = Parties::from(party_string);
        Party {
            name: party_enum,
            colour: party_enum.colour().to_string(),
            logo_file: party_enum.logo_file().map(|s| s.to_string()),
        }
    }
}
