use crate::types::results_types::ConstituencyResult;
use csv::Reader;
use std::error::Error;
use std::fs::File;

pub fn read_results_file(file_name: &str) -> Result<Vec<ConstituencyResult>, Box<dyn Error>> {
    let election_results_file = File::open(file_name)?;
    let mut election_results_data = Reader::from_reader(election_results_file);
    let headers = election_results_data.headers()?.clone();

    let mut election_results: Vec<ConstituencyResult> = Vec::new();
    for election_result in election_results_data.records() {
        let record = election_result?;
        let constituency_result = ConstituencyResult::from_record(&record, &headers);
        election_results.push(constituency_result);
    }
    Ok(election_results)
}
