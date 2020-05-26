use std::thread;

use grpc_examples::NodeServer;

#[test]
fn test_all() {
    let mut n1 = NodeServer::new(
        "1".to_owned(),
        vec![("127.0.0.1".to_owned(), 10087)],
        "127.0.0.1".to_owned(),
        10086,
    );

    let mut n2 = NodeServer::new(
        "2".to_owned(),
        vec![("127.0.0.1".to_owned(), 10086)],
        "127.0.0.1".to_owned(),
        10087,
    );

    println!("running n2");
    thread::spawn(move || n2.run());

    println!("running n1");
    n1.run()
}
