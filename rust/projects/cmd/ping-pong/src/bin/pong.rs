use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use ping_pong::Protocol;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:10086").unwrap();
    for stream in listener.incoming() {
        handle(stream.unwrap());
    }
}

fn handle(mut stream: TcpStream) {
    let mut buf = String::new();
    stream.read_to_string(&mut buf).unwrap();
    let t = Protocol::from(buf.as_str());
    stream.write_all(t.to_bytes().as_slice()).unwrap();
    stream.flush().unwrap();
}
