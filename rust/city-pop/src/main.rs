extern crate clap;

use clap::{App, Arg};

use std::process;

use city_pop::*;

fn main() {
    let matches = App::new(clap::crate_name!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .args(&[
            Arg::with_name("DATA_PATH").required(true),
            Arg::with_name("CITY_NAME").required(true),
        ])
        .get_matches();
    if matches.args.is_empty() {
        println!("{}", matches.usage());
        process::exit(1);
    }
    let result = find_city_in_csv(
        matches.value_of("CITY_NAME").unwrap().to_owned(),
        matches.value_of("DATA_PATH").unwrap().to_owned(),
    )
    .unwrap();
    println!("{:?}", result);
}
