use std::fs;

#[macro_use]
extern crate log;

extern crate env_logger;
use env_logger::{Builder, Env};

mod rpc;
use rpc::{Config, PeerServer};

fn main() {
    Builder::from_env(Env::default().default_filter_or("trace"))
        .is_test(true)
        .init();
    info!("start");

    let config = Config {
        ip: "127.0.0.1".to_owned(),
        port: 10081,
        logs: "tmp/tests/all/1/logs".to_owned(),
        peers: vec![
            ("127.0.0.1".to_owned(), 10082),
            ("127.0.0.1".to_owned(), 10083),
        ],
    };
    fs::create_dir_all(&config.logs).unwrap();
    let mut server = PeerServer::new(config.clone());

    server.run();
}
