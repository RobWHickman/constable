mod data_types;

use csv::Reader;
use std::error::Error;
use std::fs::File;
use data_types::ElectionResult;

const RESULTS_FILE: &str = "./data/results/HoC-GE2024-results-by-constituency.csv";

fn main() -> Result<(), Box<dyn Error>> {
    let election_results_file = File::open(RESULTS_FILE)?;
    let mut election_results_data = Reader::from_reader(election_results_file);

    let mut election_results: Vec<ElectionResult> = Vec::new();
    let mut i: i64 = 0;
    for election_result in election_results_data.records() {
        let record = election_result?;
        if i < 5{
            println!("{:?}", record);
        }   
        i += 1;
    }

    Ok(())
}
