extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

#[macro_use]
extern crate log;
use log::LevelFilter;

extern crate env_logger;
use env_logger::Env;
use kvs::{self, KvStore, Result};

fn main() -> Result<()> {
    let matches = App::new("kvs-client")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .settings(&[
            AppSettings::ArgRequiredElseHelp,
            AppSettings::SubcommandRequired,
        ])
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
        .subcommands(vec![
            SubCommand::with_name("set").args(&[
                Arg::with_name("KEY").required(true).takes_value(true),
                Arg::with_name("VALUE").required(true).takes_value(true),
            ]),
            SubCommand::with_name("get")
                .args(&[Arg::with_name("KEY").required(true).takes_value(true)]),
            SubCommand::with_name("rm")
                .args(&[Arg::with_name("KEY").required(true).takes_value(true)]),
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

    info!("client start");

    let mut kv_store = KvStore::open(std::env::current_dir()?)?;
    info!("loaded data into KV store");

    match matches.subcommand() {
        ("set", Some(m)) => {
            info!("setting key-value");
            kv_store.set(
                m.value_of("KEY").unwrap().to_owned(),
                m.value_of("VALUE").unwrap().to_owned(),
            )
        }
        ("get", Some(m)) => {
            info!("getting value by key");
            kv_store
                .get(m.value_of("KEY").unwrap().to_owned())
                .map_err(|err| {
                    eprintln!("{}", err);
                    err
                })
                .map(|opt| match opt {
                    Some(val) => println!("{}", val),
                    None => println!("Key not found"),
                })
        }
        ("rm", Some(m)) => {
            info!("removing key-value");
            kv_store.remove(m.value_of("KEY").unwrap().to_owned())
        }
        _ => unreachable!(),
    }
}
