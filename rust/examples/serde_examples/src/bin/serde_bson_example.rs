use std::{fs::File, io::Write};

extern crate bson;

use serde_examples::Config;

fn main() {
    let cfg = Config::default();

    // ser to bson, document
    let c_bson = bson::to_bson(&cfg).unwrap();
    println!("bson = {}", c_bson);
    let c_bson_doc = c_bson.as_document().unwrap().clone();
    println!("bson document = {}", c_bson_doc);
    let c_bson_str = c_bson_doc.to_string();
    println!("bson string = {}", c_bson_str);

    // de from bson object
    let c_from_bson: Config = bson::from_bson(c_bson).unwrap();
    println!("from bson = {}", c_from_bson);
    assert_eq!(cfg, c_from_bson);

    // de from bson document
    // have to convert bson::Document to bson::Bson
    let c_from_bson_doc: Config =
        bson::from_bson(bson::Bson::Document(c_bson_doc.clone())).unwrap();
    println!("from bson document = {}", c_from_bson);
    assert_eq!(cfg, c_from_bson_doc);

    // ser to bson bytes
    let mut c_bson_bytes = Vec::new();
    bson::encode_document(&mut c_bson_bytes, &c_bson_doc).unwrap();

    // write bson bytes to file.bson
    File::create("tmp/tmp.bson")
        .unwrap()
        .write_all(c_bson_bytes.as_slice())
        .unwrap();

    // de from file.bson
    // 1. get bson document from file.bson
    let c_bson_doc_from_bytes_file =
        bson::decode_document(&mut File::open("tmp/tmp.bson").unwrap()).unwrap();
    assert_eq!(c_bson_doc, c_bson_doc_from_bytes_file);
    // 2. de from bson document
    let c_from_bson_doc_from_bytes_file: Config =
        bson::from_bson(bson::Bson::Document(c_bson_doc_from_bytes_file)).unwrap();
    println!("from bson bytes file = {}", c_from_bson_doc_from_bytes_file);
    assert_eq!(cfg, c_from_bson_doc_from_bytes_file);
}
