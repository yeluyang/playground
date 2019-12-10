use std::{env, process};

extern crate clap;

extern crate env_logger;
extern crate log;

use rgrep::*;

fn main() {
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .args(&[
            clap::Arg::with_name("verbose")
                .multiple(true)
                .short("v")
                .long("verbose"),
            clap::Arg::with_name("ignore_case")
                .short("i")
                .long("ignore-case"),
        ])
        .args(&[
            clap::Arg::with_name("QUERY").required(true).index(1),
            clap::Arg::with_name("TEXT").required(true).index(2),
        ])
        .get_matches();

    match matches.occurrences_of("verbose")  {
        0 => env::set_var("RUST_LOG", "OFF"),
        1 => env::set_var("RUST_LOG", "ERROR"),
        2 => env::set_var("RUST_LOG", "WARN"),
        3 => env::set_var("RUST_LOG", "INFO"),
        _ => env::set_var("RUST_LOG", "TRACE"),
    }
    env_logger::init();

    let cfg = Config::new(
        matches.value_of("QUERY").unwrap(),
        matches.value_of("TEXT").unwrap(),
        matches.is_present("ignore_case"),
    );

    if let Err(e) = run(cfg) {
        eprintln!("failed: {}", e);
        process::exit(1);
    };
}
