use std::{env, fs};

fn main() {
    for entry in fs::read_dir(env::current_dir().unwrap().as_path()).unwrap() {
        let entry = entry.unwrap();
        println!("{:?}", entry.path());
    }
}
