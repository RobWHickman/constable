use crate::types::geographic_types::Constituency;
use crate::types::party_types::{Party, PARTY_COLUMNS, Parties};
use csv::StringRecord;
use crate::utils::build_column_map;

#[derive(Debug)]
pub struct PartyResult{
    pub party: Party,
    pub votes: i64
}

#[derive(Debug)]
pub struct ConstituencyResult{
    pub constituency: Constituency,
    pub winner: Party,
    pub outcome: String,
    pub majority: i64,
    pub results: Vec<PartyResult>
}

impl ConstituencyResult {
    pub fn from_record(record: &StringRecord, headers: &StringRecord) -> Self {
        let column_map = build_column_map(headers);
        let constituency = Constituency::from_record(record, headers);

        let winner_string = record.get(column_map["First party"]).expect("Missing Winning Party");
        let winner = Party::from_string(winner_string);
        
        let outcome_string = record.get(column_map["Result"]).expect("Missing Result").to_string();
        let majority_string = record.get(column_map["Majority"]).expect("Miossing Majority");
        let majority = majority_string.parse::<i64>().expect("Invalid vote count for Majority");

        let mut party_results: Vec<PartyResult> = Vec::new();
        let mut total_other_votes = 0;
        for party_col in PARTY_COLUMNS {
            let party_votes_string = record.get(column_map[party_col]).expect(&format!("Missing Party {}", party_col));
            let party_votes = party_votes_string.parse::<i64>().expect(&format!("Invalid vote count for {}", party_col));
            let party = Party::from_string(party_col);
            
            if party.name == Parties::Other {
                total_other_votes += party_votes
            } else {
                let party_result = PartyResult {
                    party: party,
                    votes: party_votes,
                };
                party_results.push(party_result);
            }

        let other_results = PartyResult {
            party: Party::from_string("Other"),
            votes: total_other_votes
        };

        party_results.push(other_results);
    }

        ConstituencyResult { constituency: constituency, winner: winner, outcome: outcome_string, majority: majority, results: party_results }
    }
}