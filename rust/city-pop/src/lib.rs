extern crate csv;
extern crate serde;

use csv::Reader;
use serde::Deserialize;

use std::{error::Error, fs::File, path::Path};

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

#[derive(Debug)]
pub struct PopulationCount {
    city: String,
    country: String,
    count: u64,
}

pub fn search<P: AsRef<Path>>(
    file_path: P,
    city_name: String,
) -> Result<PopulationCount, Box<dyn Error>> {
    println!("searching");
    let fd = File::open(file_path)?;
    let mut rdr = Reader::from_reader(fd);
    let mut found = None;
    for row in rdr.deserialize() {
        let row: Row = row?;
        if row.city == city_name {
            match row.population {
                None => {}
                Some(count) => {
                    found = Some(PopulationCount {
                        city: row.city,
                        country: row.country,
                        count,
                    });
                }
            };
            break;
        }
    }
    match found {
        Some(found) => Ok(found),
        None => Err(From::from(
            "No matching cities with a population were found.",
        )),
    }
}
