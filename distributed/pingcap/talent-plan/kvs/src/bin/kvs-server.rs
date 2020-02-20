extern crate clap;
use clap::{App, AppSettings, Arg};

fn main() {
    let matches = App::new("kvs-server")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .args(&[
            Arg::with_name("IP:PORT")
                .long("addr")
                .short("a")
                .takes_value(true)
                .default_value("127.0.0.1:4000"),
            Arg::with_name("ENGINE-NAME")
                .long("engine")
                .short("e")
                .takes_value(true),
        ])
        .get_matches();
}
