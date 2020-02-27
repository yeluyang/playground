extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

#[macro_use]
extern crate log;
use log::LevelFilter;

extern crate env_logger;
use env_logger::Env;
use kvs::{self, network::Client, Result};

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
        .args(&[Arg::with_name("IP:PORT")
            .long("addr")
            .short("a")
            .takes_value(true)
            .default_value("127.0.0.1:4000")])
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

    let mut client = Client::connect(matches.value_of("IP:PORT").unwrap())?;

    match matches.subcommand() {
        ("set", Some(m)) => {
            info!("setting key-value");
            client.set(
                m.value_of("KEY").unwrap().to_owned(),
                m.value_of("VALUE").unwrap().to_owned(),
            )
        }
        ("get", Some(m)) => {
            info!("getting value by key");
            match client.get(m.value_of("KEY").unwrap().to_owned())? {
                Some(val) => println!("{}", val),
                None => println!("Key not found"),
            };
            Ok(())
        }
        ("rm", Some(m)) => {
            info!("removing key-value");
            client.remove(m.value_of("KEY").unwrap().to_owned())
        }
        _ => unreachable!(),
    }
}
