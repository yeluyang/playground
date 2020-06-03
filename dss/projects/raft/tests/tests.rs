use std::fs;

use raft::*;

#[test]
fn all() {
    let config = Config {
        ip: "127.0.0.1".to_owned(),
        port: 10086,
        logs: "tmp/tests/all/logs".to_owned(),
        peers: vec![("127.0.0.1".to_owned(), 10087)],
    };

    fs::create_dir_all(&config.logs).unwrap();

    let mut server = PeerServer::new(config.clone());
    server.run()
}
