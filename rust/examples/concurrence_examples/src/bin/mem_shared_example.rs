#![feature(is_sorted)]

use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

fn main() {
    let counter = Arc::new(AtomicUsize::new(0usize));
    let counter_a = counter.clone();
    let counter_b = counter.clone();

    let counts = Arc::new(Mutex::new(Vec::new() as Vec<usize>));
    let shared_a = counts.clone();
    let shared_b = counts.clone();

    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(500));
        let guard = shared_a.lock();

        let mut data = guard.unwrap();
        data.push(counter_a.fetch_add(1, Ordering::SeqCst));
    });

    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(500));
        let guard = shared_b.lock();

        let mut data = guard.unwrap();
        data.push(counter_b.fetch_add(1, Ordering::SeqCst));
    });

    loop {
        thread::sleep(Duration::from_secs(1));
        let counts = counts.lock().unwrap();
        assert!(counts.is_sorted());
        assert_eq!(counts.len(), counter.load(Ordering::SeqCst));
        println!("count={{sorted={}, val={:?}}}", counts.is_sorted(), counts);
    }
}
