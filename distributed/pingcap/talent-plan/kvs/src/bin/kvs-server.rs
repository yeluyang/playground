extern crate clap;
use clap::{App, AppSettings, Arg};

#[macro_use]
extern crate log;
use log::LevelFilter;

extern crate env_logger;
use env_logger::Env;

use kvs::{network::Server, Result};

fn main() -> Result<()> {
    let matches = App::new("kvs-server")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .args(&[
            Arg::with_name("verbose")
                .short("v")
                .takes_value(false)
                .multiple(true)
                .conflicts_with("quiet"),
            Arg::with_name("quiet")
                .long("quiet")
                .short("q")
                .takes_value(false)
                .conflicts_with("verbose"),
        ])
        .args(&[
            Arg::with_name("IP:PORT")
                .long("addr")
                .short("a")
                .takes_value(true)
                .default_value("127.0.0.1:4000"),
            Arg::with_name("ENGINE-NAME")
                .long("engine")
                .short("e")
                .takes_value(true),
        ])
        .get_matches();

    let log_level = if matches.is_present("quiet") {
        LevelFilter::Off
    } else {
        match matches.occurrences_of("verbose") {
            0 => LevelFilter::Error,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    };
    env_logger::init_from_env(Env::default().default_filter_or(log_level.to_string()));

    info!("server start");

    let mut server = Server::on(
        matches.value_of("IP:PORT").unwrap().to_owned(),
        &std::env::current_dir()?.as_path(),
    )?;

    server.run()
}
