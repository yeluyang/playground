use std::fs::{self, File};

extern crate clap;
use clap::{App, AppSettings, Arg};

#[macro_use]
extern crate log;
use log::LevelFilter;

extern crate env_logger;
use env_logger::{Builder, Env};

extern crate serde_json;

extern crate grpcio;

mod rpc;
use rpc::{Config, PeerServer};

fn main() {
    let matches = App::new("ping_pong")
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .args(&[
            // common arguments
            Arg::with_name("verbose")
                .help("Print logs when running")
                .short("v")
                .long("verbose")
                .multiple(true),
            Arg::with_name("quiet")
                .help("Mute when running, block log even Error occur")
                .short("q")
                .long("quiet")
                .conflicts_with("verbose"),
            // custome arguments
            Arg::with_name("config")
                .help("/path/to/config")
                .short("c")
                .long("config")
                .takes_value(true)
                .required(true),
        ])
        .get_matches();

    let log_level = if matches.is_present("q") {
        LevelFilter::Off
    } else {
        match matches.occurrences_of("verbose") as usize {
            0 => LevelFilter::Error,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    };

    Builder::from_env(Env::default().default_filter_or(log_level.to_string()))
        .is_test(true)
        .init();

    info!("start with arguments: {:?}", matches.args);

    let config: Config =
        serde_json::from_reader(File::open(matches.value_of("config").unwrap()).unwrap()).unwrap();

    fs::create_dir_all(&config.logs).unwrap();
    let mut server = PeerServer::new(config.clone());

    server.run();
}
