use crate::utils::build_column_map;
use csv::StringRecord;

#[derive(Debug)]
pub enum Country {
    England,
    Scotland,
    Wales,
    NorthernIreland,
}

impl From<&str> for Country {
    fn from(country_string: &str) -> Self {
        match country_string {
            "England" => Country::England,
            "Scotland" => Country::Scotland,
            "Wales" => Country::Wales,
            "Northern Ireland" => Country::NorthernIreland,
            _ => panic!("Invalid Country {}", country_string),
        }
    }
}

#[derive(Debug)]
pub struct Constituency {
    pub name: String,
    pub id: String,
    pub country: Country,
}

impl Constituency {
    pub fn from_record(record: &StringRecord, headers: &StringRecord) -> Self {
        let column_map = build_column_map(headers);

        let id_string = record
            .get(column_map["ONS ID"])
            .expect("Missing ONS ID")
            .to_string();
        let name_string = record
            .get(column_map["Constituency name"])
            .expect("Missing Constituency Name")
            .to_string();
        let country_string = record
            .get(column_map["Country name"])
            .expect("Missing Country Name");

        Constituency {
            id: id_string,
            name: name_string,
            country: Country::from(country_string),
        }
    }
}
