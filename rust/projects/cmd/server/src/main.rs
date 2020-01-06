use std::{env, net};

extern crate env_logger;
extern crate log;

use server::*;

fn main() {
    env::set_var("RUST_LOG", "TRACE");
    env_logger::init();
    let listener = net::TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);
    log::info!("serving");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle(stream);
        });
    }
}
