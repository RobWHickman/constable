use crate::types::geographic_types::Constituency;
use crate::types::party_types::Party;

pub struct PartyResult{
    pub party: Party,
    pub votes: i64
}

pub struct ConstituencyResult{
    pub constituency: Constituency,
    pub winner: Party,
    pub outcome: String,
    pub majority: i64,
    pub results: Vec<PartyResult>
}