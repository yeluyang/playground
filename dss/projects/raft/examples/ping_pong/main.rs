use std::fs;

extern crate clap;
use clap::{App, AppSettings, Arg};

#[macro_use]
extern crate log;
use log::LevelFilter;

extern crate env_logger;
use env_logger::{Builder, Env};

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
            // TODO
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

    let config = Config {
        ip: "127.0.0.1".to_owned(),
        port: 10081,
        logs: "tmp/tests/all/1/logs".to_owned(),
        peers: vec![
            ("127.0.0.1".to_owned(), 10082),
            ("127.0.0.1".to_owned(), 10083),
        ],
    };
    fs::create_dir_all(&config.logs).unwrap();
    let mut server = PeerServer::new(config.clone());

    server.run();
}
