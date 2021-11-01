extern crate log;
use log::LevelFilter;

extern crate env_logger;
use env_logger::Env;

fn main() {
    env_logger::init_from_env(Env::default().default_filter_or(LevelFilter::Trace.to_string()));
    log::info!("starting");
}
