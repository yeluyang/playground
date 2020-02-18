use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
};

extern crate env_logger;
use env_logger::Env;

#[macro_use]
extern crate log;
use log::LevelFilter;

use ping_pong::Protocol;

fn main() {
    env_logger::init_from_env(Env::default().default_filter_or(LevelFilter::Info.to_string()));

    info!("server start");

    let listener = TcpListener::bind("127.0.0.1:10086").unwrap();
    info!("listening");

    for stream in listener.incoming() {
        debug!("new request income");

        handle(stream.unwrap());
    }

    info!("server exit");
}

fn handle(stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = BufWriter::new(stream);

    let mut buf = String::new();
    let len = reader.read_line(&mut buf).unwrap();
    debug!("read from request");
    trace!("request={{ len={}, val={} }}", len, buf);

    let t = Protocol::from(buf.as_str());
    writer.write_all(t.to_bytes().as_slice()).unwrap();
    writer.flush().unwrap();
    debug!("send response");
}
