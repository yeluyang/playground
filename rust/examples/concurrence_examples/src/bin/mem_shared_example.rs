use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

fn main() {
    // TODO add example of RwLock

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
        let mut iter = counts.iter();
        if let Some(last) = iter.next() {
            for item in iter {
                assert!(item > last);
            }
            assert_eq!(counts.len(), counter.load(Ordering::SeqCst));
            println!("count={:?}", counts);
        }
    }
}
