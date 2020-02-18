use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    time::Duration,
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

fn handle(mut stream: TcpStream) {
    stream
        .set_read_timeout(Some(Duration::from_secs(4)))
        .unwrap();
    let mut buf = Vec::with_capacity(10240);
    let len = stream.read(buf.as_mut_slice()).unwrap();
    debug!("read from request");
    trace!("request={{ len={}, val={:?} }}", len, &buf[..len]);

    let t = Protocol::from(buf);
    stream.write_all(t.to_bytes().as_slice()).unwrap();
    stream.flush().unwrap();
    debug!("send response");
}
