mod read;
mod types;
mod utils;

use std::error::Error;

const RESULTS_FILE: &str = "./data/results/HoC-GE2024-results-by-constituency.csv";

fn main() -> Result<(), Box<dyn Error>> {
    let constituency_results = read::read_results_file(RESULTS_FILE)?;
    for result in constituency_results {
        if result.constituency.name == "North Down" {
            println!("{:?}", result);
        }
    }

    Ok(())
}
