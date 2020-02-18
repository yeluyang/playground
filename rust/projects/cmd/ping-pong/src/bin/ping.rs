use std::{
    io::{Read, Write},
    net::TcpStream,
};

extern crate clap;
use clap::{App, AppSettings, Arg};

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
        .args(&[Arg::with_name("value").required(true).multiple(true)])
        .get_matches();

    let mut values: Vec<Protocol> = Vec::new();
    for v in matches.values_of("value").unwrap().collect::<Vec<&str>>() {
        values.push(Protocol::SimpleString(v.to_owned()));
    }
    let values = Protocol::Arrays(values);

    let mut stream = TcpStream::connect("127.0.0.1:10086").unwrap();
    stream.write_all(values.to_bytes().as_slice()).unwrap();
    stream.flush().unwrap();

    let mut buf = String::new();
    stream.read_to_string(&mut buf).unwrap();
    println!("{}", buf);
}
