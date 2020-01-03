use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

extern crate serde;

use serde::{Deserialize, Serialize};

// normal derive
#[derive(Debug, PartialEq, Default)]
// serde derive
#[derive(Deserialize, Serialize)]
pub struct Config {
    var: String,
    tuple: (f32, f64),
    map: HashMap<u8, char>,
    tuple_struct: TupleStruct,
    nested: Nested,
    array: Vec<i32>,
    enum_var: EnumVariant,
    refer: Box<String>,
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// normal derive
#[derive(Debug, PartialEq, Default)]
// serde derive
#[derive(Deserialize, Serialize)]
struct TupleStruct((), bool);

// normal derive
#[derive(Debug, PartialEq)]
// serde derive
#[derive(Deserialize, Serialize)]
enum EnumVariant {
    None,
    Var(u8),
    Tuple(u8, String),
    Struct { v: u8, s: String },
}

impl Default for EnumVariant {
    fn default() -> Self {
        EnumVariant::None
    }
}

// normal derive
#[derive(Debug, PartialEq, Default)]
// serde derive
#[derive(Deserialize, Serialize)]
struct Nested {
    a: String,
    b: char,
}
