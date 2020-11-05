use std::path::PathBuf;

extern crate env_logger;
use env_logger::{Builder, Env};

static INIT: std::sync::Once = std::sync::Once::new();

pub fn init() {
    INIT.call_once(|| {
        Builder::from_env(Env::default().default_filter_or("trace"))
            .is_test(true)
            .init();
    })
}

pub fn case_dir(mod_path: &str, case_name: &str) -> PathBuf {
    let mut path = PathBuf::from("tmp");
    for com in mod_path[mod_path.find("tests::").unwrap()..].split("::") {
        path = path.join(com);
    }
    path.join(case_name)
}
