use std::fs;

extern crate clap;
use clap::{App, AppSettings, Arg};

#[macro_use]
extern crate log;
use log::LevelFilter;

extern crate env_logger;
use env_logger::{Builder, Env};

use kvs::{network::Server, Error, KvStore, Result, SledKvsEngine};

fn main() -> Result<()> {
    let matches = App::new("kvs-server")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .args(&[
            Arg::with_name("verbose")
                .short("v")
                .takes_value(false)
                .global(true)
                .multiple(true)
                .conflicts_with("quiet"),
            Arg::with_name("quiet")
                .long("quiet")
                .short("q")
                .global(true)
                .takes_value(false)
                .conflicts_with("verbose"),
        ])
        .args(&[
            Arg::with_name("IP:PORT")
                .long("addr")
                .short("a")
                .global(true)
                .takes_value(true)
                .default_value("127.0.0.1:4000"),
            Arg::with_name("ENGINE-NAME")
                .long("engine")
                .short("e")
                .global(true)
                .takes_value(true)
                .possible_values(&["kvs", "sled"]),
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

    Builder::from_env(Env::default().default_filter_or(log_level.to_string())).init();

    info!("kvs-server.{} start", clap::crate_version!(),);

    let working_dir = std::env::current_dir()?.into_boxed_path();
    let engine_name = matches.value_of("ENGINE-NAME").unwrap();

    match engine_name {
        "kvs" => {
            for entry in fs::read_dir(&working_dir)? {
                if let Some(f_name) = entry?.path().file_name() {
                    if f_name == "conf" || f_name == "db" {
                        return Err(Error::EngineMismatch {
                            exist: "sled".to_owned(),
                            got: engine_name.to_owned(),
                        });
                    }
                }
            }
            Server::on(
                matches.value_of("IP:PORT").unwrap().to_owned(),
                KvStore::open(&working_dir)?,
            )?
            .run()
        }
        "sled" => {
            for entry in fs::read_dir(&working_dir)? {
                if let Some(ext) = entry?.path().extension() {
                    if ext == "wal" {
                        return Err(Error::EngineMismatch {
                            exist: "kvs".to_owned(),
                            got: engine_name.to_owned(),
                        });
                    }
                }
            }
            Server::on(
                matches.value_of("IP:PORT").unwrap().to_owned(),
                SledKvsEngine::open(&working_dir)?,
            )?
            .run()
        }
        _ => unreachable!(),
    }
}
