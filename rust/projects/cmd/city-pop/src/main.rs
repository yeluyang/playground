extern crate clap;

use clap::{App, AppSettings, Arg};

use std::{
    io::{self, BufRead, Write},
    process,
};

use city_pop::*;

fn main() {
    let matches = App::new(clap::crate_name!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .args(&[Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .multiple(true)])
        .args(&[Arg::with_name("city_name")
            .short("c")
            .long("city_name")
            .takes_value(true)])
        .args(&[Arg::with_name("DATA_PATH").required(true)])
        .get_matches();

    if matches.is_present("city_name") {
        match search(
            matches.value_of("DATA_PATH").unwrap().to_owned(),
            matches.value_of("city_name").unwrap().to_owned(),
        ) {
            Ok(result) => println!("{:?}", result),
            Err(err) => match err {
                CliError::NotFound(_, _) => {
                    if matches.occurrences_of("verbose") > 0 {
                        eprintln!("failed to search: {}", err)
                    }
                }
                _ => eprintln!("failed to search: {}", err),
            },
        }
    } else {
        print!("> ");
        std::io::stdout().lock().flush().unwrap();
        for line in io::stdin().lock().lines() {
            let line = line.unwrap().replace("\n", "");
            match search(matches.value_of("DATA_PATH").unwrap().to_owned(), line) {
                Ok(result) => println!("{:?}", result),
                Err(err) => eprintln!("failed to search: {}", err),
            };
            print!("> ");
            std::io::stdout().lock().flush().unwrap();
        }
    };
}
