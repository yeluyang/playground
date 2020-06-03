use std::{fs, thread};

use raft::*;

#[test]
fn all() {
    let configs = [
        Config {
            ip: "127.0.0.1".to_owned(),
            port: 10081,
            logs: "tmp/tests/all/1/logs".to_owned(),
            peers: vec![("127.0.0.1".to_owned(), 10087)],
        },
        Config {
            ip: "127.0.0.1".to_owned(),
            port: 10082,
            logs: "tmp/tests/all/2/logs".to_owned(),
            peers: vec![("127.0.0.1".to_owned(), 10087)],
        },
        Config {
            ip: "127.0.0.1".to_owned(),
            port: 10083,
            logs: "tmp/tests/all/3/logs".to_owned(),
            peers: vec![("127.0.0.1".to_owned(), 10087)],
        },
    ];
    let mut servers = Vec::new();

    for config in &configs {
        fs::create_dir_all(&config.logs).unwrap();
        servers.push(PeerServer::new(config.clone()));
    }

    for mut server in servers {
        thread::spawn(move || server.run());
    }
}
