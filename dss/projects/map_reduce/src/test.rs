extern crate env_logger;
use env_logger::{Builder, Env};

static INIT: std::sync::Once = std::sync::Once::new();
pub(crate) fn init() {
    INIT.call_once(|| {
        Builder::from_env(Env::default().default_filter_or("trace"))
            .is_test(true)
            .init();
    })
}
