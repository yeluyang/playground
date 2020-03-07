use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (sender_a, reciver) = mpsc::channel();
    let sender_b = sender_a.clone();

    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(1000));
        sender_a.send("a".to_owned());
    });

    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(1000));
        sender_b.send("b".to_owned());
    });

    // TODO try multi consumer
    let mut count = 0usize;
    let mut data: Vec<(usize, String)> = Vec::new();
    loop {
        let s = reciver.recv().unwrap();
        count += 1;
        data.push((count, s));
        println!("data={:?}", data);
    }
}
