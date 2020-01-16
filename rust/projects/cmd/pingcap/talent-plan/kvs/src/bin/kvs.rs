extern crate clap;

use clap::{App, Arg, SubCommand};

use kvs::{self, KvStore, Result};

fn main() -> Result<()> {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .subcommands(vec![
            SubCommand::with_name("set").args(&[
                Arg::with_name("KEY").required(true),
                Arg::with_name("VALUE").required(true),
            ]),
            SubCommand::with_name("get").args(&[Arg::with_name("KEY").required(true)]),
            SubCommand::with_name("rm").args(&[Arg::with_name("KEY").required(true)]),
        ])
        .get_matches();
    let mut kv_store = KvStore::open("tmp")?;
    match matches.subcommand() {
        ("set", Some(m)) => kv_store.set(
            m.value_of("KEY").unwrap().to_owned(),
            m.value_of("VALUE").unwrap().to_owned(),
        ),
        ("get", Some(m)) => kv_store
            .get(m.value_of("KEY").unwrap().to_owned())
            .map_err(|err| {
                eprintln!("{}", err);
                err
            })
            .map(|opt| match opt {
                Some(val) => println!("{}", val),
                None => unreachable!(),
            }),
        ("rm", Some(m)) => kv_store.remove(m.value_of("KEY").unwrap().to_owned()),
        _ => unreachable!(),
    }
}
