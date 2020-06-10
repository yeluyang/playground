use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

extern crate serde;

use serde::{Deserialize, Serialize};

// TODO: ron cannot support i128 and u128, should convert it manually
// TODO: bson cannot support unsigned, should convert it manually
// TODO: toml cannot support enum and tuple

// normal derive
#[derive(Debug, PartialEq, Default)]
// serde derive
#[derive(Deserialize, Serialize)]
pub struct Config {
    id: isize,
    var: String,
    tuple: ((), i8, i16, i32, i64),
    array: Vec<char>,
    map: HashMap<isize, bool>,
    array_tuples: Vec<((), i64, bool, char, String)>,
    // refer: Box<String>, // TODO
    enum_var: EnumVariant,
    tuple_struct: TupleStruct,
    array_struct: Vec<Nested>,
    nested: Nested,
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
struct TupleStruct(
    (),
    bool,
    char,
    i8,
    i16,
    i32,
    i64,
    String,
    HashMap<isize, bool>,
    Vec<Nested>,
    EnumVariant,
);

// normal derive
#[derive(Debug, PartialEq)]
// serde derive
#[derive(Deserialize, Serialize)]
enum EnumVariant {
    None,
    Var(char),
    Tuple((), bool, i8, i16, i32, i64),
    Struct { i: isize, s: String },
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
    id: isize,
    var: String,
    tuple: ((), i8, i16, i32, i64),
    array: Vec<isize>,
    map: HashMap<char, bool>,
    // refer: Box<String>, TODO
    enum_var: EnumVariant,
    tuple_struct: TupleStruct,
    array_struct: Vec<Nested>,
}
