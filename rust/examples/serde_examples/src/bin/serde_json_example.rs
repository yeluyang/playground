extern crate serde_json;

use std::{fs::File, io::Write};

use serde_examples::Config;

fn main() {
    let cfg = Config::default();

    let c_json_str = serde_json::to_string_pretty(&cfg).unwrap();
    println!("json string = {}", c_json_str);
    let c_from_json_str: Config = serde_json::from_str(&c_json_str).unwrap();
    println!("from json string = {}", c_from_json_str.to_string());
    assert_eq!(cfg, c_from_json_str);

    File::create("tmp/tmp.json")
        .unwrap()
        .write_all(c_json_str.as_bytes())
        .unwrap();
    let c_from_json_file: Config =
        serde_json::from_reader(File::open("tmp/tmp.json").unwrap()).unwrap();
    println!("from json file = {}", c_from_json_file.to_string());
    assert_eq!(cfg, c_from_json_file);

    let c_json_bytes = Vec::from(c_json_str.as_bytes());
    let c_from_json_bytes: Config = serde_json::from_slice(c_json_bytes.as_slice()).unwrap();
    println!("from json bytes = {}", c_from_json_bytes.to_string());
    assert_eq!(cfg, c_from_json_bytes);
}
