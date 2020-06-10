extern crate clap;
use clap::{App, AppSettings, Arg};

#[macro_use]
extern crate log;
use log::LevelFilter;

extern crate env_logger;
use env_logger::Env;

fn main() {
    let matches = App::new(clap::crate_name!())
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
    env_logger::init_from_env(Env::default().default_filter_or(log_level.to_string()));

    println!("start with arguments: {:?}", matches.args);
    error!("start with arguments: {:?}", matches.args);
    warn!("start with arguments: {:?}", matches.args);
    info!("start with arguments: {:?}", matches.args);
    debug!("start with arguments: {:?}", matches.args);
    trace!("start with arguments: {:?}", matches.args);
}
