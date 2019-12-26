extern crate clap;

use clap::{App, Arg};

use std::{
    io::{self, BufRead, Write},
    process,
};

use city_pop::*;

fn main() {
    let matches = App::new(clap::crate_name!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .args(&[Arg::with_name("city_name").short("city").long("city_name")])
        .args(&[Arg::with_name("DATA_PATH").required(true)])
        .get_matches();
    if matches.args.is_empty() {
        println!("{}", matches.usage());
        process::exit(1);
    }

    if matches.is_present("city_name") {
        match search(
            matches.value_of("DATA_PATH").unwrap().to_owned(),
            matches.value_of("city_name").unwrap().to_owned(),
        ) {
            Ok(result) => println!("{:?}", result),
            Err(err) => eprintln!("failed to search: {}", err),
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
