use std::{fs, thread};

use raft::*;

extern crate env_logger;
use env_logger::{Builder, Env};

static INIT: std::sync::Once = std::sync::Once::new();
fn setup_logger() {
    INIT.call_once(|| {
        Builder::from_env(Env::default().default_filter_or("trace"))
            .is_test(true)
            .init();
    })
}

#[test]
fn all() {
    setup_logger();

    let configs = [
        Config {
            ip: "127.0.0.1".to_owned(),
            port: 10081,
            logs: "tmp/tests/all/1/logs".to_owned(),
            peers: vec![
                ("127.0.0.1".to_owned(), 10082),
                ("127.0.0.1".to_owned(), 10083),
            ],
        },
        Config {
            ip: "127.0.0.1".to_owned(),
            port: 10082,
            logs: "tmp/tests/all/2/logs".to_owned(),
            peers: vec![
                ("127.0.0.1".to_owned(), 10081),
                ("127.0.0.1".to_owned(), 10083),
            ],
        },
        Config {
            ip: "127.0.0.1".to_owned(),
            port: 10083,
            logs: "tmp/tests/all/3/logs".to_owned(),
            peers: vec![
                ("127.0.0.1".to_owned(), 10081),
                ("127.0.0.1".to_owned(), 10082),
            ],
        },
    ];
    let mut servers = Vec::new();

    for config in &configs {
        fs::create_dir_all(&config.logs).unwrap();
        servers.push(PeerServer::new(config.clone()));
    }

    let mut handlers = Vec::new();
    for mut server in servers {
        handlers.push(thread::spawn(move || server.run()));
    }

    for h in handlers {
        h.join().unwrap();
    }
}
