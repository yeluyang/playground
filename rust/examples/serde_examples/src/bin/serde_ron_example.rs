use std::{fs::File, io::Write};

extern crate ron;

use ron::{de, ser};

use serde_examples::Config;

fn main() {
    let cfg = Config::default();

    // ser to ron string
    let c_ron_str = ser::to_string(&cfg).unwrap();
    println!("ron string = {}", c_ron_str);

    // deser from ron string
    let c_from_ron_str: Config = de::from_str(&c_ron_str).unwrap();
    println!("from ron string = {}", c_from_ron_str);
    assert_eq!(cfg, c_from_ron_str);

    // write ron string to file.ron
    File::create("tmp/tmp.ron")
        .unwrap()
        .write_all(c_ron_str.as_bytes())
        .unwrap();

    // deser from file.ron
    let c_from_ron_file: Config = de::from_reader(File::open("tmp/tmp.ron").unwrap()).unwrap();
    println!("from ron file = {}", c_from_ron_file.to_string());
    assert_eq!(cfg, c_from_ron_file);

    // convert ron string to bytes
    let c_ron_bytes = Vec::from(c_ron_str.as_bytes());

    // deser from bytes
    let c_from_ron_bytes: Config = de::from_bytes(c_ron_bytes.as_slice()).unwrap();
    println!("from ron bytes = {}", c_from_ron_bytes);
    assert_eq!(cfg, c_from_ron_bytes);
}
