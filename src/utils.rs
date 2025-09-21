use csv::StringRecord;
use std::collections::HashMap;

pub fn build_column_map(headers: &StringRecord) -> HashMap<&str, usize> {
    headers
        .iter()
        .enumerate()
        .map(|(i, name)| (name, i))
        .collect()
}
