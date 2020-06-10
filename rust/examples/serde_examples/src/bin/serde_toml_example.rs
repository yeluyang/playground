extern crate toml;

use std::{
    fs::File,
    io::{Read, Write},
};

use serde_examples::Config;

fn main() {
    let cfg = Config::default();

    // ser to toml string
    let c_toml_str = toml::to_string(&cfg).unwrap();
    println!(
        "toml string={{ len={}, val={} }}",
        c_toml_str.as_bytes().len(),
        c_toml_str
    );

    // deser from toml string
    let c_from_toml_str: Config = toml::from_str(&c_toml_str).unwrap();
    println!("from toml string = {}", c_from_toml_str.to_string());
    assert_eq!(cfg, c_from_toml_str);

    // convert toml string to bytes
    let c_toml_bytes = Vec::from(c_toml_str.as_bytes());

    // deser from bytes
    let c_from_toml_bytes: Config = toml::from_slice(c_toml_bytes.as_slice()).unwrap();
    println!("from toml bytes = {}", c_from_toml_bytes.to_string());
    assert_eq!(cfg, c_from_toml_bytes);

    // write toml string to file.toml
    File::create("tmp/tmp.toml")
        .unwrap()
        .write_all(c_toml_str.as_bytes())
        .unwrap();

    let mut c_toml_str_from_file = String::new();
    File::open("tmp/tmp.toml")
        .unwrap()
        .read_to_string(&mut c_toml_str_from_file)
        .unwrap();

    // deser from file.toml
    let c_from_toml_file: Config = toml::from_str(&c_toml_str_from_file).unwrap();
    println!("from toml file = {}", c_from_toml_file.to_string());
    assert_eq!(cfg, c_from_toml_file);
}
