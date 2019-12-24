extern crate csv;
extern crate serde;

use csv::Reader;
use serde::Deserialize;

use std::fs::File;

#[cfg(test)]
mod tests;

pub mod error;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Row {
    country: String,
    city: String,
    accent_city: String,
    region: String,
    population: Option<u64>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

pub fn find_city_in_csv(city_name: String, file_path: String) -> Option<Row> {
    let fd = File::open(file_path).unwrap();
    let mut rdr = Reader::from_reader(fd);
    for row in rdr.deserialize() {
        let row: Row = row.unwrap();
        if row.city == city_name {
            return Some(row);
        }
    }
    None
}
