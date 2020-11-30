//! try to challenge safety of rust

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

fn main() {
    let counter = Arc::new(Mutex::new(0usize));
    let counter_a = counter.clone();
    let counter_b = counter.clone();

    let counts = Arc::new(Mutex::new(Vec::new() as Vec<usize>));
    let shared_a = counts.clone();
    let shared_b = counts.clone();

    let (sender, receiver) = mpsc::channel::<String>();
    let sender_a = sender.clone();
    let sender_b = sender.clone();

    // following code will cause Dead-Lock
    // can package all related data in one mutex to avoid it

    thread::spawn(move || loop {
        let mut counts = shared_a.lock().unwrap();
        sender_a
            .send("A hold counts, wait for counter".to_owned())
            .unwrap();
        let mut counter = counter_a.lock().unwrap();
        sender_a
            .send("A hold both counts and counter".to_owned())
            .unwrap();

        *counter += 1;
        counts.push(*counter);
    });

    thread::spawn(move || loop {
        let mut counter = counter_b.lock().unwrap();
        sender_b
            .send("B hold counter, wait for counts".to_owned())
            .unwrap();
        let mut counts = shared_b.lock().unwrap();
        sender_b
            .send("B hold both counts and counter".to_owned())
            .unwrap();

        *counter += 1;
        counts.push(*counter);
    });

    loop {
        println!("{}", receiver.recv().unwrap());
    }
}
