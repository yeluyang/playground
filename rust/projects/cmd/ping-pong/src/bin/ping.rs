use std::{
    io::{Read, Write},
    net::TcpStream,
};

extern crate clap;
use clap::{App, AppSettings, Arg};

extern crate env_logger;
use env_logger::Env;

#[macro_use]
extern crate log;
use log::LevelFilter;

use ping_pong::Protocol;

fn main() {
    let matches = App::new(clap::crate_name!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .args(&[Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .multiple(true)])
        .args(&[
            Arg::with_name("quiet").short("q").conflicts_with("verbose"),
            Arg::with_name("value").required(true),
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

    info!("client start");

    let arg = matches.value_of("value").unwrap();
    trace!("arguments={}", arg);

    let value = if arg.parse::<i128>().is_ok() {
        Protocol::Integers(arg.parse::<i128>().unwrap())
    } else {
        Protocol::SimpleString(arg.to_owned())
    };

    let mut stream = TcpStream::connect("127.0.0.1:10086").unwrap();
    info!("listening");

    stream.write_all(value.to_bytes().as_slice()).unwrap();
    stream.flush().unwrap();
    info!("write request into tcp socket");

    let mut buf = String::new();
    stream.read_to_string(&mut buf).unwrap();
    info!("read response from tcp socket");

    println!("{}", buf);

    info!("client exit");
}
